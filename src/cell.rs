use std::cell::UnsafeCell;
// Cell to allow mutate it because there no one thread which have references to `val`
// and you never given out ref into the value you store  and you can replace just fine
pub struct Cell<T> {
    val: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(val: T) -> Self {
        Cell {
            val: UnsafeCell::new(val),
        }
    }

    pub fn set(&self, val: T)
    where
        T: Copy,
    {
        unsafe { *self.val.get() = val };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.val.get() }
    }
}
