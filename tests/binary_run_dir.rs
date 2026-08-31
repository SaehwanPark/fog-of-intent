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

fn mcp_binary_path() -> PathBuf {
  if let Some(path) = std::env::var_os("CARGO_BIN_EXE_fog-of-intent-mcp") {
    return PathBuf::from(path);
  }
  if let Some(path) = std::env::var_os("CARGO_BIN_EXE_fog_of_intent_mcp") {
    return PathBuf::from(path);
  }
  let mut path = std::env::current_exe().expect("integration test path");
  path.pop();
  path.pop();
  path.push("fog-of-intent-mcp");
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
    "usage: fog-of-intent [--scenario <id>] [--select] [--mcp] [--run-dir <path>] [--color auto|always|never] [--width <cols>]\n\noptions:\n  --scenario <id>    select m3-two-window-fixture-v1, m2-strategy-happy-path-v1, m2-strategy-risk-taking-v1, m2-strategy-conservative-v1, m6-behavioral-experiments-v1, m7-calibration-proof-v1, m8-team-scenarios-v1, m9-interactive-match-v1, m9-complete-match-replay-v1, m10-human-study-synthesis-v1, m10-empirical-cohort-study-v1, m11-gui-presentation-v1, m11-gui-browser-flow-v1, m12-alpha-release-checks-v1, m12-reproducibility-bundle-v1, or m12-alpha-archive-v1\n  --select, -s       interactively choose a scenario from the catalog menu\n  --list-scenarios   list all available scenarios and descriptions\n  --mcp              start Model Context Protocol (MCP) JSON-RPC stdio server\n  --run-dir <path>   store bounded run artifacts in this directory (interactive scenarios only)\n  --color <mode>     auto, always, or never (default auto)\n  --width <cols>     override terminal column width for line wrapping (default 80)\n  --help             show this help\n  --version, -V      show package version\n"
  );
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_list_scenarios_outputs_catalog_table() {
  for flag in ["--list-scenarios", "-l"] {
    let output = Command::new(binary_path())
      .arg(flag)
      .output()
      .expect("run executable list scenarios");

    assert!(output.status.success(), "stderr: {:?}", output.stderr);
    let stdout = String::from_utf8(output.stdout).expect("catalog UTF-8");
    assert!(stdout.starts_with("Fog of Intent — Scenario Catalog\n\n"));
    assert!(stdout.contains("m3-two-window-fixture-v1"));
    assert!(stdout.contains("m2-strategy-happy-path-v1"));
    assert!(stdout.contains("m2-strategy-risk-taking-v1"));
    assert!(stdout.contains("m2-strategy-conservative-v1"));
    assert!(stdout.contains("m6-behavioral-experiments-v1"));
    assert!(stdout.contains("m8-team-scenarios-v1"));
    assert!(stdout.contains("m9-interactive-match-v1"));
    assert!(stdout.contains("m9-complete-match-replay-v1"));
    assert!(stdout.contains("m10-human-study-synthesis-v1"));
    assert!(stdout.contains("m11-gui-presentation-v1"));
    assert!(stdout.contains("m12-alpha-release-checks-v1"));
    assert!(stdout.contains("m12-reproducibility-bundle-v1"));
    assert!(stdout.contains("m12-alpha-archive-v1"));
    assert!(stdout.contains("interactive-lane"));
    assert!(stdout.contains("behavioral-battery"));
    assert!(stdout.contains("team-battery"));
    assert!(stdout.contains("replay-transcript"));
    assert!(stdout.contains("study-synthesis"));
    assert!(stdout.contains("html-presentation"));
    assert!(stdout.contains("release-checks"));
    assert!(stdout.contains("reproducibility-bundle"));
    assert!(output.stderr.is_empty());
  }
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
  assert!(stdout.contains("commit: status=committed intent=contest"));
  assert!(stdout.contains("commit: status=committed intent=stabilize"));
  assert!(stdout.contains("advanced: window=first"));
  assert!(stdout.contains("advanced: window=second"));
  assert!(stdout.contains("replay: status=verified run_id=current records=2"));
  assert!(stdout.contains("debrief: schema="));
  assert!(stdout.ends_with("quit: status=closed\n"));
  assert!(!stdout.contains("hash"));
  assert!(!stdout.contains("source_"));
  assert!(!stdout.contains("error:"));
}

#[test]
fn binary_completes_happy_path_strategy_playthrough() {
  let binary = binary_path();
  let root = temporary_root();

  let output = run_binary_with_scenario(
    &binary,
    &root,
    Some("m2-strategy-happy-path-v1"),
    "observe\nplan contest\ncommit\nadvance\nplan contest\ncommit\nadvance\nsave happy-run\nreplay\ndebrief\nquit\n",
  );

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  assert!(output.stderr.is_empty());
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8");
  assert!(stdout.contains("observation: schema="));
  assert!(stdout.contains("commit: status=committed intent=contest"));
  assert!(stdout.contains("advanced: window=first outcome=held_space"));
  assert!(stdout.contains("advanced: window=second outcome=held_space"));
  assert!(stdout.contains("save: status=saved run_id=happy-run records=2"));
  assert!(stdout.contains("replay: status=verified run_id=current records=2"));
  assert!(stdout.contains("debrief: schema="));
  assert!(stdout.ends_with("quit: status=closed\n"));
  assert!(root.join("happy-run.foi-artifact").is_file());

  let _ = fs::remove_dir_all(root);
}

#[test]
fn binary_completes_risk_taking_strategy_playthrough() {
  let output = run_scenario_binary(
    &binary_path(),
    "m2-strategy-risk-taking-v1",
    "observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\nreplay\ndebrief\nquit\n",
  );

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  assert!(output.stderr.is_empty());
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8");
  assert!(stdout.contains("observation: schema="));
  assert!(stdout.contains("commit: status=committed intent=contest"));
  assert!(stdout.contains("advanced: window=first outcome=yielded_space"));
  assert!(stdout.contains("commit: status=committed intent=stabilize"));
  assert!(stdout.contains("replay: status=verified run_id=current records=2"));
  assert!(stdout.contains("debrief: schema="));
  assert!(stdout.ends_with("quit: status=closed\n"));
}

#[test]
fn binary_completes_conservative_strategy_playthrough() {
  let output = run_scenario_binary(
    &binary_path(),
    "m2-strategy-conservative-v1",
    "observe\nplan stabilize\ncommit\nadvance\nplan stabilize\ncommit\nadvance\nreplay\ndebrief\nquit\n",
  );

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  assert!(output.stderr.is_empty());
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8");
  assert!(stdout.contains("observation: schema="));
  assert!(stdout.contains("commit: status=committed intent=stabilize"));
  assert!(stdout.contains("advanced: window=first outcome=yielded_space"));
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

#[test]
fn binary_prints_replay_verified_complete_match_transcript() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m9-complete-match-replay-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 transcript");
  let lines: Vec<&str> = stdout.lines().collect();
  assert_eq!(lines.len(), 6, "full transcript: {stdout}");
  assert_eq!(lines[0], "match-replay: begin");
  assert!(lines[1].starts_with(
    "match: scenario=scenario-complete-allied-snowball-v1 winner=allied condition=nexus-demolished"
  ));
  assert!(lines[3].starts_with(
    "match: scenario=scenario-complete-comeback-concession-v1 winner=allied condition=match-conceded"
  ));
  assert_eq!(lines[5], "match-replay: complete");
}

#[test]
fn binary_prints_gui_presentation_document() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m11-gui-presentation-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 HTML");
  assert!(stdout.starts_with("<!DOCTYPE html>"));
  assert!(stdout.contains("<html lang=\"en\">"));
  assert!(stdout.contains("<meta name=\"viewport\""));
  assert!(stdout.contains("<svg"));
  assert!(!stdout.contains("<script"));
}

#[test]
fn binary_prints_alpha_release_checks_report() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m12-alpha-release-checks-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 report");
  assert!(stdout.contains("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
  assert!(stdout.contains("READY FOR PUBLIC ALPHA"));
  assert!(stdout.contains("clean-install"));
  assert!(stdout.contains("reproducibility"));
  assert!(stdout.contains("security-advisory"));
  assert!(stdout.contains("license-compliance"));
  assert!(stdout.contains("compatibility-matrix"));
  assert!(stdout.contains("data-redaction"));
  assert!(stdout.ends_with('\n'));
}

#[test]
fn binary_prints_reproducibility_bundle_report() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m12-reproducibility-bundle-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 report");
  assert!(stdout.contains("# Public Alpha Reproducibility Bundle Audit Report"));
  assert!(stdout.contains("**Eligible for Release:** Yes"));
  assert!(stdout.contains("PKG-BENCHMARK-01"));
  assert!(stdout.contains("PKG-REPLAY-01"));
  assert!(stdout.contains("PKG-EXPERIMENT-01"));
  assert!(stdout.contains("PKG-CALIBRATION-01"));
  assert!(stdout.contains("PKG-TELEMETRY-01"));
  assert!(stdout.ends_with('\n'));
}

fn run_select_binary(binary: &Path, input: &str) -> Output {
  let mut child = Command::new(binary)
    .arg("--select")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn select binary");
  child
    .stdin
    .as_mut()
    .expect("select stdin")
    .write_all(input.as_bytes())
    .expect("write select input");
  child.wait_with_output().expect("wait for select binary")
}

#[test]
fn binary_interactive_select_runs_chosen_strategy_scenario() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "2\nobserve\nquit\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains("[2] HappyPath Strategy Playthrough"));
  assert!(stdout.contains("observation: schema="));
  assert!(stdout.ends_with("quit: status=closed\n"));
}

#[test]
fn binary_interactive_select_runs_match_replay_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "m9\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains("match-replay: begin"));
  assert!(stdout.contains("match-replay: complete"));
}

#[test]
fn binary_interactive_select_runs_team_scenarios_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "team\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(
    stdout.contains("# Fog of Intent — Milestone M8 Team Communication & Shot-Calling Battery")
  );
  assert!(stdout.contains("scenario-high-trust-gank-v1"));
  assert!(stdout.contains("Benchmark Battery Summary"));
}

#[test]
fn binary_interactive_select_cancels_on_quit() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "q\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_interactive_select_retries_on_invalid_and_runs() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "invalid-choice\n1\nquit\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("unknown scenario selection: 'invalid-choice'"));
  assert!(stdout.ends_with("quit: status=closed\n"));
}

#[test]
fn binary_supports_interactive_branch_exploration_across_windows() {
  let output = run_scenario_binary(
    &binary_path(),
    "m3-two-window-fixture-v1",
    "plan contest\ncommit\nadvance\nplan yield\nbranch first\nplan stabilize\ncommit\nadvance\nplan contest\nbranch second\nplan yield\nbranch first\nquit\n",
  );
  assert!(
    output.status.success(),
    "transcript stderr: {:?}",
    output.stderr
  );
  assert!(output.stderr.is_empty());
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8 output");
  assert!(
    stdout
      .contains("branch: status=verified point=first parent_intent=contest branch_intent=yield")
  );
  assert!(stdout.contains(
    "branch: status=verified point=second parent_intent=stabilize branch_intent=contest"
  ));
  assert!(stdout.ends_with("quit: status=closed\n"));
}

#[test]
fn binary_supports_width_flag_and_wraps_lines() {
  let binary = binary_path();
  let mut child = Command::new(binary)
    .args(["--scenario", "m3-two-window-fixture-v1", "--width", "40"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn width fixture binary");

  child
    .stdin
    .as_mut()
    .expect("fixture stdin")
    .write_all(b"observe\nquit\n")
    .expect("write fixture input");

  let output = child.wait_with_output().expect("wait for binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 transcript");
  for line in stdout.lines() {
    assert!(
      line.chars().count() <= 40,
      "line length {} > 40: '{}'",
      line.chars().count(),
      line
    );
  }
}

#[test]
fn binary_runs_accessible_two_window_transcript_and_passes_audit() {
  // Run with explicit --width 80 so that output is wrapped and auditable against standard bounds.
  let binary = binary_path();
  let mut child = Command::new(&binary)
    .args(["--scenario", "m3-two-window-fixture-v1", "--width", "80"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn accessibility fixture binary");
  child
    .stdin
    .as_mut()
    .expect("fixture stdin")
    .write_all(
      b"observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\ndebrief\nquit\n",
    )
    .expect("write fixture input");
  let output = child
    .wait_with_output()
    .expect("wait for accessibility fixture binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("transcript UTF-8");
  let report = fog_of_intent::cli::audit_cli_presentation_text(
    &stdout,
    fog_of_intent::terminal::TerminalDimensions::standard(),
    false,
  );
  assert!(
    report.all_passed,
    "accessibility audit report failed: {:?}",
    report
  );
  assert_eq!(report.compliance_rate_bp, 10_000);
}

#[test]
fn binary_lists_m9_interactive_match_in_scenario_catalog() {
  let binary = binary_path();
  let output = Command::new(&binary)
    .arg("--list-scenarios")
    .output()
    .expect("list scenarios");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8");
  assert!(
    stdout.contains("m9-interactive-match-v1"),
    "catalog missing m9-interactive-match-v1: {stdout}"
  );
  assert!(
    stdout.contains("Interactive 5v5 Tactical Match Playthrough"),
    "catalog missing scenario display name: {stdout}"
  );
}

#[test]
fn binary_runs_interactive_m9_match_and_reaches_victory() {
  let binary = binary_path();
  let commands = [
    "observe",
    "rotate 1 bot_river",
    "advance",
    "ward allied 3 bot_river 3",
    "advance",
    "idle",
    "advance",
    "idle",
    "advance",
    "idle",
    "advance",
    "contest bot 4000",
    "advance",
    "siege outer mid 4000",
    "advance",
    "idle",
    "advance",
    "siege inner mid 4500",
    "advance",
    "idle",
    "advance",
    "siege inhibitor_turret mid 5000",
    "advance",
    "siege inhibitor mid 3500",
    "advance",
    "rotate 2 opposing_base",
    "advance",
    "siege nexus 6500",
    "advance",
    "evaluate",
    "advance",
    "debrief",
    "quit",
  ]
  .join("\n");
  let input = format!("{commands}\n");

  let mut child = Command::new(&binary)
    .args(["--scenario", "m9-interactive-match-v1"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn m9 interactive match binary");

  child
    .stdin
    .as_mut()
    .expect("stdin")
    .write_all(input.as_bytes())
    .expect("write stdin");

  let output = child.wait_with_output().expect("wait for match binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 transcript");

  assert!(stdout.contains("match_observation: turn=1 status=in_progress"));
  assert!(stdout.contains("actor: id=4 team=opposing location=unknown"));
  assert!(!stdout.contains("actor: id=4 team=opposing location=lane:mid:far-side"));
  assert!(stdout.contains("advanced: turn=1 action=rotation"));
  assert!(stdout.contains("advanced: turn=2 action=warding"));
  assert!(stdout.contains("advanced: turn=6 action=objective-contest"));
  assert!(stdout.contains("advanced: turn=7 action=structure-siege"));
  assert!(stdout.contains(
    "advanced: turn=15 action=terminal-evaluation events=0 effects=0 match_status=concluded"
  ));
  assert!(stdout.contains("match_debrief: scenario=scenario-complete-allied-snowball-v1 winner=allied condition=nexus-demolished final_turn=14"));
  assert!(stdout.contains("quit: session=closed"));
}

#[test]
fn binary_rejects_in_progress_m9_debrief() {
  let binary = binary_path();
  let mut child = Command::new(&binary)
    .args(["--scenario", "m9-interactive-match-v1", "--color", "never"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn m9 interactive match binary");

  child
    .stdin
    .as_mut()
    .expect("stdin")
    .write_all(b"debrief\nquit\n")
    .expect("write stdin");

  let output = child.wait_with_output().expect("wait for match binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 transcript");
  assert!(stdout.contains("error: match debrief is unavailable until terminal evaluation"));
  assert!(!stdout.contains("match_debrief:"));
}

#[test]
fn binary_runs_mcp_serve_and_responds_to_json_rpc() {
  let binary = binary_path();
  let input = [
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
    r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"observe","arguments":{}}}"#,
    r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"match_observe","arguments":{}}}"#,
  ]
  .join("\n")
    + "\n";

  let mut child = Command::new(&binary)
    .args(["mcp", "serve"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn mcp serve binary");

  child
    .stdin
    .as_mut()
    .expect("stdin")
    .write_all(input.as_bytes())
    .expect("write stdin");

  let output = child.wait_with_output().expect("wait for mcp binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");

  assert!(stdout.contains(r#""name":"fog-of-intent""#));
  assert!(stdout.contains("observation:"));
  assert!(stdout.contains("match_observation:"));
}

#[test]
fn binary_runs_mcp_flag_and_subcommand_variants() {
  let binary = binary_path();
  let input = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string() + "\n";

  // Test --mcp flag
  let mut child = Command::new(&binary)
    .arg("--mcp")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn --mcp");
  child
    .stdin
    .as_mut()
    .unwrap()
    .write_all(input.as_bytes())
    .unwrap();
  let output = child.wait_with_output().unwrap();
  assert!(output.status.success());
  assert!(
    String::from_utf8(output.stdout)
      .unwrap()
      .contains(r#""id":1"#)
  );

  // Test mcp serve --transport stdio
  let mut child2 = Command::new(&binary)
    .args(["mcp", "serve", "--transport", "stdio"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn mcp serve --transport stdio");
  child2
    .stdin
    .as_mut()
    .unwrap()
    .write_all(input.as_bytes())
    .unwrap();
  let output2 = child2.wait_with_output().unwrap();
  assert!(output2.status.success());
  assert!(
    String::from_utf8(output2.stdout)
      .unwrap()
      .contains(r#""id":1"#)
  );
}

#[test]
fn binary_runs_m8_team_scenarios_and_prints_debrief_battery() {
  let output = Command::new(binary_path())
    .args(["--scenario", "m8-team-scenarios-v1"])
    .output()
    .expect("run m8 team scenarios");

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("m8 team scenarios UTF-8 output");
  assert!(
    stdout.starts_with("# Fog of Intent — Milestone M8 Team Communication & Shot-Calling Battery")
  );
  assert!(stdout.contains("scenario-high-trust-gank-v1"));
  assert!(stdout.contains("scenario-low-trust-dissent-v1"));
  assert!(stdout.contains("scenario-conflicting-calls-arbitration-v1"));
  assert!(stdout.contains("scenario-missing-message-fallback-v1"));
  assert!(stdout.contains("scenario-strategic-dissent-survival-v1"));
  assert!(stdout.contains("Strategic Disagreement Evaluation"));
  assert!(stdout.contains("LegitimateDissent"));
  assert!(stdout.contains("Benchmark Battery Summary"));
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_runs_m10_study_synthesis_and_prints_battery() {
  let output = Command::new(binary_path())
    .args(["--scenario", "m10-human-study-synthesis-v1"])
    .output()
    .expect("run m10 study synthesis");

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("m10 study synthesis UTF-8 output");
  assert!(stdout.starts_with(
    "# Fog of Intent — Milestone M10 Human Usability & Accessibility Alpha Synthesis Battery"
  ));
  assert!(stdout.contains("scenario-alpha-synthesis-baseline-v1"));
  assert!(stdout.contains("scenario-alpha-synthesis-accessibility-gated-v1"));
  assert!(stdout.contains("scenario-alpha-synthesis-sampling-gap-v1"));
  assert!(stdout.contains("Benchmark Battery Summary"));
  assert!(stdout.contains("AlphaReady"));
  assert!(stdout.contains("BlockedByReadinessGates"));
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_interactive_select_runs_study_synthesis_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "study\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains(
    "# Fog of Intent — Milestone M10 Human Usability & Accessibility Alpha Synthesis Battery"
  ));
  assert!(stdout.contains("scenario-alpha-synthesis-baseline-v1"));
  assert!(stdout.contains("Benchmark Battery Summary"));
}

#[test]
fn binary_runs_m6_behavioral_experiments_and_prints_battery() {
  let output = Command::new(binary_path())
    .args(["--scenario", "m6-behavioral-experiments-v1"])
    .output()
    .expect("run m6 behavioral experiments");

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("m6 behavioral experiments UTF-8 output");
  assert!(stdout.starts_with(
    "# Fog of Intent — Milestone M6 Automated Behavioral Experiments & Population Validation Battery"
  ));
  assert!(stdout.contains("cautious-laner-v1"));
  assert!(stdout.contains("risk-taking-laner-v1"));
  assert!(stdout.contains("yielding-laner-v1"));
  assert!(stdout.contains("Benchmark Battery Summary"));
  assert!(stdout.contains("**Regression Gate Status:** PASS"));
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_interactive_select_runs_behavioral_experiments_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "behavioral\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains(
    "# Fog of Intent — Milestone M6 Automated Behavioral Experiments & Population Validation Battery"
  ));
  assert!(stdout.contains("cautious-laner-v1"));
  assert!(stdout.contains("Benchmark Battery Summary"));
}

#[test]
fn binary_interactive_select_runs_reproducibility_bundle_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "reproducibility\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains("# Public Alpha Reproducibility Bundle Audit Report"));
  assert!(stdout.contains("PKG-BENCHMARK-01"));
}

#[test]
fn binary_runs_m7_calibration_proof_and_prints_battery() {
  let output = Command::new(binary_path())
    .args(["--scenario", "m7-calibration-proof-v1"])
    .output()
    .expect("run m7 calibration proof");

  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("m7 calibration proof UTF-8 output");
  assert!(stdout.starts_with(
    "# Fog of Intent — Milestone M7 Semantic-to-Parametric Calibration Proof Battery"
  ));
  assert!(stdout.contains("cautious-laner-semantic-v1"));
  assert!(stdout.contains("risk-taking-laner-semantic-v1"));
  assert!(stdout.contains("yielding-laner-semantic-v1"));
  assert!(stdout.contains("Diagnostic Choice Dilemma Catalog"));
  assert!(stdout.contains("Multi-Model Empirical Alignment"));
  assert!(stdout.contains("Calibration Proof Battery Summary"));
  assert!(stdout.contains("**Recalibration Trigger Gate Status:** PASS"));
  assert!(output.stderr.is_empty());
}

#[test]
fn binary_interactive_select_runs_calibration_proof_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "calibration\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(
    stdout
      .contains("# Fog of Intent — Milestone M7 Semantic-to-Parametric Calibration Proof Battery")
  );
  assert!(stdout.contains("cautious-laner-semantic-v1"));
  assert!(stdout.contains("Calibration Proof Battery Summary"));
}

#[test]
fn binary_prints_gui_browser_flow_report() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m11-gui-browser-flow-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 report");
  assert!(stdout.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
  assert!(stdout.contains("**Battery Status:** **ALL SCENARIOS VERIFIED PASS**"));
  assert!(stdout.contains("scenario-gui-browser-standard-flow-v1"));
  assert!(stdout.contains("scenario-gui-browser-network-recovery-v1"));
  assert!(stdout.contains("scenario-gui-browser-accessibility-flow-v1"));
  assert!(stdout.contains("scenario-gui-browser-degraded-fallback-v1"));
  assert!(stdout.ends_with('\n'));
}

#[test]
fn binary_interactive_select_runs_browser_flow_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "browser\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
  assert!(stdout.contains("scenario-gui-browser-standard-flow-v1"));
}

// --- Dedicated fog-of-intent-mcp standalone binary integration tests ---

#[test]
fn mcp_binary_help_and_version_are_successful() {
  let binary = mcp_binary_path();

  let help_output = Command::new(&binary)
    .arg("--help")
    .output()
    .expect("run mcp binary --help");
  assert!(
    help_output.status.success(),
    "stderr: {:?}",
    help_output.stderr
  );
  let help_text = String::from_utf8(help_output.stdout).expect("help UTF-8");
  assert!(help_text.contains("usage: fog-of-intent-mcp"));
  assert!(help_text.contains("--tools"));
  assert!(help_text.contains("--resources"));
  assert!(help_text.contains("--prompts"));

  let version_output = Command::new(&binary)
    .arg("--version")
    .output()
    .expect("run mcp binary --version");
  assert!(
    version_output.status.success(),
    "stderr: {:?}",
    version_output.stderr
  );
  let version_text = String::from_utf8(version_output.stdout).expect("version UTF-8");
  assert!(version_text.starts_with("fog-of-intent-mcp "));
}

#[test]
fn mcp_binary_tools_resources_prompts_listings() {
  let binary = mcp_binary_path();

  let tools_out = Command::new(&binary)
    .arg("--tools")
    .output()
    .expect("run mcp binary --tools");
  assert!(tools_out.status.success());
  let tools_text = String::from_utf8(tools_out.stdout).expect("tools UTF-8");
  assert!(tools_text.contains("# Fog of Intent MCP Tools Catalog"));
  assert!(tools_text.contains("`observe`:"));
  assert!(tools_text.contains("`gui_browser_flow_run`:"));

  let res_out = Command::new(&binary)
    .arg("--resources")
    .output()
    .expect("run mcp binary --resources");
  assert!(res_out.status.success());
  let res_text = String::from_utf8(res_out.stdout).expect("resources UTF-8");
  assert!(res_text.contains("# Fog of Intent MCP Resources Catalog"));
  assert!(res_text.contains("`fog-of-intent://scenario/rules`"));

  let prompts_out = Command::new(&binary)
    .arg("--prompts")
    .output()
    .expect("run mcp binary --prompts");
  assert!(prompts_out.status.success());
  let prompts_text = String::from_utf8(prompts_out.stdout).expect("prompts UTF-8");
  assert!(prompts_text.contains("# Fog of Intent MCP Prompts Catalog"));
  assert!(prompts_text.contains("`lane_decision_window`:"));
}

#[test]
fn mcp_binary_runs_stdio_json_rpc() {
  let binary = mcp_binary_path();

  let mut child = Command::new(&binary)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn mcp binary");

  let init_req =
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}"#;
  let tools_req = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
  let input = format!("{init_req}\n{tools_req}\n");

  child
    .stdin
    .as_mut()
    .expect("stdin")
    .write_all(input.as_bytes())
    .expect("write json-rpc requests");

  let output = child.wait_with_output().expect("wait for mcp binary");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("mcp stdout UTF-8");
  assert!(stdout.contains(r#""protocolVersion":"2024-11-05""#));
  assert!(stdout.contains(r#""serverInfo":{"name":"fog-of-intent""#));
  assert!(stdout.contains(r#""name":"observe""#));
  assert!(stdout.contains(r#""name":"gui_browser_flow_run""#));
}

#[test]
fn mcp_binary_rejects_invalid_args() {
  let binary = mcp_binary_path();

  let output = Command::new(&binary)
    .arg("--unknown-flag")
    .output()
    .expect("run mcp binary with invalid arg");
  assert!(!output.status.success());
  let stderr = String::from_utf8(output.stderr).expect("stderr UTF-8");
  assert!(stderr.contains("unexpected executable argument; use --help"));
}

#[test]
fn binary_prints_alpha_archive_report() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m12-alpha-archive-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 report");
  assert!(stdout.contains("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(stdout.contains("**Archive Disposition:** **READY FOR TAGGED RELEASE**"));
  assert!(stdout.contains("source-manifest"));
  assert!(stdout.contains("lockfile-inventory"));
  assert!(stdout.contains("reproducibility-bundle"));
  assert!(stdout.ends_with('\n'));
}

#[test]
fn binary_interactive_select_runs_alpha_archive_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "archive\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(stdout.contains("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(stdout.contains("**Archive Disposition:** **READY FOR TAGGED RELEASE**"));
}

#[test]
fn binary_prints_m10_cohort_study_report() {
  let binary = binary_path();

  let output = run_scenario_binary(&binary, "m10-empirical-cohort-study-v1", "");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 report");
  assert!(
    stdout.contains("# Fog of Intent — Milestone M10 Empirical Multi-Cohort Study Trials Battery")
  );
  assert!(stdout.contains("scenario-cohort-trial-balanced-alpha-v1"));
  assert!(stdout.contains("scenario-cohort-trial-access-focused-v1"));
  assert!(stdout.contains("scenario-cohort-trial-novice-onboarding-v1"));
  assert!(stdout.contains("scenario-cohort-trial-strategy-moba-contrast-v1"));
  assert!(stdout.contains("**Regression Gate Status:** PASS"));
  assert!(stdout.ends_with('\n'));
}

#[test]
fn binary_interactive_select_runs_cohort_study_via_alias() {
  let binary = binary_path();
  let output = run_select_binary(&binary, "trials\n");
  assert!(output.status.success(), "stderr: {:?}", output.stderr);
  let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
  assert!(stdout.contains("Fog of Intent — Scenario Selection"));
  assert!(
    stdout.contains("# Fog of Intent — Milestone M10 Empirical Multi-Cohort Study Trials Battery")
  );
  assert!(stdout.contains("scenario-cohort-trial-balanced-alpha-v1"));
}
