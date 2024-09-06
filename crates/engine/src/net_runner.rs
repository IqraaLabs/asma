/// # Net Runner
/// Executes its Net instance. Each Net instance is essentially scheduled and controlled by
/// YNetRunner. When a specification gets loaded into the engine some instances of Net get
/// created. Collectively, these Net instances form the process template. When a process instance (case)
/// is launched, each Net instance gets deep-cloned by the engine, and the copy is then wrapped by the
/// NetRunner, which executes it
pub struct NetRunner {}
