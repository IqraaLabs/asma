use crate::net_element::NetElement;

/// # Condition Type
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConditionType {
    Intermediate,
    Input,
    Output,
}

/// # Condition
/// a condition is like a place in Petri nets. It is a sibling member to the tasks inside any net. They
/// typically sit between tasks, and store the process state ‘identifiers’ (‘tokens’ in Petri speak). When two tasks
/// are directly connected together, an “invisible” condition is created between the tasks, when the
/// process is loaded into the engine
pub struct Condition {
    id: u32,
    name: String,
    description: Option<String>,
    condition_type: ConditionType,
}


impl NetElement for Condition {
    fn id(&self) -> u32 {
        self.id
    }
}


impl Condition {
    fn new(id: u32, name: String, description: Option<String>, condition_type: ConditionType) -> Self {
        Condition {
            id,
            name,
            description,
            condition_type,
        }
    }
}
