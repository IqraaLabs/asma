use crate::net_element::NetElement;
use crate::task::Task;

/// # Flow
/// represent the arcs that join tasks to conditions in a process model. They refer to
/// both their preceding task/condition and to their succeeding task/condition. Likewise, each task/condition
/// is “aware” of the flows preceding it, and the flows succeeding it. Therefore, tasks/conditions, and their flows
/// form up a bidirectional linked list, of sorts (except it’s more of a graph than a list, but you get the idea)
pub struct Flow {
    id: u64,
    name: String,
    description: String,
    previous: Task,
    next: Task,
}

impl NetElement for Flow {
    fn get_id(&self) -> u64 {
        self.id
    }
}
