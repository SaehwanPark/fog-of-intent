//! Bounded CLI transcript for M9 composed complete-match replays.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This module projects the canonical composed complete matches into stable
//! labeled plain text for the executable `--scenario m9-complete-match-replay-v1`
//! surface. It is a pure projection: each canonical plan is executed and then
//! replay-verified by re-execution, and any hash mismatch fails closed
//! instead of printing an unverified transcript. No hash values, resolved
//! inputs, or hidden state appear in the output — only categorical labels.

use crate::map::complete_match::CompleteMatchResult;
use crate::map::complete_match_catalog::CompleteMatchCatalog;

/// Executable scenario id for the complete-match replay transcript.
pub const CLI_MATCH_REPLAY_SCENARIO_ID: &str = "m9-complete-match-replay-v1";

/// Labeled transcript of replay-verified complete matches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchReplayTranscript {
  lines: Vec<String>,
}

impl MatchReplayTranscript {
  /// The labeled lines in output order.
  pub fn lines(&self) -> &[String] {
    &self.lines
  }
}

/// Build the complete-match replay transcript: every canonical complete
/// match executed once and replay-verified by full re-execution.
///
/// Pure function — deterministic, no I/O. Fails closed when a plan cannot
/// execute or its replay does not reproduce the committed hashes.
pub fn build_match_replay_transcript() -> Result<MatchReplayTranscript, &'static str> {
  let mut lines = vec!["match-replay: begin".to_owned()];
  for plan in CompleteMatchCatalog::all() {
    let first = plan
      .execute()
      .map_err(|_| "match-replay: plan execution failed")?;
    let replay = plan
      .execute()
      .map_err(|_| "match-replay: replay execution failed")?;
    if first.initial_hash != replay.initial_hash
      || first.final_hash != replay.final_hash
      || first != replay
    {
      return Err("match-replay: replay verification mismatch");
    }
    lines.push(match_line(&first));
    lines.push(replay_line(first.scenario_id));
  }
  lines.push("match-replay: complete".to_owned());
  Ok(MatchReplayTranscript { lines })
}

fn match_line(result: &CompleteMatchResult) -> String {
  format!(
    "match: scenario={} winner={} condition={} final-turn={} objectives-allied={} objectives-opposing={} phases={} events={} effects={}",
    result.scenario_id,
    winner_label(result.winner),
    result.condition.as_str(),
    result.final_turn,
    result.allied_objectives_secured,
    result.opposing_objectives_secured,
    result.phases.len(),
    result.total_events,
    result.total_effects,
  )
}

fn replay_line(scenario_id: &str) -> String {
  format!("replay: scenario={scenario_id} initial-hash-match=yes final-hash-match=yes")
}

fn winner_label(winner: crate::map::topology::TeamSide) -> &'static str {
  match winner {
    crate::map::topology::TeamSide::Allied => "allied",
    crate::map::topology::TeamSide::Opposing => "opposing",
  }
}
