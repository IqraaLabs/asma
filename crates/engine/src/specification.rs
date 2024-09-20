use crate::net::Net;
use crate::version::Version;

/// # Specification
/// the outer container of a process model. Such an object is a process
/// specification/model. A process specification in YAWL basically contains a set of Nets (process nets)
pub struct Specification {
    /// unique identifier of a specification
    pub id: String,

    /// specification version
    /// format: `major.minor`
    pub version: Version,

    /// specification name
    pub name: Option<String>,

    /// short text that describe, what does the specification do
    /// and what is the use case of the specification
    pub description: Option<String>,

    pub root_net: Net,
}

impl Specification {
    pub fn new(id: String, version: Version, root_net: Net) -> Self {
        Self { id, version, name: None, description: None, root_net }
    }
}


#[test]
fn test_specification_parsing() {
    let _ = r#"
    <Specification
        id="test-specification-id"
        name="test-specification-name"
        version="0.1"
        description="test-specification-description">
        <RootNet>
            <InputCondition id="input-condition">
                <FlowsInto ref="greet" default="true"/>
                <FlowsInto ref="greet-by-name">
                    <Predicate>false</Predicate>
                </FlowsInto>
            </InputCondition>
            <Task id="greet">
                <FlowsInto ref="output-condition"/>
            </Task>
            <Task id="greet-by-name">
                <FlowsInto ref="output-condition"/>
            </Task>
            <OutputCondition id="output-condition"/>
        </RootNet>
    </Specification>
    "#;
}


