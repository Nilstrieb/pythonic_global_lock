#![doc = include_str!("../README.md")]

use parking_lot::lock_api::{MutexGuard, RawMutex};
use parking_lot::Mutex;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

static LOCK: Mutex<()> = Mutex::const_new(RawMutex::INIT, ());

/// A global lock that locks every `GLock` using the same global lock (like the GIL in cpython lmao)
pub struct GLock<T> {
    inner: UnsafeCell<T>,
}

impl<T> GLock<T> {
    /// Creates a new GLock
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Lock the global lock, not allowing any other `GLock` to be locked during the locked duration
    pub fn lock(&self) -> GLockGuard<T> {
        let global_guard = LOCK.lock();
        // SAFETY: We took the global lock above,
        // so no other one is allowed to get to this critical section here
        let value = unsafe { &mut *self.inner.get() };
        GLockGuard {
            value,
            _global_guard: global_guard,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

/// A guard that guards a globally locked value
pub struct GLockGuard<'a, T> {
    value: &'a mut T,
    _global_guard: MutexGuard<'a, parking_lot::RawMutex, ()>,
}

impl<'a, T> Deref for GLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for GLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
