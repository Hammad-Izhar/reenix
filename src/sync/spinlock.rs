use core::{cell::UnsafeCell, sync::atomic::AtomicBool};

pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        while self
            .locked
            .swap(true, core::sync::atomic::Ordering::Acquire)
        {
            core::hint::spin_loop();
        }

        SpinlockGuard { lock: self }
    }
    pub fn unlock(&self) {}
}

unsafe impl<T: Send> Sync for Spinlock<T> where T: Send {}

impl<T> core::ops::Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .locked
            .store(false, core::sync::atomic::Ordering::Release);
    }
}

unsafe impl<T> Send for SpinlockGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for SpinlockGuard<'_, T> where T: Sync {}
