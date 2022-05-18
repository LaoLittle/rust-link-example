fn main() {
    unsafe {
        println!("Hello, world!");
        unsafe { test() }

        let a = get();
        println!("{}", a.is_none());
        set(131);
        let b = get();
        println!("{:?}", b);
    }
}

#[link(name = "lib")]
extern {
    fn test();

    fn get() -> Option<&'static i32>;

    fn set(i: i32);
}