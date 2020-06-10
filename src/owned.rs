pub struct Contact {
    // The String type is on a reference to the heap, but let's ignore that for
    // now
    pub name: String,
    pub address: String,
}

pub struct AddressBook {
    // A Vec is a heap-allocated array. Internally, its a pointer to the array,
    // an int for its length, and an int for its capacity
    contacts: Vec<Contact>,
}

// An Owned version of the address book. Contacts are kept in an array on the
// heap, and deallocated when the AddressBook is deallocated.
impl AddressBook {
    pub fn new() -> Self {
        return AddressBook {
            contacts: Vec::new(),
        };
    }

    pub fn add(&mut self, contact: Contact) {
        // This takes ownership of contact, copying it from the stack (contact)
        // into the heap (self.contacts)
        self.contacts.push(contact);
    }

    pub fn list(&self) -> &[Contact] {
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
    book.add(alice);

    // Add another contact to the book
    let bob = Contact {
        name: "Bob".to_owned(),
        address: "123 Berry Boardwalk".to_owned(),
    };

    // Copy this contact into the array of contacts
    book.add(bob);

    // Add a third contact to the book
    let carol = Contact {
        name: "Carol".to_owned(),
        address: "123 Cherry Crossing".to_owned(),
    };

    // Copy this contact into the array of contacts
    book.add(carol);

    book
}

pub fn main() {
    let book = create_book();

    for (i, contact) in book.list().iter().enumerate() {
        println!("{:2}: {:10} {:10}", i, contact.name, contact.address);
    }
}
