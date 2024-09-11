use crate::net_element::NetElement;

/// # Identifier
/// represent the things that flow through the Nets indicating its process state.
/// If you are familiar with Petri nets then consider the references to objects of this class the tokens. Due to
/// the fact that composite tasks contain nets of their own the identifiers are capable of creating children. The
/// children pass through the subordinate nets. This idea is fully described in the YAWL Book
pub struct Identifier {
    id: u64,
    name: String,
    description: String,
}

impl NetElement for Identifier {
    fn get_id(&self) -> u64 {
        self.id
    }
}