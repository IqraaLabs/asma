use crate::net_element::NetElement;

/// # Task
pub struct Task {
    id: u64,
    name: String,
    description: String,
}


impl NetElement for Task {
    fn get_id(&self) -> u64 {
        self.id
    }
}
