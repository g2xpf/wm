use std::cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct Rw<T> {
    inner: Rc<RefCell<T>>,
    modified: Rc<Cell<bool>>,
}

impl<T> Rw<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        // initialized means modified
        Rw {
            inner: Rc::new(RefCell::new(t)),
            modified: Rc::new(Cell::new(true)),
        }
    }

    // this API might not be needed
    #[inline]
    pub fn dry_modify(&self) {
        self.modified.set(true);
    }

    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.try_borrow_mut().expect("already borrowed")
    }

    #[inline]
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.inner.try_borrow()
    }

    #[inline]
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        match self.inner.try_borrow_mut() {
            ok @ Ok(_) => {
                self.modified.set(true);
                ok
            }
            err => err,
        }
    }

    #[inline]
    pub fn into_inner(self) -> Result<T, Self> {
        let is_modified = self.modified;
        match Rc::try_unwrap(self.inner) {
            Ok(t) => Ok(t.into_inner()),
            Err(inner) => Err(Self {
                inner,
                modified: is_modified,
            }),
        }
    }

    #[inline]
    pub fn is_modified(&self) -> bool {
        self.modified.take()
    }
}

#[derive(Clone)]
pub struct R<T> {
    inner: Rc<RefCell<T>>,
    modified: Rc<Cell<bool>>,
}

impl<T> R<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        R {
            inner: Rc::new(RefCell::new(t)),
            modified: Rc::new(Cell::new(true)),
        }
    }

    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match Rc::get_mut(&mut self.inner) {
            Some(inner) => {
                self.modified.set(true);
                Some(inner.get_mut())
            }
            None => None,
        }
    }

    // this API might not be needed
    #[inline]
    pub fn dry_modify(&self) {
        self.modified.set(true);
    }

    #[inline]
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.inner.try_borrow()
    }

    #[inline]
    pub fn into_inner(self) -> Result<T, Self> {
        let modified = self.modified;
        match Rc::try_unwrap(self.inner) {
            Ok(t) => Ok(t.into_inner()),
            Err(inner) => Err(Self { inner, modified }),
        }
    }

    #[inline]
    pub fn is_modified(&self) -> bool {
        self.modified.take()
    }
}

pub trait ToR<T> {
    fn to_r(&self) -> R<T>;
}

impl<T> ToR<T> for Rw<T> {
    fn to_r(&self) -> R<T> {
        R {
            inner: Rc::clone(&self.inner),
            modified: Rc::clone(&self.modified),
        }
    }
}

impl<T> ToR<T> for R<T> {
    fn to_r(&self) -> R<T> {
        R {
            inner: Rc::clone(&self.inner),
            modified: Rc::clone(&self.modified),
        }
    }
}
