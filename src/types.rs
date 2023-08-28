use windows::core::GUID;

/// A provider of countersets.
/// Uniquely identified by its GUID, which appears to be fixed.
#[derive(Debug)]
pub struct Provider {
    pub id: GUID,
    pub name: String,
    pub countersets: Vec<CounterSet>,
}

/// A set of counters.
/// Uniquely identified by its GUID, which appears to be fixed.
/// Generally represents a category of something, like "Disk IO".
#[derive(Debug)]
pub struct CounterSet {
    pub id: GUID,
    pub name: String,
    pub help: String,
    pub counters: Vec<Counter>,
    pub instances: Option<Vec<Instance>>,
}

/// A counter in a counterset.
/// Uniquely identified by the combination of name and id. (I have seen duplicate ids in practice, but not duplicate names.)
/// Normally represents a category of something, like "Bytes Read", and seems to generally be fixed for a given counterset.
#[derive(Debug)]
pub struct Counter {
    pub id: u32,
    pub name: String,
    pub help: String,
}

/// An instance of a counterset.
/// Not all countersets have instances.
/// Instances are generally things like "2.5GB Ethernet Adapter", and so are not fixed.
#[derive(Debug)]
pub struct Instance {
    pub id: u32,
    pub name: String
}
