fn main() {
    println!("-- Owned Version --");
    addressbook::owned::main();

    println!("\n\n-- Reference-Counted Version --");
    addressbook::refcounted::main();

    println!("\n\n-- Garbage-Collected Version --");
    addressbook::garbagecollected::main();
}
