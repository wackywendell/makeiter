extern crate makeiter;

use makeiter::{make_iter,FuncIter};

pub fn main() {
    println!("{}", FuncIter::new(|| Some(3u)).next());
    println!("{}", make_iter(|| Some(3u)).next());
}
