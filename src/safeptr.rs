#[repr(transparent)]
pub struct ThreadSafePtr<T>(*mut T);

unsafe impl<T> Send for ThreadSafePtr<T> {}
unsafe impl<T> Sync for ThreadSafePtr<T> {}

impl<T> ThreadSafePtr<T> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        ThreadSafePtr(ptr)
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0
    }
}
