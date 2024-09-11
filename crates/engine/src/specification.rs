use crate::net::Net;
use crate::version::Version;

/// # Specification
/// the outer container of a process model. Such an object is a process
/// specification/model. A process specification in YAWL basically contains a set of Nets (process nets)
struct Specification {
    id: String,
    version: Version,
    description: String,
    root_net: Net,
}

impl Specification {}