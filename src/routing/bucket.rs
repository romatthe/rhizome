use routing::Contact;

#[derive(Debug)]
pub struct Bucket {
    contacts: Vec<Contact>
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket { contacts: vec![] }
    }

    pub fn add_contact(&self, contact: &Contact) {
        if self.contains(contact) {

        }
    }

    pub fn remove_contact(&mut self, contact: &Contact) {
        match self.contacts.iter().position(|c| c == contact) {
            Some(index)     => Some(self.contacts.remove(index)),
            None            => None
        };
    }

    pub fn contains(&self, contact: &Contact) -> bool {
        self.contacts.contains(contact)
    }
}