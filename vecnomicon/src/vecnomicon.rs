use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

pub struct VecNomicon<T> {
    buf: RawMiconVec<T>,
    len: usize,
}

pub struct IntoIter<T> {
    buf: RawMiconVec<T>,
    start: *const T,
    end: *const T,
}

struct RawMiconVec<T> {
    data: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T> Send for RawMiconVec<T> where T: Send {}
unsafe impl<T> Sync for RawMiconVec<T> where T: Sync {}
unsafe impl<T> Send for VecNomicon<T> where T: Send {}
unsafe impl<T> Sync for VecNomicon<T> where T: Sync {}

impl<T> RawMiconVec<T> {
    fn new() -> Self {
        assert_ne!(0, size_of::<T>(), "We're not read to handle ZST's");
        Self {
            cap: 0,
            data: NonNull::dangling(),
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        assert!(new_layout.size() <= usize::MAX, "allocation is so large");
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.data.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.data = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawMiconVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.data.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> VecNomicon<T> {
    pub fn new() -> Self {
        assert_ne!(0, size_of::<T>(), "We're not read to handle ZST's");
        Self {
            len: 0,
            buf: RawMiconVec::new(),
        }
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    fn data(&self) -> *mut T {
        self.buf.data.as_ptr()
    }

    fn grow(&mut self) {
        self.buf.grow()
    }

    pub fn push(&mut self, elemen: T) {
        if self.len == self.cap() {
            self.grow();
        }
        unsafe {
            ptr::write(self.data().add(self.len), elemen); //write data for the pointer
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.data().add(self.len))) } //read the value for safe
        }
    }

    pub fn insert(&mut self, index: usize, elemen: T) {
        assert!(index <= self.len, "index out of bounds");
        unsafe {
            ptr::copy(
                self.data().add(index),     //copy from the index
                self.data().add(index + 1), //copy dest
                self.len - index,           //quantity
            );
            ptr::write(self.data().add(index), elemen);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index <= self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result: T = ptr::read(self.data().add(index));
            ptr::copy(
                self.data().add(index + 1),
                self.data().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let buf = ptr::read(&self.buf);
            let cap = self.cap();
            let len = self.len;

            std::mem::forget(self);

            IntoIter {
                start: buf.data.as_ptr(),
                end: if cap == 0 {
                    buf.data.as_ptr()
                } else {
                    buf.data.as_ptr().add(len)
                },
                buf: buf,
            }
        }
    }
}

impl<T> Deref for VecNomicon<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data(), self.len) }
    }
}

impl<T> DerefMut for VecNomicon<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data(), self.len) }
    }
}

impl<T> Drop for VecNomicon<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / std::mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // drop any remaining elements
        for _ in &mut *self {}
    }
}
