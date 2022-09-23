use serde::Deserialize;

/// A team number that uniquely identifies an FRC [Team](super::team::Team)
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
pub struct TeamNumber(u32);

impl TeamNumber {
    /// Create a new `TeamNumber` with the given value
    pub const fn new(n: u32) -> Self {
        Self(n)
    }
    
    /// Get the team number from this wrapper structure
    pub const fn val(&self) -> u32 {
        self.0
    }
}

impl AsRef<u32> for TeamNumber {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
