use std::marker::PhantomData;

use serde::Deserialize;

use crate::{ctx::Context, Error};

/// A team number that uniquely identifies an FRC [Team](super::team::Team)
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
pub struct TeamNumber(u32);

/// Trait required by all types that are referenced by [Key]s in the TBA API, with a single method
/// to upgrade the key into the object it references
pub trait KeyReferenced: Sized {
    fn dereference(key: Key<Self>, ctx: &Context) -> Result<Self, Error>;
}

/// A key that references an element of type [T]
pub struct Key<T: KeyReferenced> {
    key: String,
    boo: PhantomData<T>,
}

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

impl<T: KeyReferenced> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self { key: self.key.clone(), boo: PhantomData }
    }
}
impl<T: KeyReferenced> std::fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)
    }
}
impl<T: KeyReferenced> std::fmt::Display for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)
    }
}

impl<'de, T: KeyReferenced> Deserialize<'de> for Key<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        Ok(Self {
            key: String::deserialize(deserializer)?,
            boo: PhantomData,
        }) 
    }
}
