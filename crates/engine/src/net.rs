use crate::condition::Condition;
use crate::flow::Flow;

/// # Net
/// an object of this class contains a set of interconnected tasks (action elements) and conditions.
/// In addition to the parameter definitions inherited from its superclass (YDecomposition) YNet
/// allows local variables to be defined
pub struct Net {
    tasks: Vec<Condition>,
    conditions: Vec<Condition>,
    flows: Vec<Flow>,
}
