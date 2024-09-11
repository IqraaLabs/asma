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
    id: u64,
    name: Option<String>,
    description: Option<String>,
    condition_type: ConditionType,
}


impl NetElement for Condition {
    fn get_id(&self) -> u64 {
        self.id
    }
}


impl Condition {
    fn new(id: u64, name: &str, description: &str, condition_type: ConditionType) -> Self {
        Condition {
            id,
            name: Some(name.to_string()),
            description: Some(description.to_string()),
            condition_type,
        }
    }
}

#[test]
fn create_condition() {
    let condition = Condition::new(1, "name", "description", ConditionType::Input);
    assert_eq!(condition.get_id(), 1);
}