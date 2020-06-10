fn main() {
    cc::Build::new().file("src/pointers.c").compile("pointers");
    println!("cargo:rerun-if-changed=src/pointers.c");
}
