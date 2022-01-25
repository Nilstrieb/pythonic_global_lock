use parking_lot::lock_api::{MutexGuard, RawMutex};
use parking_lot::Mutex;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

static LOCK: Mutex<()> = Mutex::const_new(RawMutex::INIT, ());

pub struct GLock<T> {
    inner: UnsafeCell<T>,
}

impl<T> GLock<T> {
    pub fn lock(&self) -> GLockGuard<T> {
        let mutex_guard = LOCK.lock();
        // SAFETY: We have the global lock
        let value = unsafe { &mut *self.inner.get() };
        GLockGuard { value, mutex_guard }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

struct GLockGuard<'a, T> {
    value: &'a mut T,
    mutex_guard: MutexGuard<'a, parking_lot::RawMutex, ()>,
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
