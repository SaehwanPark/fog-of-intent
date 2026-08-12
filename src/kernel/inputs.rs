//! Input trace categories and resolved execution inputs.

use super::primitives::{DrawId, StreamId, Units};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InputTrace {
  stream: StreamId,
  draw: DrawId,
}

impl InputTrace {
  pub fn new(stream: StreamId, draw: DrawId) -> Self {
    Self { stream, draw }
  }

  pub fn stream(self) -> StreamId {
    self.stream
  }

  pub fn draw(self) -> DrawId {
    self.draw
  }
}

macro_rules! input_category {
  ($name:ident) => {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct $name {
      trace: InputTrace,
    }

    impl $name {
      pub fn new(trace: InputTrace) -> Self {
        Self { trace }
      }

      pub fn trace(self) -> InputTrace {
        self.trace
      }
    }
  };
}

input_category!(EnvironmentInputs);
input_category!(ObservationInputs);
input_category!(PolicyInputs);
input_category!(CoordinationInputs);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExecutionInputs {
  trace: InputTrace,
  yielded: Units,
}

impl ExecutionInputs {
  pub fn new(trace: InputTrace, yielded: Units) -> Self {
    Self { trace, yielded }
  }

  pub fn trace(self) -> InputTrace {
    self.trace
  }

  pub fn yielded(self) -> Units {
    self.yielded
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResolvedInputs {
  environment: EnvironmentInputs,
  observation: ObservationInputs,
  policy: PolicyInputs,
  coordination: CoordinationInputs,
  execution: ExecutionInputs,
}

impl ResolvedInputs {
  pub fn new(
    environment: EnvironmentInputs,
    observation: ObservationInputs,
    policy: PolicyInputs,
    coordination: CoordinationInputs,
    execution: ExecutionInputs,
  ) -> Self {
    Self {
      environment,
      observation,
      policy,
      coordination,
      execution,
    }
  }

  pub fn for_execution(execution: ExecutionInputs) -> Self {
    let neutral = InputTrace::new(StreamId::new(0), DrawId::new(0));
    Self::new(
      EnvironmentInputs::new(neutral),
      ObservationInputs::new(neutral),
      PolicyInputs::new(neutral),
      CoordinationInputs::new(neutral),
      execution,
    )
  }

  pub fn environment(self) -> EnvironmentInputs {
    self.environment
  }

  pub fn observation(self) -> ObservationInputs {
    self.observation
  }

  pub fn policy(self) -> PolicyInputs {
    self.policy
  }

  pub fn coordination(self) -> CoordinationInputs {
    self.coordination
  }

  pub fn execution(self) -> ExecutionInputs {
    self.execution
  }
}
