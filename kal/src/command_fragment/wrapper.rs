use std::ops::{Deref, DerefMut};

/// Wrapper for [`Vec`] that separates elements by space
#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommaSeparated<T>(pub Vec<T>);

impl<T> Deref for CommaSeparated<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CommaSeparated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wrapper for [`Vec`] that separates elements by comma
#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpaceSeparated<T>(pub Vec<T>);

impl<T> Deref for SpaceSeparated<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SpaceSeparated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
