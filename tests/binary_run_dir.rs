use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

fn temporary_root() -> PathBuf {
  let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
  std::env::temp_dir().join(format!(
    "fog-of-intent-binary-run-dir-{}-{sequence}",
    std::process::id()
  ))
}

fn binary_path() -> PathBuf {
  if let Some(path) = std::env::var_os("CARGO_BIN_EXE_fog_of_intent") {
    return PathBuf::from(path);
  }
  let mut path = std::env::current_exe().expect("integration test path");
  path.pop();
  path.pop();
  path.push("fog-of-intent");
  #[cfg(windows)]
  path.set_extension("exe");
  path
}

fn run_binary_with_scenario(
  binary: &Path,
  root: &Path,
  scenario: Option<&str>,
  input: &str,
) -> Output {
  let mut command = Command::new(binary);
  if let Some(scenario) = scenario {
    command.args(["--scenario", scenario]);
  }
  command
    .arg("--run-dir")
    .arg(root)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  let mut child = command.spawn().expect("spawn fixture binary");
  child
    .stdin
    .as_mut()
    .expect("fixture stdin")
    .write_all(input.as_bytes())
    .expect("write fixture input");
  child.wait_with_output().expect("wait for fixture binary")
}

fn run_default_binary(binary: &Path, working_directory: &Path, input: &str) -> Output {
  let mut child = Command::new(binary)
    .current_dir(working_directory)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn default fixture binary");
  child
    .stdin
    .as_mut()
    .expect("default fixture stdin")
    .write_all(input.as_bytes())
    .expect("write default fixture input");
  child
    .wait_with_output()
    .expect("wait for default fixture binary")
}

fn run_scenario_binary(binary: &Path, scenario: &str, input: &str) -> Output {
  let mut child = Command::new(binary)
    .arg("--scenario")
    .arg(scenario)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn scenario fixture binary");
  child
    .stdin
    .as_mut()
    .expect("scenario fixture stdin")
    .write_all(input.as_bytes())
    .expect("write scenario fixture input");
  child
    .wait_with_output()
    .expect("wait for scenario fixture binary")
}

#[test]
fn run_directory_wires_save_and_load_across_processes() {
  let binary = binary_path();
  let root = temporary_root();

  let first = run_binary_with_scenario(
    &binary,
    &root,
    Some("m3-two-window-fixture-v1"),
    "plan contest\ncommit\nadvance\nsave run\nquit\n",
  );
  assert!(first.status.success(), "first stderr: {:?}", first.stderr);
  let first_stdout = String::from_utf8(first.stdout).expect("first UTF-8 output");
  assert!(first_stdout.contains("save: status=saved run_id=run records=1"));
  assert!(root.join("run.foi-artifact").is_file());

  let second = run_binary_with_scenario(
    &binary,
    &root,
    Some("m3-two-window-fixture-v1"),
    "load run\ninspect history\nquit\n",
  );
  assert!(
    second.status.success(),
    "second stderr: {:?}",
    second.stderr
  );
  let second_stdout = String::from_utf8(second.stdout).expect("second UTF-8 output");
  assert!(second_stdout.contains("load: status=loaded run_id=run records=1"));
  assert!(second_stdout.contains("history: records=1 status=open"));

  let _ = fs::remove_dir_all(root);
}

#[test]
fn binary_argument_failures_are_non_success_and_path_free() {
  let binary = binary_path();
  let secret_path = OsString::from("private-run-directory");
  let output = Command::new(binary)
    .args(["--run-dir"])
    .arg(&secret_path)
    .arg("--run-dir")
    .arg("other")
    .output()
    .expect("run argument parser");

  assert!(!output.status.success());
  let stderr = String::from_utf8(output.stderr).expect("argument stderr UTF-8");
  assert!(stderr.contains("--run-dir may be provided only once"));
  assert!(!stderr.contains("private-run-directory"));

  for token in ["--help", "--run-dir", "--unknown"] {
    let output = Command::new(binary_path())
      .arg("--run-dir")
      .arg(token)
      .output()
      .expect("run option-shaped path parser");
    assert!(!output.status.success());
  }

  for args in [
    vec!["--scenario"],
    vec!["--scenario", "unknown-fixture"],
    vec!["--scenario", "--run-dir"],
  ] {
    let output = Command::new(binary_path())
      .args(args)
      .output()
      .expect("run scenario parser");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("scenario stderr UTF-8");
    assert!(!stderr.contains("unknown-fixture"));
  }
}

#[test]
fn binary_help_is_successful_and_bounded() {
  let output = Command::new(binary_path())
    .arg("--help")
    .output()
    .expect("run executable help");

  assert!(output.status.success());
  assert_eq!(
    String::from_utf8(output.stdout).expect("help UTF-8 output"),
    "usage: fog-of-intent [--scenario <id>] [--run-dir <path>]\n\noptions:\n  --scenario <id>   select m3-two-window-fixture-v1\n  --run-dir <path>  store bounded run artifacts in this directory\n  --help            show this help\n  --version, -V     show package version\n"
  );
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_version_aliases_are_successful_and_host_free() {
  let expected = format!("fog-of-intent {}\n", env!("CARGO_PKG_VERSION"));
  for argument in ["--version", "-V"] {
    let output = Command::new(binary_path())
      .arg(argument)
      .output()
      .expect("run executable version");
    assert!(output.status.success());
    assert_eq!(
      String::from_utf8(output.stdout).expect("version UTF-8"),
      expected
    );
    assert!(output.stderr.is_empty());
  }

  let combined = Command::new(binary_path())
    .args(["--version", "--run-dir", "ignored"])
    .output()
    .expect("run combined version arguments");
  assert!(!combined.status.success());
}

#[test]
fn binary_completes_the_documented_two_window_transcript() {
  let output = run_scenario_binary(
    &binary_path(),
    "m3-two-window-fixture-v1",
    "observe\nmessage ping ally\ncontingency retreat if threat\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\nreplay\ndebrief\nquit\n",
  );

  assert!(
    output.status.success(),
    "transcript stderr: {:?}",
    output.stderr
  );
  assert!(output.stderr.is_empty());
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8 output");
  assert!(stdout.contains("observation: schema="));
  assert!(stdout.contains("draft: status=staged field=message"));
  assert!(stdout.contains("draft: status=staged field=contingency"));
  assert!(stdout.contains("advanced: window=first"));
  assert!(stdout.contains("advanced: window=second"));
  assert!(stdout.contains("replay: status=verified run_id=current records=2"));
  assert!(stdout.contains("debrief: schema="));
  assert!(stdout.ends_with("quit: status=closed\n"));
}

#[test]
fn binary_accepts_the_versioned_fixture_scenario() {
  let binary = binary_path();
  let output = Command::new(binary)
    .args(["--scenario", "m3-two-window-fixture-v1"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn explicitly selected fixture binary")
    .wait_with_output()
    .expect("wait for explicitly selected fixture binary");

  assert!(output.status.success());
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_without_run_directory_remains_in_memory() {
  let binary = binary_path();
  let root = temporary_root();
  fs::create_dir_all(&root).expect("isolated working directory");

  let output = run_default_binary(&binary, &root, "save run\nquit\n");
  assert!(
    output.status.success(),
    "default stderr: {:?}",
    output.stderr
  );
  let stdout = String::from_utf8(output.stdout).expect("default UTF-8 output");
  assert!(stdout.contains("save: status=saved run_id=run records=0"));
  assert_eq!(fs::read_dir(&root).expect("isolated directory").count(), 0);

  let _ = fs::remove_dir_all(root);
}
