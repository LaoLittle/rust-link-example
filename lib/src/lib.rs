#![feature(once_cell)]

use std::lazy::SyncOnceCell;

#[no_mangle]
fn test() {
    println!("xixi");
}

static A: SyncOnceCell<i32> = SyncOnceCell::new();

#[no_mangle]
fn set(i: i32) {
    A.set(i);
}

#[no_mangle]
fn get() -> Option<&'static i32> {
    A.get()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
