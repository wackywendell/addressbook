pub struct Contact {
    pub name: String,
    pub address: String,
}

pub struct AddressBook {
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
