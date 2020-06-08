use gc::{Finalize, Gc, Trace};

#[derive(Trace, Finalize)]
pub struct Contact {
    pub name: String,
    pub address: String,
    relations: Vec<(String, Gc<Contact>)>,
}

impl Contact {
    pub fn new(name: String, address: String) -> Self {
        Contact {
            name,
            address,
            relations: Vec::new(),
        }
    }

    pub fn add_relation(&mut self, relationship: String, contact: Gc<Contact>) {
        self.relations.push((relationship, contact));
    }

    pub fn relations(&self) -> &[(String, Gc<Contact>)] {
        &self.relations[..]
    }
}

pub struct AddressBook {
    contacts: Vec<Gc<Contact>>,
}

// A reference-counted version of the address book. Contacts are kept in an
// array of reference-counted pointers, and can be shared between address books.
impl AddressBook {
    pub fn new() -> Self {
        return AddressBook {
            contacts: Vec::new(),
        };
    }

    pub fn add(&mut self, contact: Contact) -> Gc<Contact> {
        // This takes ownership of contact, copying it from the stack (contact)
        // into the heap, by creating a new reference-counted contact.
        // A reference-counted pointer to it is returned.
        let rc = Gc::new(contact);
        self.contacts.push(rc.clone());
        rc
    }

    pub fn add_shared(&mut self, contact: Gc<Contact>) {
        // This adds a reference to an already existing Contact to this address
        // book
        self.contacts.push(contact);
    }
}
