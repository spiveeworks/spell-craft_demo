use std::rc;
use std::cell;
use std::ops;
use std::mem;

pub struct Owned<T> (rc::Rc<cell::RefCell<T>>);
pub struct Link<T> (rc::Weak<cell::RefCell<T>>);

// These types are a mess...
// raw pointer to keep the reference count alive
//   while an unbounded Ref exists.
// but since we need to manually drop them in order,
//   Option the Ref so that we can do the Option dance during Drop.
// so basically never use the strong pointer,
//   and never empty the Option.
pub struct Ref<'a, T: 'a> {
    strong: *const cell::RefCell<T>,
    borrow: Option<cell::Ref<'a, T>>,
}

pub struct RefMut<'a, T: 'a> {
    strong: *const cell::RefCell<T>,
    borrow: Option<cell::RefMut<'a, T>>,
}

pub enum BorrowError {
    Missing,
    Busy(cell::BorrowError),
}

pub enum BorrowMutError {
    Missing,
    Busy(cell::BorrowMutError),
}


impl From<cell::BorrowError> for BorrowError {
    fn from(value: cell::BorrowError) -> Self {
        BorrowError::Busy(value)
    }
}

impl From<cell::BorrowMutError> for BorrowMutError {
    fn from(value: cell::BorrowMutError) -> Self {
        BorrowMutError::Busy(value)
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Link(self.0.clone())
    }
}

impl<T> Owned<T> {
    pub fn new(value: T) -> Self {
        // where's point-free when you need it
        Owned(rc::Rc::new(cell::RefCell::new(value)))
    }

    pub fn share(&self) -> Link<T> {
        Link(rc::Rc::downgrade(&self.0))
    }

    pub fn ptr_eq(&self, other: &Ref<T>) -> bool {
        unsafe {
            // use the Rc implementation of ptr_eq
            let other = rc::Rc::from_raw(other.strong);
            let result = rc::Rc::ptr_eq(&self.0, &other);
            // don't drop the Rc, that's the Ref's job
            mem::forget(other);
            result
        }
    }
}

impl<T> Link<T> {
    // create an empty link, useful when initalizing cycles
    pub fn new() -> Self {
        Link(rc::Weak::new())
    }


    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        let strong = self.0
                         .upgrade()
                         .ok_or(BorrowError::Missing)?;
        // creating this structure early will undo the strong counter
        // if the try_borrow fails
        let mut result = Ref {
            strong: rc::Rc::into_raw(strong),
            borrow: None,
        };
        unsafe {
            let ref_cell = &*result.strong;
            result.borrow = Some(ref_cell.try_borrow()?);
        }
        Ok(result)
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        let strong = self.0
                         .upgrade()
                         .ok_or(BorrowMutError::Missing)?;
        // see above
        let mut result = RefMut {
            strong: rc::Rc::into_raw(strong),
            borrow: None,
        };
        unsafe {
            let ref_cell = &*result.strong;
            result.borrow = Some(ref_cell.try_borrow_mut()?);
        }
        Ok(result)
    }
}

impl<'a, T: 'a> ops::Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.borrow
              .as_ref()
              .expect("Empty Ref not dropped.")
    }
}

impl<'a, T: 'a> ops::Deref for RefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.borrow
              .as_ref()
              .expect("Empty RefMut not dropped.")
    }
}

impl<'a, T: 'a> ops::DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.borrow
                  .as_mut()
                  .expect("Empty RefMut not dropped.")
    }
}



// These drop methods are safe,
// we can drop the Rc because it is private
// we have to drop the borrow first though, to uphold other contracts
impl<'a, T: 'a> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        unsafe {
            mem::drop(self.borrow.take());
            mem::drop(rc::Rc::from_raw(self.strong));
        }
    }
}

impl<'a, T: 'a> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        unsafe {
            mem::drop(self.borrow.take());
            mem::drop(rc::Rc::from_raw(self.strong));
        }
    }
}


