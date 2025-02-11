//! Power policy actions
//! This modules contains wrapper structs that use type states to enforce the valid actions for each device state
use super::device::StateKind;

mod device;
mod policy;

pub use device::*;
pub use policy::*;

/// Trait to provide the kind of a state type
pub trait Kind {
    /// Return the kind of a state type
    fn kind() -> StateKind;
}

/// State type for a detached device
pub struct Detached;
impl Kind for Detached {
    fn kind() -> StateKind {
        StateKind::Detached
    }
}

/// State type for an attached device
pub struct Idle;
impl Kind for Idle {
    fn kind() -> StateKind {
        StateKind::Idle
    }
}

/// State type for a device that is providing power
pub struct ConnectedProvider;
impl Kind for ConnectedProvider {
    fn kind() -> StateKind {
        StateKind::ConnectedProvider
    }
}

/// State type for a device that is consuming power
pub struct ConnectedConsumer;
impl Kind for ConnectedConsumer {
    fn kind() -> StateKind {
        StateKind::ConnectedConsumer
    }
}
