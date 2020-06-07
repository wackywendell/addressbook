use std::alloc::{alloc, dealloc, Layout};
use std::ops::Deref;

// A reference-counted pointer.
//
// Rc<T> will generally live on the stack, but its value will be on the heap.
// When cloned, it keeps an internal count of how many references exist, and
// will deallocate the internal value when the last reference is dropped.
pub struct Rc<T> {
    ref_count: *mut usize,
    value: *const T,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        unsafe {
            // Let's allocate enough space on the heap for our value
            let value_ptr: *mut T = alloc(Layout::new::<T>()) as *mut T;
            debug_assert!(!value_ptr.is_null());
            // And now fill the value on the heap (value_ptr) with the one on the stack (value)
            value_ptr.write(value);

            // We need to do the same for our reference count, because the count
            // needs to be the same for all the references. It can't be on the
            // stack, because then we can't ensure that it only gets cleaned up
            // when the last Rc is cleaned up
            let ref_count_ptr: *mut usize = alloc(Layout::new::<usize>()) as *mut usize;
            debug_assert!(!value_ptr.is_null());
            ref_count_ptr.write(1);

            Rc {
                ref_count: ref_count_ptr,
                value: value_ptr,
            }
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.value.as_ref().unwrap() }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count += 1;
        }
        Rc {
            ref_count: self.ref_count,
            value: self.value,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        println!("Dropping!");
        let count = unsafe {
            *self.ref_count -= 1;
            *self.ref_count
        };
        if count > 0 {
            // There are other references still out there, so leave them be
            std::mem::forget(self.value);
            std::mem::forget(self.ref_count);
            self.value = std::ptr::null();
            self.ref_count = std::ptr::null::<*const usize>() as *mut usize;
            return;
        }

        // This was the last reference - let's clean it up
        unsafe {
            std::ptr::drop_in_place(self.value as *mut T);
            dealloc(self.value as *mut u8, Layout::new::<T>());
            dealloc(self.ref_count as *mut u8, Layout::new::<usize>());
            std::mem::forget(self.value);
            std::mem::forget(self.ref_count);
            self.value = std::ptr::null();
            self.ref_count = std::ptr::null::<*const usize>() as *mut usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::null;
    use core::sync::atomic::{AtomicIsize, Ordering};

    #[derive(Debug, Clone)]
    struct Value<'a> {
        pub x: i32,
        pub y: i64,
        drops: &'a AtomicIsize,
    }

    impl<'a> Drop for Value<'a> {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_basic() {
        let drops = AtomicIsize::new(0);
        let value = Value {
            x: 42,
            y: 800,
            drops: &drops,
        };

        let p1 = Rc::new(value.clone());

        // Pointers should not be null
        assert_ne!(p1.ref_count as *const u8, null());
        assert_ne!(p1.value as *const u8, null());
        unsafe {
            // Values on the heap should be as expected
            assert_eq!(*p1.ref_count, 1);
            assert_eq!((*p1.value).x, value.x);
            assert_eq!((*p1.value).y, value.y);
        }

        assert_eq!(drops.load(Ordering::SeqCst), 0);

        drop(p1);

        // Now that both references are gone, the value should be dropped
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_allocation() {
        let drops = AtomicIsize::new(0);
        let value = Value {
            x: 42,
            y: 800,
            drops: &drops,
        };

        let p1 = Rc::new(value.clone());

        // Pointers should not be null
        assert_ne!(p1.ref_count as *const u8, null());
        assert_ne!(p1.value as *const u8, null());
        unsafe {
            // Values on the heap should be as expected
            assert_eq!(*p1.ref_count, 1);
            assert_eq!((*p1.value).x, value.x);
            assert_eq!((*p1.value).y, value.y);
        }

        // Let's clone it, and check again
        let p2 = p1.clone();

        assert_eq!(p1.ref_count, p2.ref_count);
        unsafe {
            assert_eq!(*p1.ref_count, 2);
            assert_eq!(*p2.ref_count, 2);
        }

        drop(p2);
        // We should not have actually dropped the value yet
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        // Pointers should still not be null for p1
        assert_ne!(p1.ref_count as *const u8, null());
        assert_ne!(p1.value as *const u8, null());
        unsafe {
            // Values on the heap should be as expected
            assert_eq!(*p1.ref_count, 1);

            assert_eq!((*p1.value).x, value.x);
            assert_eq!((*p1.value).y, value.y);
        }

        drop(p1);

        // Now that both references are gone, the value should be dropped
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
