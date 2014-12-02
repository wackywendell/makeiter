extern crate makeiter;

use makeiter::FuncIter;

pub fn main() {
    println!("{}", FuncIter::new(|| Some(3u)).next());
}
