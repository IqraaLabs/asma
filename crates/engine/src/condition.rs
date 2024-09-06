/// # Condition
/// a condition is like a place in Petri nets. It is a sibling member to the tasks inside any net. They
/// typically sit between tasks, and store the process state ‘identifiers’ (‘tokens’ in Petri speak). When two tasks
/// are directly connected together, an “invisible” condition is created between the tasks, when the
/// process is loaded into the engine
pub struct Condition {}


pub enum Type {
    Standard,
    Input,
    Output,
}

