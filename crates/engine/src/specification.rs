use crate::net::Net;
use crate::version::Version;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::io;

/// # Specification
/// the outer container of a process model. Such an object is a process
/// specification/model. A process specification in YAWL basically contains a set of Nets (process nets)
/// TODO: write custom serde serializer and deserializer instead of using attributes
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Specification {
    /// unique identifier of a specification
    #[serde(rename = "@id")]
    pub id: String,

    /// specification version
    /// format: `major.minor`
    #[serde(rename = "@version")]
    pub version: Version,

    /// specification name
    #[serde(rename = "@name")]
    pub name: Option<String>,

    /// short text that describe, what does the specification do
    /// and what is the use case of the specification
    #[serde(rename = "@description")]
    pub description: Option<String>,

    pub root_net: Net,
}

impl Specification {
    pub fn new(id: String, version: Version, root_net: Net) -> Self {
        Self { id, version, name: None, description: None, root_net }
    }

    pub fn parse<R>(reader: R) -> Result<Specification, io::Error>
    where
        R: io::Read,
    {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let count = 0;
        let buf = Vec::new();
        todo!()
    }
}


#[test]
fn test_specification_parsing() {
    let spec = r#"
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


