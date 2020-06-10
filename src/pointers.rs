extern "C" {
    fn pointers_main();
}

pub fn main() {
    unsafe {
        pointers_main();
    }
}
