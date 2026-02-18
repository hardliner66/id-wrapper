use nanoid::nanoid;

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct WithId<T> {
    id: String,
    pub inner: T,
}

impl<T> WithId<T> {
    pub fn new(inner: T) -> Self
    {
        let id = format!("{}::{}", std::any::type_name::<T>(), nanoid!());
        Self { id, inner }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<T> Deref for WithId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for WithId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> From<T> for WithId<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone> Clone for WithId<T> {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl<T: PartialEq> PartialEq for WithId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.inner == other.inner
    }
}

impl<T: Eq> Eq for WithId<T> {}

impl<T: PartialOrd> PartialOrd for WithId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord> Ord for WithId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: Default> Default for WithId<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Debug> Debug for WithId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithId")
            .field("id", &self.id)
            .field("inner", &self.inner)
            .finish()
    }
}
