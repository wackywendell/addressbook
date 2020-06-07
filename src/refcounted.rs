use crate::util::Rc;

pub struct Contact {
    pub name: String,
    pub address: String,
}

pub struct AddressBook {
    contacts: Vec<Rc<Contact>>,
}

// A reference-counted version of the address book. Contacts are kept in an
// array of reference-counted pointers, and can be shared between address books.
impl AddressBook {
    pub fn new() -> Self {
        return AddressBook {
            contacts: Vec::new(),
        };
    }

    pub fn add(&mut self, contact: Contact) -> Rc<Contact> {
        // This takes ownership of contact, copying it from the stack (contact)
        // into the heap, by creating a new reference-counted contact.
        // A reference-counted pointer to it is returned.
        let rc = Rc::new(contact);
        self.contacts.push(rc.clone());
        rc
    }

    pub fn add_shared(&mut self, contact: Rc<Contact>) {
        // This adds a reference to an already existing Contact to this address
        // book
        self.contacts.push(contact);
    }
}
