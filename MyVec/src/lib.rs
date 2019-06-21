
#![feature(ptr_internals, alloc, heap_api)]

use std::ptr::{Unique, self};
use std::mem;
use std::alloc::oom;
use std::io::set_panic;
use std::ops::{Deref, DerefMut};

struct  MyVec<T> {
    ptr: Unique<T>,
    len: usize,
    cap: usize,
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            ::std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> MyVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "还没准备好处理零尺寸类型" );
        MyVec{ ptr: Unique::empty(), cap: 0, len: 0}
    }

    fn grow(&mut self)  {
        unsafe {
            let align = mem::align_of::<T>();
            let type_size =  mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = heap::allocate(type_size, align);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * type_size;

                assert!(old_num_bytes <= (::std::isize::MAX as usize)/2, "caption over flow!");
                let new_num_bytes = old_num_bytes * 2;
                let ptr = heap::reallocate(self.ptr.as_ptr() as *mut _,
                                    old_num_bytes,
                                    new_num_bytes,
                                    align);
                (new_cap, ptr)
            };
            if ptr.is_null() { oom(); }
            self.ptr = Unique::new(ptr as *mut _);
            self.cap = new_cap;
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.cap { self.grow(); }
        unsafe {
            ptr::write(self.ptr.offset(self.len as isize), item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.offset(self.len as isize)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds!");
        if self.cap == self.len { self.grow(); }

        unsafe {
            if index < self.len {
                ptr::copy(self.ptr.offset(index as isize),
                    self.ptr.offset(index as isize + 1),
                            self.len - index
                );
            }
            ptr::write(self.ptr.offset(index as isize), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds!");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.offset(index as isize));
            ptr::copy(self.ptr.offset(index as isize + 1), self.ptr.offset(index as isize), self.len - index);
            result
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        let ptr = self.ptr;
        let cap = self.cap;
        let len = self.len;

        mem::forget(self);

        unsafe  {
            IntoIter {
                buf: ptr,
                cap: cap,
                start: *ptr,
                end: if cap == 0 {
                    *ptr
                } else {
                    ptr.offset(len as isize)
                }
            }
        }
    }
}


impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}

            let align = mem::align_of::<T>();
            let  elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe  {
                heap::deallocate(self.ptr.as_ptr() as *mut _, num_bytes, align);
            }
        }
    }
}

struct IntoIter<T> {
    buf: Unique<T>,
    cap: usize,
    start: *const T,
    end: *const T,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result  = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start  == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}