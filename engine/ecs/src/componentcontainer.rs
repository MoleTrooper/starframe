use crate::storage::{ComponentStorage, CreateWithCapacity};
use crate::IdType;
use hibitset::BitSet;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub(crate) type WriteAccess<'a, T> = RwLockWriteGuard<'a, Box<dyn ComponentStorage<T>>>;
pub(crate) type ReadAccess<'a, T> = RwLockReadGuard<'a, Box<dyn ComponentStorage<T>>>;

/// A generic container for components that keeps track of users.
/// Space handles all the updates for you - none of this should be directly accessed by the user.
pub(crate) struct ComponentContainer<T: 'static> {
    users: BitSet,
    storage: RwLock<Box<dyn ComponentStorage<T>>>,
}

impl<T> ComponentContainer<T> {
    pub fn new<S>(capacity: IdType) -> Self
    where
        S: ComponentStorage<T> + CreateWithCapacity + 'static,
    {
        let new_container = ComponentContainer {
            storage: RwLock::new(Box::new(S::with_capacity(capacity))),
            users: BitSet::with_capacity(capacity as u32),
        };

        new_container
    }

    pub fn insert(&mut self, id: IdType, comp: T) {
        self.users.add(id as u32);
        self.storage.write().unwrap().insert(id, comp);
    }

    pub fn get_users(&self) -> &BitSet {
        &self.users
    }

    /// Get read access to the underlying storage.
    /// # Panics
    /// Panics if the storage is poisoned or the current thread already has a lock.
    pub fn read(&self) -> ReadAccess<'_, T> {
        self.storage.read().unwrap()
    }

    /// Get write access to the underlying storage.
    /// # Panics
    /// Panics if the storage is poisoned or the current thread already has a lock.
    pub fn write(&self) -> WriteAccess<'_, T> {
        self.storage.write().unwrap()
    }
}