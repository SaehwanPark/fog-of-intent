//! Deterministic HTML5/CSS/SVG GUI presentation document generator and verification.
//!
//! Generates standalone, self-contained, accessibility-compliant HTML5 documents
//! with procedural SVG vector maps, CSS design tokens, and W3C semantic landmarks.
//! All outputs strictly omit latent opponent state, true-state hashes, and private
//! chain-of-thought.

use core::fmt;

use crate::gui::dto::{
  GuiAccessibilityDto, GuiActiveTab, GuiDebriefViewDto, GuiMapViewDto, GuiPlanViewDto,
  GuiPresentationBundle, GuiTimelineViewDto, GuiVisionStatus,
};
use crate::gui::state::{GuiClientState, GuiDisplayOptions};

/// Schema version for actor-visible GUI HTML presentation generator.
pub const GUI_HTML_SCHEMA_VERSION: &str = "m11-gui-html-v1";

/// Error types encountered during GUI HTML rendering or verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiHtmlError {
  /// Presentation bundle failed invariant validation.
  BundleInvariantViolation(&'static str),
  /// Schema version mismatch.
  SchemaMismatch { expected: String, actual: String },
  /// HTML document is missing the standard W3C doctype declaration.
  MissingDoctype,
  /// HTML document is missing standard viewport metadata.
  MissingViewport,
  /// HTML document is missing one or more required semantic landmarks.
  MissingLandmark(&'static str),
  /// HTML document contains forbidden external network resources.
  ForbiddenExternalResource(String),
  /// HTML document contains forbidden client-side script tags.
  ForbiddenScriptTag,
  /// HTML document leaks private chain-of-thought or latent state.
  LatentInformationLeak(&'static str),
}

impl fmt::Display for GuiHtmlError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BundleInvariantViolation(msg) => write!(f, "bundle invariant violation: {msg}"),
      Self::SchemaMismatch { expected, actual } => {
        write!(f, "schema mismatch: expected {expected}, actual {actual}")
      }
      Self::MissingDoctype => write!(f, "missing standard '<!DOCTYPE html>' declaration"),
      Self::MissingViewport => write!(f, "missing standard viewport meta tag"),
      Self::MissingLandmark(name) => write!(f, "missing required semantic landmark: <{name}>"),
      Self::ForbiddenExternalResource(url) => {
        write!(f, "forbidden external resource reference: {url}")
      }
      Self::ForbiddenScriptTag => {
        write!(f, "forbidden '<script>' tag found in presentation document")
      }
      Self::LatentInformationLeak(msg) => write!(f, "latent information leak detected: {msg}"),
    }
  }
}

impl std::error::Error for GuiHtmlError {}

/// Structured verification report for an HTML presentation document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiHtmlVerificationReport {
  pub schema_version: String,
  pub document_title: String,
  pub byte_length: usize,
  pub has_valid_doctype: bool,
  pub has_viewport_meta: bool,
  pub has_all_landmarks: bool,
  pub zero_external_resources: bool,
  pub zero_script_tags: bool,
  pub zero_latent_leaks: bool,
  pub is_compliant: bool,
}

/// Render a complete, standalone, accessible HTML5/CSS/SVG document from a GUI presentation bundle and client state.
pub fn render_gui_html_document(
  bundle: &GuiPresentationBundle,
  state: &GuiClientState,
) -> Result<String, GuiHtmlError> {
  bundle
    .validate_invariants()
    .map_err(GuiHtmlError::BundleInvariantViolation)?;

  let title = format!(
    "Fog of Intent GUI — Turn {} ({})",
    bundle.turn, bundle.observer_role
  );
  let css = generate_vanilla_css(&state.display_options);
  let nav = render_navigation(state.active_tab);
  let main_content = render_active_tab_content(bundle, state);
  let sidebar = render_sidebar(bundle, state);

  let doc = format!(
    r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <style>
{css}
  </style>
</head>
<body>
  <header role="banner" class="gui-header">
    <div class="header-brand">
      <h1 class="brand-title">Fog of Intent</h1>
      <span class="brand-badge">GUI Presentation Client</span>
    </div>
    <div class="header-telemetry">
      <div class="telemetry-item"><span class="label">Turn:</span> <span class="val">{turn}</span></div>
      <div class="telemetry-item"><span class="label">Observer:</span> <span class="val">{role}</span></div>
      <div class="telemetry-item"><span class="label">Phase:</span> <span class="val">{phase}</span></div>
      <div class="telemetry-item"><span class="label">Density:</span> <span class="val">{mode}</span></div>
    </div>
  </header>
  {nav}
  <div class="gui-body-layout">
    <main role="main" class="gui-main">
{main_content}
    </main>
    <aside role="complementary" aria-label="Telemetry and Status" class="gui-sidebar">
{sidebar}
    </aside>
  </div>
  <footer role="contentinfo" class="gui-footer">
    <p class="footer-notice">Host-owned simulation truth. Pure presentation-only projection.</p>
    <p class="footer-schemas">DTO: {dto_schema} | HTML: {html_schema}</p>
  </footer>
</body>
</html>"##,
    title = title,
    css = css,
    nav = nav,
    main_content = main_content,
    sidebar = sidebar,
    turn = bundle.turn,
    role = bundle.observer_role,
    phase = bundle.timeline_view.current_phase,
    mode = state.display_options.view_mode.as_str(),
    dto_schema = bundle.schema_version,
    html_schema = GUI_HTML_SCHEMA_VERSION,
  );

  Ok(doc)
}

/// Verify that an HTML document satisfies all W3C, semantic, security, and invariant requirements.
pub fn verify_gui_html_document(
  html: &str,
  bundle: &GuiPresentationBundle,
) -> Result<GuiHtmlVerificationReport, GuiHtmlError> {
  if !html.starts_with("<!DOCTYPE html>") {
    return Err(GuiHtmlError::MissingDoctype);
  }
  if !html.contains(r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#) {
    return Err(GuiHtmlError::MissingViewport);
  }

  let required_landmarks = ["header", "nav", "main", "aside", "footer"];
  for landmark in required_landmarks {
    let tag = format!("<{landmark}");
    if !html.contains(&tag) {
      return Err(GuiHtmlError::MissingLandmark(landmark));
    }
  }

  // Security: check for external network resources and script injection
  let url_check_str = html.replace("http://www.w3.org/2000/svg", "");
  if url_check_str.contains("http://")
    || url_check_str.contains("https://")
    || url_check_str.contains("src=\"//")
    || url_check_str.contains("href=\"//")
  {
    let bad_idx = url_check_str
      .find("http://")
      .or_else(|| url_check_str.find("https://"))
      .unwrap_or(0);
    let sample: String = url_check_str[bad_idx..].chars().take(40).collect();
    return Err(GuiHtmlError::ForbiddenExternalResource(sample));
  }
  if html.contains("<script") || html.contains("javascript:") {
    return Err(GuiHtmlError::ForbiddenScriptTag);
  }

  // Information invariant: zero private chain-of-thought
  if html.contains("chain_of_thought")
    || html.contains("private_reasoning")
    || html.contains("internal_thought")
  {
    return Err(GuiHtmlError::LatentInformationLeak(
      "found private chain-of-thought marker in HTML",
    ));
  }

  // Information invariant: verify unseen opposing actors do not leak true positions
  for actor in &bundle.map_view.actors {
    if !actor.is_visible && actor.team == "Opposing" && actor.location_id != "Unknown" {
      return Err(GuiHtmlError::LatentInformationLeak(
        "unseen opposing actor true position leaked",
      ));
    }
  }

  let title = format!(
    "Fog of Intent GUI — Turn {} ({})",
    bundle.turn, bundle.observer_role
  );

  Ok(GuiHtmlVerificationReport {
    schema_version: GUI_HTML_SCHEMA_VERSION.to_string(),
    document_title: title,
    byte_length: html.len(),
    has_valid_doctype: true,
    has_viewport_meta: true,
    has_all_landmarks: true,
    zero_external_resources: true,
    zero_script_tags: true,
    zero_latent_leaks: true,
    is_compliant: true,
  })
}

fn generate_vanilla_css(options: &GuiDisplayOptions) -> String {
  let (bg_color, fg_color, card_bg, border_color, accent_color) = if options.high_contrast_enabled {
    ("#000000", "#ffffff", "#121212", "#ffffff", "#ffff00")
  } else {
    ("#1a1e24", "#e2e8f0", "#222730", "#3a4454", "#60a5fa")
  };

  let anim_rule = if options.reduced_motion_enabled {
    "*, *::before, *::after { animation-duration: 0.01ms !important; transition-duration: 0.01ms !important; }"
  } else {
    "transition: all 0.15s ease-in-out;"
  };

  let zoom_scale = f64::from(options.zoom_level_bp) / 10000.0;

  format!(
    r##"    :root {{
      --bg-color: {bg_color};
      --fg-color: {fg_color};
      --card-bg: {card_bg};
      --border-color: {border_color};
      --accent-color: {accent_color};
      --zoom-scale: {zoom_scale:.2};
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    {anim_rule}
    body {{
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      background-color: var(--bg-color);
      color: var(--fg-color);
      line-height: 1.5;
      padding: 1rem;
    }}
    .gui-header {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 0.75rem 1.25rem;
      background-color: var(--card-bg);
      border: 1px solid var(--border-color);
      border-radius: 6px;
      margin-bottom: 1rem;
    }}
    .brand-title {{ font-size: 1.25rem; font-weight: 700; color: var(--accent-color); }}
    .brand-badge {{ font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; margin-left: 0.5rem; opacity: 0.8; }}
    .header-telemetry {{ display: flex; gap: 1rem; font-size: 0.875rem; }}
    .telemetry-item .label {{ opacity: 0.7; margin-right: 0.25rem; }}
    .telemetry-item .val {{ font-weight: 600; }}
    .gui-nav {{
      display: flex;
      gap: 0.5rem;
      margin-bottom: 1rem;
      overflow-x: auto;
    }}
    .nav-tab {{
      padding: 0.5rem 1rem;
      border: 1px solid var(--border-color);
      background-color: var(--card-bg);
      color: var(--fg-color);
      text-decoration: none;
      border-radius: 4px;
      font-size: 0.875rem;
      font-weight: 500;
    }}
    .nav-tab.active {{
      background-color: var(--accent-color);
      color: #000000;
      border-color: var(--accent-color);
      font-weight: 700;
    }}
    .gui-body-layout {{
      display: grid;
      grid-template-columns: 1fr 300px;
      gap: 1rem;
      margin-bottom: 1rem;
    }}
    @media (max-width: 768px) {{
      .gui-body-layout {{ grid-template-columns: 1fr; }}
    }}
    .gui-main {{
      background-color: var(--card-bg);
      border: 1px solid var(--border-color);
      border-radius: 6px;
      padding: 1.25rem;
    }}
    .gui-sidebar {{
      background-color: var(--card-bg);
      border: 1px solid var(--border-color);
      border-radius: 6px;
      padding: 1.25rem;
    }}
    .gui-footer {{
      padding: 0.75rem;
      text-align: center;
      font-size: 0.75rem;
      opacity: 0.7;
      border-top: 1px solid var(--border-color);
    }}
    .section-title {{ font-size: 1.1rem; font-weight: 600; margin-bottom: 0.75rem; border-bottom: 1px solid var(--border-color); padding-bottom: 0.25rem; }}
    .card-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 0.75rem; }}
    .kpi-card {{ border: 1px solid var(--border-color); border-radius: 4px; padding: 0.75rem; background-color: var(--bg-color); }}
    .kpi-label {{ font-size: 0.75rem; opacity: 0.8; }}
    .kpi-score {{ font-size: 1.25rem; font-weight: 700; }}
    .kpi-tier {{ font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--accent-color); }}
    .tag-list {{ display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem; }}
    .symbol-tag {{ font-family: monospace; font-size: 0.75rem; border: 1px solid var(--border-color); padding: 0.2rem 0.5rem; border-radius: 3px; }}
    .svg-map-container {{ width: 100%; height: 320px; background-color: #0b0f14; border: 1px solid var(--border-color); border-radius: 4px; display: flex; justify-content: center; align-items: center; }}"##
  )
}

fn render_navigation(active_tab: GuiActiveTab) -> String {
  let tabs = [
    (GuiActiveTab::MapView, "Map View"),
    (GuiActiveTab::TimelineView, "Timeline"),
    (GuiActiveTab::PlanView, "Plan & Focus"),
    (GuiActiveTab::DebriefView, "Causal Debrief"),
    (GuiActiveTab::AccessibilityView, "Accessibility"),
  ];

  let mut nav =
    String::from(r#"<nav role="navigation" aria-label="Main Navigation" class="gui-nav">"#);
  for (tab, label) in tabs {
    let is_active = tab == active_tab;
    let active_attr = if is_active {
      r#" class="nav-tab active" aria-current="page""#
    } else {
      r#" class="nav-tab""#
    };
    nav.push_str(&format!(
      r##"<a href="#{}"{}>{}</a>"##,
      tab.as_str(),
      active_attr,
      label
    ));
  }
  nav.push_str("</nav>");
  nav
}

fn render_active_tab_content(bundle: &GuiPresentationBundle, state: &GuiClientState) -> String {
  match state.active_tab {
    GuiActiveTab::MapView => render_map_view_html(&bundle.map_view, &state.display_options),
    GuiActiveTab::TimelineView => render_timeline_view_html(&bundle.timeline_view),
    GuiActiveTab::PlanView => render_plan_view_html(&bundle.plan_view),
    GuiActiveTab::DebriefView => render_debrief_view_html(bundle.debrief_view.as_ref()),
    GuiActiveTab::AccessibilityView => {
      render_accessibility_view_html(&bundle.accessibility, &state.display_options)
    }
  }
}

fn render_map_view_html(map: &GuiMapViewDto, options: &GuiDisplayOptions) -> String {
  let mut html = String::new();
  html.push_str(r#"<section class="tab-content map-tab">"#);
  html.push_str(r#"<h2 class="section-title">Spatial Map &amp; Fog of War</h2>"#);
  html.push_str(r#"<div class="svg-map-container">"#);
  html.push_str(&render_procedural_svg_map(map, options));
  html.push_str("</div>");

  html.push_str(r#"<div style="margin-top: 1rem;">"#);
  html.push_str(
    r#"<h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Observed Map Locations</h3>"#,
  );
  html.push_str(r#"<div class="card-grid">"#);
  for loc in &map.locations {
    let vis_badge = match loc.vision_status {
      GuiVisionStatus::FullVision => "[FULL-VISION]",
      GuiVisionStatus::LastKnown => "[LAST-KNOWN]",
      GuiVisionStatus::ConcealedInFog => "[IN-FOG]",
    };
    html.push_str(&format!(
      r#"<div class="kpi-card"><div class="kpi-label">{}</div><div class="kpi-score" style="font-size: 1rem;">{}</div><div class="kpi-tier">{}</div></div>"#,
      loc.location_id, loc.terrain_kind, vis_badge
    ));
  }
  html.push_str("</div></div>");
  html.push_str("</section>");
  html
}

fn render_procedural_svg_map(map: &GuiMapViewDto, _options: &GuiDisplayOptions) -> String {
  let mut svg = String::from(
    r##"<svg role="img" aria-label="Tactical Map Canvas" viewBox="0 0 600 300" width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">"##,
  );
  svg.push_str(r##"<rect width="600" height="300" fill="#10151c"/>"##);
  svg.push_str(r##"<line x1="50" y1="150" x2="550" y2="150" stroke="#2c384a" stroke-width="2"/>"##);

  // Render locations
  let loc_coords = [
    ("BlueBase", 80, 150),
    ("TopLane", 200, 80),
    ("MidLane", 300, 150),
    ("BotLane", 200, 220),
    ("RedBase", 520, 150),
  ];
  for (id, cx, cy) in loc_coords {
    svg.push_str(&format!(
      r##"<circle cx="{cx}" cy="{cy}" r="16" fill="#1e293b" stroke="#475569" stroke-width="2"/>"##
    ));
    svg.push_str(&format!(
      r##"<text x="{cx}" y="{y}" fill="#94a3b8" font-size="10" text-anchor="middle">{id}</text>"##,
      y = cy + 28
    ));
  }

  // Render actors
  for actor in &map.actors {
    if actor.is_visible {
      let is_allied = actor.team == "Allied" || actor.team == map.observer_team;
      let color = if is_allied { "#3b82f6" } else { "#ef4444" };
      let badge = if is_allied { "[A]" } else { "[E]" };
      svg.push_str(&format!(
        r##"<circle cx="300" cy="140" r="10" fill="{color}" stroke="#ffffff" stroke-width="1.5"/>"##
      ));
      svg.push_str(&format!(
        r##"<text x="300" y="125" fill="#ffffff" font-size="9" font-weight="bold" text-anchor="middle">{badge} {role}</text>"##,
        role = actor.actor_role
      ));
    }
  }

  svg.push_str("</svg>");
  svg
}

fn render_timeline_view_html(timeline: &GuiTimelineViewDto) -> String {
  let mut html = String::new();
  html.push_str(r#"<section class="tab-content timeline-tab">"#);
  html.push_str(r#"<h2 class="section-title">Temporal Turn &amp; Event Timeline</h2>"#);
  html.push_str(r#"<div class="card-grid">"#);
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Current Turn</div><div class="kpi-score">{}</div><div class="kpi-tier">{}</div></div>"#,
    timeline.current_turn, timeline.current_phase
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Active Rotations</div><div class="kpi-score">{}</div><div class="kpi-tier">Actors in Transit</div></div>"#,
    timeline.active_rotations_count
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Pending Delayed Effects</div><div class="kpi-score">{}</div><div class="kpi-tier">Queue Depth</div></div>"#,
    timeline.pending_delayed_effects_count
  ));
  html.push_str("</div>");

  if !timeline.scheduled_objective_spawns.is_empty() {
    html.push_str(r#"<div style="margin-top: 1rem;">"#);
    html.push_str(
      r#"<h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Scheduled Objective Spawns</h3>"#,
    );
    html.push_str(r#"<div class="tag-list">"#);
    for spawn in &timeline.scheduled_objective_spawns {
      html.push_str(&format!(r#"<span class="symbol-tag">{}</span>"#, spawn));
    }
    html.push_str("</div></div>");
  }
  html.push_str("</section>");
  html
}

fn render_plan_view_html(plan: &GuiPlanViewDto) -> String {
  let mut html = String::new();
  html.push_str(r#"<section class="tab-content plan-tab">"#);
  html.push_str(r#"<h2 class="section-title">Plan, Focus &amp; Contingency</h2>"#);
  html.push_str(r#"<div class="card-grid">"#);
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Selected Intent</div><div class="kpi-score">{}</div><div class="kpi-tier">Primary Action</div></div>"#,
    plan.selected_intent
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Target Focus</div><div class="kpi-score">{}</div><div class="kpi-tier">Priority</div></div>"#,
    plan.target_focus
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Commitment</div><div class="kpi-score">{}</div><div class="kpi-tier">Investment Tier</div></div>"#,
    plan.commitment
  ));
  html.push_str("</div>");

  html.push_str(r#"<div style="margin-top: 1rem;">"#);
  html.push_str(
    r#"<h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Contingencies &amp; Messaging</h3>"#,
  );
  html.push_str(r#"<div class="tag-list">"#);
  if let Some(ping) = &plan.ping_signal {
    html.push_str(&format!(r#"<span class="symbol-tag">Ping: {ping}</span>"#));
  }
  if let Some(abort) = &plan.abort_condition {
    html.push_str(&format!(
      r#"<span class="symbol-tag">Abort On: {abort}</span>"#
    ));
  }
  if let Some(fb) = &plan.fallback_behavior {
    html.push_str(&format!(
      r#"<span class="symbol-tag">Fallback: {fb}</span>"#
    ));
  }
  if let Some(preview) = &plan.staged_message_preview {
    html.push_str(&format!(
      r#"<span class="symbol-tag">Message: {preview}</span>"#
    ));
  }
  html.push_str("</div></div>");
  html.push_str("</section>");
  html
}

fn render_debrief_view_html(debrief: Option<&GuiDebriefViewDto>) -> String {
  let mut html = String::new();
  html.push_str(r#"<section class="tab-content debrief-tab">"#);
  html.push_str(r#"<h2 class="section-title">Causal Attribution Debrief</h2>"#);

  if let Some(deb) = debrief {
    html.push_str(r#"<div class="card-grid">"#);
    html.push_str(&format!(
      r#"<div class="kpi-card"><div class="kpi-label">Attribution Quadrant</div><div class="kpi-score" style="font-size: 1.1rem;">{}</div><div class="kpi-tier">2D Category</div></div>"#,
      deb.quadrant
    ));
    html.push_str(&format!(
      r#"<div class="kpi-card"><div class="kpi-label">Coordination Score</div><div class="kpi-score">{} bp</div><div class="kpi-tier">{}</div></div>"#,
      deb.coordination_score_bp, deb.coordination_rating
    ));
    html.push_str(&format!(
      r#"<div class="kpi-card"><div class="kpi-label">Execution Score</div><div class="kpi-score">{} bp</div><div class="kpi-tier">{}</div></div>"#,
      deb.execution_score_bp, deb.execution_rating
    ));
    html.push_str("</div>");

    if !deb.kpi_cards.is_empty() {
      html.push_str(r#"<div style="margin-top: 1rem;"><h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Causal KPI Breakdown</h3><div class="card-grid">"#);
      for kpi in &deb.kpi_cards {
        html.push_str(&format!(
          r#"<div class="kpi-card"><div class="kpi-label">{}</div><div class="kpi-score">{} bp</div><div class="kpi-tier">{}</div></div>"#,
          kpi.label, kpi.score_bp, kpi.tier
        ));
      }
      html.push_str("</div></div>");
    }

    if !deb.causal_factor_tags.is_empty() {
      html.push_str(r#"<div style="margin-top: 1rem;"><h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Attributed Causal Factors</h3><div class="tag-list">"#);
      for tag in &deb.causal_factor_tags {
        html.push_str(&format!(r#"<span class="symbol-tag">{}</span>"#, tag));
      }
      html.push_str("</div></div>");
    }
  } else {
    html
      .push_str(r#"<p style="opacity: 0.8;">No causal debrief available for the active turn.</p>"#);
  }

  html.push_str("</section>");
  html
}

fn render_accessibility_view_html(
  acc: &GuiAccessibilityDto,
  options: &GuiDisplayOptions,
) -> String {
  let mut html = String::new();
  html.push_str(r#"<section class="tab-content accessibility-tab">"#);
  html.push_str(
    r#"<h2 class="section-title">Accessibility &amp; Universal Usability (WCAG 2.1 AA)</h2>"#,
  );

  html.push_str(r#"<div class="card-grid">"#);
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">High Contrast</div><div class="kpi-score">{}</div><div class="kpi-tier">Display Mode</div></div>"#,
    if options.high_contrast_enabled { "Active" } else { "Standard" }
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Reduced Motion</div><div class="kpi-score">{}</div><div class="kpi-tier">Animation Rule</div></div>"#,
    if options.reduced_motion_enabled { "Enforced" } else { "Enabled" }
  ));
  html.push_str(&format!(
    r#"<div class="kpi-card"><div class="kpi-label">Symbol Tags</div><div class="kpi-score">{}</div><div class="kpi-tier">Non-Color Mode</div></div>"#,
    if options.symbol_tags_visible { "Active" } else { "Standard" }
  ));
  html.push_str("</div>");

  if !acc.non_color_symbol_tags.is_empty() {
    html.push_str(r#"<div style="margin-top: 1rem;"><h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Registered Non-Color Symbolic Tags</h3><div class="tag-list">"#);
    for tag in &acc.non_color_symbol_tags {
      html.push_str(&format!(
        r#"<span class="symbol-tag">[{}] {}: {}</span>"#,
        tag.symbol_code, tag.entity_id, tag.label
      ));
    }
    html.push_str("</div></div>");
  }

  if !acc.aria_announcements.is_empty() {
    html.push_str(r#"<div style="margin-top: 1rem;" role="region" aria-live="polite"><h3 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Active ARIA Live Announcements</h3><div class="tag-list">"#);
    for ann in &acc.aria_announcements {
      html.push_str(&format!(r#"<span class="symbol-tag">{}</span>"#, ann));
    }
    html.push_str("</div></div>");
  }

  html.push_str("</section>");
  html
}

fn render_sidebar(bundle: &GuiPresentationBundle, state: &GuiClientState) -> String {
  let mut html = String::new();
  html.push_str(r#"<h2 class="section-title">Client State</h2>"#);
  html.push_str(&format!(
    r#"<p style="font-size: 0.85rem; margin-bottom: 0.5rem;"><strong>Active Tab:</strong> {}</p>"#,
    state.active_tab.as_str()
  ));
  html.push_str(&format!(
    r#"<p style="font-size: 0.85rem; margin-bottom: 0.5rem;"><strong>View Mode:</strong> {}</p>"#,
    state.display_options.view_mode.as_str()
  ));
  html.push_str(&format!(
    r#"<p style="font-size: 0.85rem; margin-bottom: 0.5rem;"><strong>Observer:</strong> {}</p>"#,
    state.observer_role
  ));

  html.push_str(r#"<div style="margin-top: 1rem; border-top: 1px solid var(--border-color); padding-top: 0.5rem;">"#);
  html.push_str(r#"<h3 style="font-size: 0.9rem; margin-bottom: 0.5rem;">Observer Context</h3>"#);
  html.push_str(&format!(
    r#"<p style="font-size: 0.8rem; opacity: 0.8;">Role: {}</p><p style="font-size: 0.8rem; opacity: 0.8;">Team: {}</p><p style="font-size: 0.8rem; opacity: 0.8;">Bundle: {}</p>"#,
    bundle.observer_role, bundle.map_view.observer_team, bundle.bundle_id
  ));
  html.push_str("</div>");

  html
}
