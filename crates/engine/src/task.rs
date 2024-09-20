use crate::net_element::NetElement;

/// # Task
pub struct Task {
    id: u32,
    name: String,
    description: Option<String>,
}

impl Task {

    /// create new task
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Task {
            id,
            name, 
            description
        }
    }

}


impl NetElement for Task {
    fn id(&self) -> u32 {
        self.id
    }
}
