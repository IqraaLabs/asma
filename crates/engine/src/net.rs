use crate::condition::Condition;
use crate::flow::Flow;
use crate::task::Task;

/// an object of this class contains a set of interconnected tasks (action elements) and conditions.
/// In addition to the parameter definitions inherited from its superclass (YDecomposition) YNet
/// allows local variables to be defined
pub struct Net {
    input_condition: Condition,
    output_condition: Condition,
    tasks: Vec<Task>,
    conditions: Vec<Condition>,
    flows: Vec<Flow>,
}


impl Net {
    pub fn new(input_condition: Condition, output_condition: Condition) -> Self {
        Net {
            tasks: vec![],
            conditions: vec![],
            flows: vec![],
            input_condition,
            output_condition
        }
    }
}
