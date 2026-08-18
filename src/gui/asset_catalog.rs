//! Canonical benchmark asset governance scenarios for M11.

use super::asset::{
  AssetFallbackKind, AssetGovernanceAuditReport, AssetGovernanceError, AssetGovernanceManifest,
  AssetKind, AssetLicense, AssetRecord, audit_asset_governance,
};

/// Canonical schema version for the M11 GUI asset catalog contract.
pub const GUI_ASSET_CATALOG_SCHEMA_VERSION: &str = "m11-gui-asset-catalog-v1";

/// Definition of a benchmark asset governance scenario.
#[derive(Debug, Clone)]
pub struct AssetGovernanceScenarioDefinition {
  pub scenario_id: &'static str,
  pub description: &'static str,
  pub manifest: AssetGovernanceManifest,
  pub expected_total_assets: usize,
  pub expected_gates_passed: bool,
}

impl AssetGovernanceScenarioDefinition {
  pub fn execute(&self) -> Result<AssetGovernanceAuditReport, AssetGovernanceError> {
    audit_asset_governance(&self.manifest)
  }
}

/// Catalog of canonical benchmark asset governance scenarios.
pub struct AssetGovernanceCatalog;

impl AssetGovernanceCatalog {
  pub fn get(scenario_id: &str) -> Option<AssetGovernanceScenarioDefinition> {
    match scenario_id {
      "scenario-gui-asset-core-v1" => Some(Self::scenario_core_v1()),
      "scenario-gui-asset-minimal-vector-v1" => Some(Self::scenario_minimal_vector_v1()),
      "scenario-gui-asset-fallback-audit-v1" => Some(Self::scenario_fallback_audit_v1()),
      _ => None,
    }
  }

  pub fn list() -> Vec<&'static str> {
    vec![
      "scenario-gui-asset-core-v1",
      "scenario-gui-asset-minimal-vector-v1",
      "scenario-gui-asset-fallback-audit-v1",
    ]
  }

  fn scenario_core_v1() -> AssetGovernanceScenarioDefinition {
    AssetGovernanceScenarioDefinition {
      scenario_id: "scenario-gui-asset-core-v1",
      description: "Complete core GUI asset bundle covering map, roles, structures, objectives, UI, and audio with 100% fallback and permissive licenses",
      manifest: AssetGovernanceManifest {
        manifest_id: "manifest-gui-assets-core-v1".to_string(),
        version: "1.0.0".to_string(),
        assets: vec![
          AssetRecord {
            asset_id: "asset-map-terrain-base-v1".to_string(),
            kind: AssetKind::MapTexture,
            license: AssetLicense::Mit,
            author: "Fog of Intent Vector Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/map_terrain.svg"
              .to_string(),
            content_hash: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
              .to_string(),
            fallback_kind: AssetFallbackKind::ProceduralVector,
            fallback_symbol: "[MAP-BASE]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-actor-top-laner-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Cc0,
            author: "OpenGameArt Contributor".to_string(),
            source_uri: "https://opengameart.org/content/top-laner-token".to_string(),
            content_hash: "sha256:ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
              .to_string(),
            fallback_kind: AssetFallbackKind::TextualGlyph,
            fallback_symbol: "[TOP]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-actor-jungler-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Cc0,
            author: "OpenGameArt Contributor".to_string(),
            source_uri: "https://opengameart.org/content/jungler-token".to_string(),
            content_hash: "sha256:3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d"
              .to_string(),
            fallback_kind: AssetFallbackKind::TextualGlyph,
            fallback_symbol: "[JNG]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-actor-mid-laner-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Cc0,
            author: "OpenGameArt Contributor".to_string(),
            source_uri: "https://opengameart.org/content/mid-laner-token".to_string(),
            content_hash: "sha256:2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6"
              .to_string(),
            fallback_kind: AssetFallbackKind::TextualGlyph,
            fallback_symbol: "[MID]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-actor-bot-carry-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Cc0,
            author: "OpenGameArt Contributor".to_string(),
            source_uri: "https://opengameart.org/content/bot-carry-token".to_string(),
            content_hash: "sha256:18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4"
              .to_string(),
            fallback_kind: AssetFallbackKind::TextualGlyph,
            fallback_symbol: "[BOT]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-actor-support-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Cc0,
            author: "OpenGameArt Contributor".to_string(),
            source_uri: "https://opengameart.org/content/support-token".to_string(),
            content_hash: "sha256:3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea"
              .to_string(),
            fallback_kind: AssetFallbackKind::TextualGlyph,
            fallback_symbol: "[SUP]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-structure-turret-v1".to_string(),
            kind: AssetKind::StructureIcon,
            license: AssetLicense::Mit,
            author: "Fog of Intent Vector Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/turret.svg"
              .to_string(),
            content_hash: "sha256:4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[TURRET]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-objective-dragon-v1".to_string(),
            kind: AssetKind::ObjectiveIcon,
            license: AssetLicense::Mit,
            author: "Fog of Intent Vector Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/dragon.svg"
              .to_string(),
            content_hash: "sha256:ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[DRAGON]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-ui-ping-caution-v1".to_string(),
            kind: AssetKind::UiIcon,
            license: AssetLicense::Apache2,
            author: "Material Design Icons".to_string(),
            source_uri: "https://fonts.google.com/icons?selected=Material+Icons:warning"
              .to_string(),
            content_hash: "sha256:e7f6c011776e8db7cd330b54174fd76f7d0216b612387a5ffcfb81e6f0919683"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[PING-WARN]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audio-ping-alert-v1".to_string(),
            kind: AssetKind::AudioCue,
            license: AssetLicense::Cc0,
            author: "Freesound Public Domain".to_string(),
            source_uri: "https://freesound.org/people/contributor/sounds/12345/".to_string(),
            content_hash: "sha256:7902699be42c8a8e46fbbb4501726517e86b22c56a189f7625a6da49081b2451"
              .to_string(),
            fallback_kind: AssetFallbackKind::SilentVisualCue,
            fallback_symbol: "[ALERT-FLASH]".to_string(),
          },
        ],
      },
      expected_total_assets: 10,
      expected_gates_passed: true,
    }
  }

  fn scenario_minimal_vector_v1() -> AssetGovernanceScenarioDefinition {
    AssetGovernanceScenarioDefinition {
      scenario_id: "scenario-gui-asset-minimal-vector-v1",
      description: "Minimal procedural vector asset bundle for low-overhead or headless rendering environments",
      manifest: AssetGovernanceManifest {
        manifest_id: "manifest-gui-assets-minimal-vector-v1".to_string(),
        version: "1.0.0".to_string(),
        assets: vec![
          AssetRecord {
            asset_id: "asset-vec-map-grid-v1".to_string(),
            kind: AssetKind::MapTexture,
            license: AssetLicense::PublicDomain,
            author: "Fog of Intent Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/grid.svg".to_string(),
            content_hash: "sha256:a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
              .to_string(),
            fallback_kind: AssetFallbackKind::ProceduralVector,
            fallback_symbol: "[GRID]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-vec-actor-token-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::PublicDomain,
            author: "Fog of Intent Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/token.svg".to_string(),
            content_hash: "sha256:3b6697b69c470125c56c7cbbd75e47854eb1ce467fae322304882e307304bf54"
              .to_string(),
            fallback_kind: AssetFallbackKind::ProceduralVector,
            fallback_symbol: "[TOKEN]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-vec-structure-base-v1".to_string(),
            kind: AssetKind::StructureIcon,
            license: AssetLicense::PublicDomain,
            author: "Fog of Intent Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/structure.svg"
              .to_string(),
            content_hash: "sha256:f52fbd32b2b3b86ff88ef6c490628285f482af15ddcb29541f94bcf526a3f6c7"
              .to_string(),
            fallback_kind: AssetFallbackKind::ProceduralVector,
            fallback_symbol: "[STRUCT]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-vec-objective-star-v1".to_string(),
            kind: AssetKind::ObjectiveIcon,
            license: AssetLicense::PublicDomain,
            author: "Fog of Intent Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/star.svg".to_string(),
            content_hash: "sha256:252f10c83610ebca1a059c0bae8255eba2f95be4d1d7bcfa89d7248a82d9f111"
              .to_string(),
            fallback_kind: AssetFallbackKind::ProceduralVector,
            fallback_symbol: "[OBJ]".to_string(),
          },
        ],
      },
      expected_total_assets: 4,
      expected_gates_passed: true,
    }
  }

  fn scenario_fallback_audit_v1() -> AssetGovernanceScenarioDefinition {
    AssetGovernanceScenarioDefinition {
      scenario_id: "scenario-gui-asset-fallback-audit-v1",
      description: "Accessibility and fallback audit manifest verifying non-color symbolic tags and silent visual cues for audio assets",
      manifest: AssetGovernanceManifest {
        manifest_id: "manifest-gui-assets-fallback-audit-v1".to_string(),
        version: "1.0.0".to_string(),
        assets: vec![
          AssetRecord {
            asset_id: "asset-audit-high-contrast-map-v1".to_string(),
            kind: AssetKind::MapTexture,
            license: AssetLicense::CustomPermissive,
            author: "A11y Map Project".to_string(),
            source_uri: "https://a11y-games.org/assets/hc_map.svg".to_string(),
            content_hash: "sha256:cd2eb0837c9b4c962c22d2ff8b5441b7b45805887f051d39bf133b5836acbe47"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[HC-MAP]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audit-symbol-ally-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Mit,
            author: "A11y Map Project".to_string(),
            source_uri: "https://a11y-games.org/assets/ally.svg".to_string(),
            content_hash: "sha256:2c624232cdd221771294dfbb310aca000a0df6ac8b66b696d90efD9f50b12147"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[ALLY-TAG]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audit-symbol-enemy-v1".to_string(),
            kind: AssetKind::ActorSprite,
            license: AssetLicense::Mit,
            author: "A11y Map Project".to_string(),
            source_uri: "https://a11y-games.org/assets/enemy.svg".to_string(),
            content_hash: "sha256:19581e27de7ced00ff1ce50b2047e7a567c76b1cbaebabe5ef03f7c3017bb5b7"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[ENEMY-TAG]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audit-audio-ping-assist-v1".to_string(),
            kind: AssetKind::AudioCue,
            license: AssetLicense::Cc0,
            author: "Freesound A11y".to_string(),
            source_uri: "https://freesound.org/people/a11y/sounds/67890/".to_string(),
            content_hash: "sha256:4a44dc15364204a80fe80e9039455cc1608281820fe2b24f1e5233ade6af1dd5"
              .to_string(),
            fallback_kind: AssetFallbackKind::SilentVisualCue,
            fallback_symbol: "[ASSIST-BANNER]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audit-audio-objective-spawn-v1".to_string(),
            kind: AssetKind::AudioCue,
            license: AssetLicense::Cc0,
            author: "Freesound A11y".to_string(),
            source_uri: "https://freesound.org/people/a11y/sounds/67891/".to_string(),
            content_hash: "sha256:88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589"
              .to_string(),
            fallback_kind: AssetFallbackKind::SilentVisualCue,
            fallback_symbol: "[SPAWN-BANNER]".to_string(),
          },
          AssetRecord {
            asset_id: "asset-audit-structure-inhibitor-v1".to_string(),
            kind: AssetKind::StructureIcon,
            license: AssetLicense::Mit,
            author: "Fog of Intent Vector Team".to_string(),
            source_uri: "https://github.com/SaehwanPark/fog-of-intent/assets/inhibitor.svg"
              .to_string(),
            content_hash: "sha256:a541724a0f4414e21a221f5be67aa3c0765c92c300f8628ef34eb51da039b5d3"
              .to_string(),
            fallback_kind: AssetFallbackKind::NonColorSymbolicTag,
            fallback_symbol: "[INHIBITOR]".to_string(),
          },
        ],
      },
      expected_total_assets: 6,
      expected_gates_passed: true,
    }
  }
}
