use core::marker::PhantomData;

extern "C" {
    fn mutex_lock_c();
    fn mutex_unlock_c();
}

pub struct Mutex<T> {
    inner: T,
}

impl<T> Mutex<T> {
    pub fn init(t: T) -> Self {
        Mutex { inner: t }
    }

    pub fn acquire(&self) -> MutexGuard<'_, T> {
        unsafe { mutex_lock_c() };
        MutexGuard {
            inner: &self.inner as *const _ as *mut _,
            data: PhantomData,
        }
    }
}

pub struct MutexGuard<'a, T> {
    inner: *mut T,
    data: PhantomData<&'a T>,
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe { self.inner.as_mut() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { mutex_unlock_c() };
    }
}
