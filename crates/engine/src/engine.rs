use crate::net_runner::NetRunner;

/// # Engine
/// the engine is responsible for storing all the active process specifications in object format. It
/// is also a single control point for all operations over running process instances, e.g. launching cases, starting
// work-items (check-out), completing work-items (check-in), cancelling cases etc. It delegates some of these
// process instance controlling operations to some NetRunner, however the engine stores
// and aggregates each NetRunner instance and correlates it with the YIdentifier object running through it.
struct Engine {
    net_runners: Vec<NetRunner>,
}

impl Engine {
    pub fn create_net_runner(&mut self) {
        let net_runner = NetRunner::new();
        self.net_runners.push(net_runner);
    }
}
