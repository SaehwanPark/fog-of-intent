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

fn run_binary(binary: &Path, root: &Path, input: &str) -> Output {
  let mut child = Command::new(binary)
    .arg("--run-dir")
    .arg(root)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn fixture binary");
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

#[test]
fn run_directory_wires_save_and_load_across_processes() {
  let binary = binary_path();
  let root = temporary_root();

  let first = run_binary(
    &binary,
    &root,
    "plan contest\ncommit\nadvance\nsave run\nquit\n",
  );
  assert!(first.status.success(), "first stderr: {:?}", first.stderr);
  let first_stdout = String::from_utf8(first.stdout).expect("first UTF-8 output");
  assert!(first_stdout.contains("save: status=saved run_id=run records=1"));
  assert!(root.join("run.foi-artifact").is_file());

  let second = run_binary(&binary, &root, "load run\ninspect history\nquit\n");
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
    "usage: fog-of-intent [--run-dir <path>]\n\noptions:\n  --run-dir <path>  store bounded run artifacts in this directory\n  --help            show this help\n"
  );
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
