//! Discrete map topology and spatial coordinate representations for M9.

use core::fmt;

/// Team side identification for bases and territory.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TeamSide {
  Allied,
  Opposing,
}

impl TeamSide {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Allied => "allied",
      Self::Opposing => "opposing",
    }
  }

  pub const fn opposing(self) -> Self {
    match self {
      Self::Allied => Self::Opposing,
      Self::Opposing => Self::Allied,
    }
  }
}

impl fmt::Display for TeamSide {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// The three canonical strategic lanes on the map.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LaneId {
  Top,
  Mid,
  Bot,
}

impl LaneId {
  pub const ALL: [Self; 3] = [Self::Top, Self::Mid, Self::Bot];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Top => "top",
      Self::Mid => "mid",
      Self::Bot => "bot",
    }
  }
}

impl fmt::Display for LaneId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Discrete position sector within a lane.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LaneSector {
  NearTower,
  Center,
  FarSide,
}

impl LaneSector {
  pub const ALL: [Self; 3] = [Self::NearTower, Self::Center, Self::FarSide];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::NearTower => "near-tower",
      Self::Center => "center",
      Self::FarSide => "far-side",
    }
  }
}

impl fmt::Display for LaneSector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// River segments connecting lanes and jungle zones.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RiverSide {
  TopRiver,
  BotRiver,
}

impl RiverSide {
  pub const ALL: [Self; 2] = [Self::TopRiver, Self::BotRiver];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::TopRiver => "top-river",
      Self::BotRiver => "bot-river",
    }
  }
}

impl fmt::Display for RiverSide {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Jungle quadrants between lanes and river.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JungleSide {
  TopJungle,
  BotJungle,
}

impl JungleSide {
  pub const ALL: [Self; 2] = [Self::TopJungle, Self::BotJungle];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::TopJungle => "top-jungle",
      Self::BotJungle => "bot-jungle",
    }
  }
}

impl fmt::Display for JungleSide {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Discrete spatial location on the three-lane map.
///
/// Total locations: 2 bases + 9 lane sectors + 2 river zones + 2 jungle zones = 15 locations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MapLocation {
  Base(TeamSide),
  Lane(LaneId, LaneSector),
  River(RiverSide),
  Jungle(JungleSide),
}

impl MapLocation {
  pub const ALLIED_BASE: Self = Self::Base(TeamSide::Allied);
  pub const OPPOSING_BASE: Self = Self::Base(TeamSide::Opposing);

  pub const TOP_NEAR_TOWER: Self = Self::Lane(LaneId::Top, LaneSector::NearTower);
  pub const TOP_CENTER: Self = Self::Lane(LaneId::Top, LaneSector::Center);
  pub const TOP_FAR_SIDE: Self = Self::Lane(LaneId::Top, LaneSector::FarSide);

  pub const MID_NEAR_TOWER: Self = Self::Lane(LaneId::Mid, LaneSector::NearTower);
  pub const MID_CENTER: Self = Self::Lane(LaneId::Mid, LaneSector::Center);
  pub const MID_FAR_SIDE: Self = Self::Lane(LaneId::Mid, LaneSector::FarSide);

  pub const BOT_NEAR_TOWER: Self = Self::Lane(LaneId::Bot, LaneSector::NearTower);
  pub const BOT_CENTER: Self = Self::Lane(LaneId::Bot, LaneSector::Center);
  pub const BOT_FAR_SIDE: Self = Self::Lane(LaneId::Bot, LaneSector::FarSide);

  pub const TOP_RIVER: Self = Self::River(RiverSide::TopRiver);
  pub const BOT_RIVER: Self = Self::River(RiverSide::BotRiver);

  pub const TOP_JUNGLE: Self = Self::Jungle(JungleSide::TopJungle);
  pub const BOT_JUNGLE: Self = Self::Jungle(JungleSide::BotJungle);

  pub const ALL_LOCATIONS: [Self; 15] = [
    Self::ALLIED_BASE,
    Self::OPPOSING_BASE,
    Self::TOP_NEAR_TOWER,
    Self::TOP_CENTER,
    Self::TOP_FAR_SIDE,
    Self::MID_NEAR_TOWER,
    Self::MID_CENTER,
    Self::MID_FAR_SIDE,
    Self::BOT_NEAR_TOWER,
    Self::BOT_CENTER,
    Self::BOT_FAR_SIDE,
    Self::TOP_RIVER,
    Self::BOT_RIVER,
    Self::TOP_JUNGLE,
    Self::BOT_JUNGLE,
  ];

  pub const fn index(self) -> usize {
    match self {
      Self::ALLIED_BASE => 0,
      Self::OPPOSING_BASE => 1,
      Self::TOP_NEAR_TOWER => 2,
      Self::TOP_CENTER => 3,
      Self::TOP_FAR_SIDE => 4,
      Self::MID_NEAR_TOWER => 5,
      Self::MID_CENTER => 6,
      Self::MID_FAR_SIDE => 7,
      Self::BOT_NEAR_TOWER => 8,
      Self::BOT_CENTER => 9,
      Self::BOT_FAR_SIDE => 10,
      Self::TOP_RIVER => 11,
      Self::BOT_RIVER => 12,
      Self::TOP_JUNGLE => 13,
      Self::BOT_JUNGLE => 14,
    }
  }

  pub const fn from_index(index: usize) -> Option<Self> {
    if index < Self::ALL_LOCATIONS.len() {
      Some(Self::ALL_LOCATIONS[index])
    } else {
      None
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ALLIED_BASE => "base:allied",
      Self::OPPOSING_BASE => "base:opposing",
      Self::TOP_NEAR_TOWER => "lane:top:near-tower",
      Self::TOP_CENTER => "lane:top:center",
      Self::TOP_FAR_SIDE => "lane:top:far-side",
      Self::MID_NEAR_TOWER => "lane:mid:near-tower",
      Self::MID_CENTER => "lane:mid:center",
      Self::MID_FAR_SIDE => "lane:mid:far-side",
      Self::BOT_NEAR_TOWER => "lane:bot:near-tower",
      Self::BOT_CENTER => "lane:bot:center",
      Self::BOT_FAR_SIDE => "lane:bot:far-side",
      Self::TOP_RIVER => "river:top",
      Self::BOT_RIVER => "river:bot",
      Self::TOP_JUNGLE => "jungle:top",
      Self::BOT_JUNGLE => "jungle:bot",
    }
  }

  pub const fn is_base(self) -> bool {
    matches!(self, Self::Base(_))
  }

  pub const fn is_lane(self) -> bool {
    matches!(self, Self::Lane(_, _))
  }

  pub const fn is_river(self) -> bool {
    matches!(self, Self::River(_))
  }

  pub const fn is_jungle(self) -> bool {
    matches!(self, Self::Jungle(_))
  }

  pub const fn lane_id(self) -> Option<LaneId> {
    match self {
      Self::Lane(lane_id, _) => Some(lane_id),
      _ => None,
    }
  }

  pub const fn lane_sector(self) -> Option<LaneSector> {
    match self {
      Self::Lane(_, sector) => Some(sector),
      _ => None,
    }
  }
}

impl fmt::Display for MapLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}
