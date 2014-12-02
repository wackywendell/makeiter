#![feature(unboxed_closures)]

/// Holds a closure for iteration
pub struct FuncIter<F> {
    f: F,
}

impl<F, R> FuncIter<F> where F: FnMut() -> R {
    pub fn new(f: F) -> FuncIter<F> {
        FuncIter {
            f: f,
        }
    }
}

#[test]
fn base_test() {
    let val : Option<()> = None;
    let _ = FuncIter::new(|| val);
}
