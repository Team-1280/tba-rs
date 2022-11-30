use std::sync::Arc;
use async_trait::async_trait;
use serde::Deserialize;
use crate::{ctx::Context, Error};

/// A team number that uniquely identifies an FRC [Team](super::team::Team)
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
pub struct TeamNumber(u32);

/// Trait implemented by all key references in the TBA API, with method to upgrade the reference
/// into a concrete value
#[async_trait(?Send)]
pub trait Key: Sized {
    type Referenced;
    
    async fn upgrade(self, ctx: &Context) -> Result<Arc<Self::Referenced>, Error>;
}

#[macro_export]
macro_rules! key {
    ($name:ident($internal:ty) -> $referenced:ty => ($this:ident, $ctxi:ident) with $ep:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, ::serde::Deserialize)]
        #[serde(transparent)]
        #[repr(transparent)]
        pub struct $name($internal);
        
        #[::async_trait::async_trait(?Send)]
        impl crate::model::id::Key for $name {
            type Referenced = $referenced;
            async fn upgrade($this: Self, $ctxi: &crate::ctx::Context) -> Result<::std::sync::Arc<Self::Referenced>, crate::Error> {
                $ep
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
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

impl std::fmt::Display for TeamNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
