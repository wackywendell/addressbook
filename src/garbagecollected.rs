use core::borrow::BorrowMut;
use core::cell::RefCell;

use shredder::{Gc, Scan};

type Gcc<T> = Gc<RefCell<T>>;

#[derive(Scan)]
pub struct Contact {
    pub name: String,
    pub address: String,
    // relations: Vec<(String, Gcc<Contact>)>,
    relations: Vec<Gcc<Contact>>,
}

impl Contact {
    pub fn new(name: String, address: String) -> Self {
        Contact {
            name,
            address,
            relations: Vec::new(),
        }
    }

    pub fn add_relation(&mut self, relationship: String, contact: Gcc<Contact>) {
        // self.relations.push((relationship, contact));
        self.relations.push(contact);
    }

    pub fn relations(&self) -> &[Gcc<Contact>] {
        // &[(String, Gc<Contact>)] {
        &self.relations[..]
    }
}

pub struct AddressBook {
    contacts: Vec<Gcc<Contact>>,
}

// A garbage-collected version of the address book. Contacts are kept in an
// array of garbage-collected pointers, and can be shared between address books
// and reference each other.
impl AddressBook {
    pub fn new() -> Self {
        return AddressBook {
            contacts: Vec::new(),
        };
    }

    pub fn add(&mut self, contact: Contact) -> Gcc<Contact> {
        // This takes ownership of contact, copying it from the stack (contact)
        // into the heap, by creating a new garbage-collected contact.
        // A garbage-collected pointer to it is returned.
        let rc = Gcc::new(RefCell::new(contact));
        self.contacts.push(rc.clone());
        rc
    }

    pub fn add_shared(&mut self, contact: Gcc<Contact>) {
        // This adds a reference to an already existing Contact to this address
        // book
        self.contacts.push(contact);
    }

    pub fn list(&self) -> &[Gcc<Contact>] {
        &self.contacts[..]
    }
}

pub fn create_book() -> AddressBook {
    let mut book = AddressBook::new();

    let alice = Contact {
        name: "Alice".to_owned(),
        address: "123 Apple Ave".to_owned(),
        relations: Vec::new(),
    };

    // Add this contact to the book
    let alice_ref = book.add(alice);

    // Add another contact to the book
    let mut bob = Contact {
        name: "Bob".to_owned(),
        address: "123 Berry Boardwalk".to_owned(),
        relations: Vec::new(),
    };

    // Add a relation to alice
    // bob.relations.push(("sister".to_owned(), alice_ref.clone()));
    bob.borrow_mut().relations.push(alice_ref.clone());

    // Copy this contact into the array of contacts
    let _bob_ref = book.add(bob);

    // Add a third contact to the book
    let carol = Contact {
        name: "Carol".to_owned(),
        address: "123 Cherry Crossing".to_owned(),
        relations: Vec::new(),
    };

    // Copy this contact into the array of contacts
    let _carol_ref = book.add(carol);

    book
}

pub fn main() {
    let book = create_book();

    for (i, contact_ref) in book.list().iter().enumerate() {
        let contact = contact_ref.borrow();
        println!("{:2}: {:10} {:10}", i, contact.name, contact.address);
        // for (relationship, person) in contact.relations() {
        //     println!("    ==> {}: {}", relationship, person.name);
        for person in contact.relations() {
            println!("    ==> {}", person.borrow().name);
        }
    }
}
