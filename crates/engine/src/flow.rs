use task::Task;

/// # Flow
/// represent the arcs that join tasks to conditions in a process model. They refer to
/// both their preceding task/condition and to their succeeding task/condition. Likewise, each task/condition
/// is “aware” of the flows preceding it, and the flows succeeding it. Therefore, tasks/conditions, and their flows
/// form up a bidirectional linked list, of sorts (except it’s more of a graph than a list, but you get the idea)
pub struct Flow {
    previous: Task,
    next: Task,
}
