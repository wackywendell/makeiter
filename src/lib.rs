#![feature(unboxed_closures)]

/// Holds a closure for iteration
pub struct FuncIter<F> {
    f: F,
}

// `F` is the type of the unboxed closure
// Is an unboxed closure because `F` implements the `FnOnce` trait
impl<F, R> FuncIter<F> where F: FnMut() -> R {
    pub fn new(f: F) -> FuncIter<F> {
        FuncIter {
            f: f,
        }
    }
}

impl<F, A> Iterator<A> for FuncIter<F> where F: FnMut() -> Option<A> {
    fn next(&mut self) -> Option<A> {
        self.f.call_mut(())
    }
}

pub fn make_iter<F, A>(f : F) -> FuncIter<F> where F: FnMut() -> Option<A> {
    FuncIter::new(f)
}

#[test]
fn base_test() {
    assert_eq!(FuncIter::new(|| Some(3u)).next(), Some(3u));
    
    let val : Option<()> = None;
    assert_eq!(FuncIter::new(|| val).next(), None);
    
    let i = 4u;
    assert_eq!(FuncIter::new(|| Some(i)).next(), Some(i));
    
    let mut i = 4u;
    let mut it = FuncIter::new(|| {
        if i <= 0 {None}
        else {i -= 1; Some(i)}
    });
    
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}

#[test]
fn make_iter_test() {
    let mut i = 4u;
    let mut it = make_iter(|| {
        if i <= 0 {None}
        else {i -= 1; Some(i)}
    });
    
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}
