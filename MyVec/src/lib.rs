
#![feature(ptr_internals, alloc, heap_api)]

use std::ptr::{Unique, NonNull, self};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::alloc::{Alloc, GlobalAlloc, Layout, Global, handle_alloc_error}
use std::io::set_panic;

struct  MyVec<T> {
    buf: RawVec<T>,
    len: usize
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            ::std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

impl<T> MyVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "还没准备好处理零尺寸类型" );
        MyVec{ buf: RawVec::new(), len: 0}
    }

    fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }
    fn cap(&self) -> usize { self.buf.cap }

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
        if self.len == self.cap { self.buf.grow(); }
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
                            self.len - index);
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
            ptr::copy(self.ptr().offset(index as isize + 1),
                      self.ptr().offset(index as isize),
                      self.len - index);
            result
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        unsafe  {
            let iter= RawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);

            IntoIter {
                iter: iter,
                _buf: buf,
            }
        }
    }

    pub fn drain(&mut self) ->Drain<T> {
        unsafe  {
            let iter = RawValIter::new(&self);
            self.len = 0;

            Drain {
                iter: iter,
                vec: PhantomData,
            }
        }
    }
}

struct  RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                (( slice.as_ptr() as usize ) + slice.len()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().offset(slice.len() as isize)
            }
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = if mem::size_of::<T>() == 0 {
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len = (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = if mem::size_of::<T>() == 0 {
                    ( self.end as usize -1 ) as *const _
                } else {
                    self.end.offset(-1)
                };
                Some(ptr::read(self.end))
            }
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}


pub struct  Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T > {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut self.iter {}
    }
}



impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
        }
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

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}

            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_type = elem_size * self.cap;
            unsafe {
                heap::deallocate(self.buf.as_ptr() as *mut _, num_type, align);
            }
        }
    }
}

struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize
}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() == 0, "TODO:实现零尺寸类型的支持");
        RawVec { ptr: Unique::empty(), cap: 0}
    }

    fn grow(&mut self) {
        unsafe {
            let aligen = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap = 0 {
                let ptr = heap::allocate(elem_size, aligen);
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
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let num_bytes = elem_size * self.cap;
            unsafe {
                heap::deallocate(self.ptr.as_mut() as *mut _, num_bytes, align);
            }
        }
    }
}



impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {

    }
}
