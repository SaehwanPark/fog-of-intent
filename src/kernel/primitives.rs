//! Primitive value types and identifiers for the kernel.

use std::fmt;

pub const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

pub const MAX_UNITS: u8 = 10;
pub const CURRENT_RULESET: RulesetId = RulesetId(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorId(u8);

impl ActorId {
  pub const fn new(value: u8) -> Self {
    Self(value)
  }

  pub fn value(self) -> u8 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Turn(u32);

impl Turn {
  pub fn new(value: u32) -> Self {
    Self(value)
  }

  pub fn value(self) -> u32 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RulesetId(u16);

impl RulesetId {
  pub const fn new(value: u16) -> Self {
    Self(value)
  }

  pub fn value(self) -> u16 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StreamId(u8);

impl StreamId {
  pub fn new(value: u8) -> Self {
    Self(value)
  }

  pub fn value(self) -> u8 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DrawId(u16);

impl DrawId {
  pub fn new(value: u16) -> Self {
    Self(value)
  }

  pub fn value(self) -> u16 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateHash(u64);

impl StateHash {
  pub(crate) fn from_raw(value: u64) -> Self {
    Self(value)
  }

  pub fn value(self) -> u64 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Units(u8);

impl Units {
  pub fn new(value: u8) -> Result<Self, BoundsError> {
    if value <= MAX_UNITS {
      Ok(Self(value))
    } else {
      Err(BoundsError {
        value,
        maximum: MAX_UNITS,
      })
    }
  }

  pub fn zero() -> Self {
    Self(0)
  }

  pub fn value(self) -> u8 {
    self.0
  }

  pub(crate) fn subtract(self, amount: Self) -> Option<Self> {
    self.0.checked_sub(amount.0).map(Self)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundsError {
  pub value: u8,
  pub maximum: u8,
}

impl fmt::Display for BoundsError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{} exceeds maximum {}", self.value, self.maximum)
  }
}

pub(crate) fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
  for byte in bytes {
    hash ^= u64::from(*byte);
    hash = hash.wrapping_mul(FNV_PRIME);
  }
  hash
}
