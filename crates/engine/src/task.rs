pub enum State {
    Enabled,
    Busy,
    Suspended,
    Suspending,
}

pub struct Task {
    id: u64,
    name: String,
}
