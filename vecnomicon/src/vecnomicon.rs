use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::size_of,
    ptr::NonNull,
};

pub struct VecNomicon<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T> Send for VecNomicon<T> where T: Send {}
unsafe impl<T> Sync for VecNomicon<T> where T: Sync {}

impl<T> VecNomicon<T> {
    pub fn new() -> Self {
        assert_eq!(0, size_of::<T>(), "We're not read to handle ZST's");
        Self {
            len: 0,
            cap: 0,
            _marker: PhantomData::default(),
            data: NonNull::dangling(),
        }
    }

    fn grow(&mut self) {
        let zero_cap = self.cap == 0;
        let (new_cap, new_layout) = if zero_cap {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 1 * self.cap;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "allocation is too large"
        );
        let new_ptr = if zero_cap {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.data.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        self.data = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap
    }
}
