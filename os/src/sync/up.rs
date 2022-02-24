
pub struct UPSafeCell<T>{
    inner:RefCell<T>,
}

// unsafe impl<T> Sync for UPSafeCell<T> { }

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self{ inner: RefCell::new(value) }
    }
    // '_是匿名生命周期标记 | RefMut可变借用
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner,borrow_mut()
    }
}