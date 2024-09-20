use crate::net_element::NetElement;
use crate::task::Task;

/// # Flow
/// represent the arcs that join tasks to conditions in a process model. They refer to
/// both their preceding task/condition and to their succeeding task/condition. Likewise, each task/condition
/// is “aware” of the flows preceding it, and the flows succeeding it. Therefore, tasks/conditions, and their flows
/// form up a bidirectional linked list, of sorts (except it’s more of a graph than a list, but you get the idea)
pub struct Flow {
    id: u32,
    name: String,
    description: Option<String>,
    from: Task,
    to: Task,
}

impl Flow {

    /// create new flow instance
    /// ```rust
    /// # use engine::flow::Flow;
    /// # use engine::task::Task;
    /// # use engine::net_element::NetElement;
    /// let from_task: Task = Task::new(1, "task 1".to_string(), None);
    /// let to_task: Task = Task::new(2, "task 2".to_string(), None);
    /// let flow = Flow::new(1, "flow name".to_string(), None, from_task, to_task);
    /// assert_eq!(flow.id(), 1);
    /// ```
    pub fn new( id: u32, name: String, description: Option<String>, from: Task, to: Task) -> Self {
        Flow{ id, name, description, from, to }
    }
}

impl NetElement for Flow {
    fn id(&self) -> u32 {
        self.id
    }
}
