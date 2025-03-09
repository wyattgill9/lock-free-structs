use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub struct LockFreeStack<T> {
    top: AtomicPtr<Node<T>>,
    size: AtomicUsize,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            top: AtomicPtr::new(ptr::null_mut()),
            size: AtomicUsize::new(0),
        }
    }

    #[inline(always)]
    pub fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));

        loop {
            let top = self.top.load(Ordering::Acquire);
            unsafe { (*new_node).next = top };

            if self
                .top
                .compare_exchange(top, new_node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                self.size.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    #[inline(always)]
    pub fn pop(&self) -> Option<T> {
        loop {
            let top = self.top.load(Ordering::Acquire);
            if top.is_null() {
                return None;
            }

            let next = unsafe { (*top).next };

            if self
                .top
                .compare_exchange(top, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                let value = unsafe { ptr::read(&(*top).value) };

                unsafe { drop(Box::from_raw(top)) };

                self.size.fetch_sub(1, Ordering::Relaxed);
                return Some(value);
            }
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
