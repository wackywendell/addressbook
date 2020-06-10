use crate::util;

pub struct Contact {
    pub name: String,
    pub address: String,
}

pub struct AddressBook {
    contacts: Vec<util::Rc<Contact>>,
}

// A reference-counted version of the address book. Contacts are kept in an
// array of reference-counted pointers, and can be shared between address books.
impl AddressBook {
    pub fn new() -> Self {
        return AddressBook {
            contacts: Vec::new(),
        };
    }

    pub fn add(&mut self, contact: Contact) -> util::Rc<Contact> {
        // This takes ownership of contact, copying it from the stack (contact)
        // into the heap, by creating a new reference-counted contact.
        // A reference-counted pointer to it is returned.
        let rc = util::Rc::new(contact);
        self.contacts.push(rc.clone());
        rc
    }

    pub fn add_shared(&mut self, contact: util::Rc<Contact>) {
        // This adds a reference to an already existing Contact to this address
        // book
        self.contacts.push(contact);
    }

    pub fn list(&self) -> &[util::Rc<Contact>] {
        &self.contacts[..]
    }
}

pub fn create_book() -> AddressBook {
    let mut book = AddressBook::new();

    let alice = Contact {
        name: "Alice".to_owned(),
        address: "123 Apple Ave".to_owned(),
    };

    // Add this contact to the book
    let alice_ref = book.add(alice);

    // Add another contact to the book
    let bob = Contact {
        name: "Bob".to_owned(),
        address: "123 Berry Boardwalk".to_owned(),
    };

    // Copy this contact into the array of contacts
    let _bob_ref = book.add(bob);

    // Add a third contact to the book
    let carol = Contact {
        name: "Carol".to_owned(),
        address: "123 Cherry Crossing".to_owned(),
    };

    // Copy this contact into the array of contacts
    let _carol_ref = book.add(carol);

    let mut book2 = AddressBook::new();

    // Add a reference-counted pointer to alice to book2, raising its reference
    // count to 2
    book2.add_shared(alice_ref);

    // Drop the first book. This will decrement the reference counts in alice,
    // bob, and carol, causing bob and carol to reach 0 and be deleted. alice is
    // still referenced by book2, so it will go from 2 references to 1 and not
    // be deleted
    drop(book);

    book2
}

pub fn main() {
    let book = create_book();

    for (i, contact) in book.list().iter().enumerate() {
        println!("{:2}: {:10} {:10}", i, contact.name, contact.address);
    }
}
