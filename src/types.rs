use windows::core::{Error, Result, GUID};
use windows::Win32::Foundation::{RPC_X_ENUM_VALUE_OUT_OF_RANGE, WIN32_ERROR};
use windows::Win32::System::Performance::{
    PERF_COUNTERSET_MULTI_INSTANCES, PERF_COUNTERSET_SINGLE_AGGREGATE,
    PERF_COUNTERSET_SINGLE_INSTANCE, PERF_COUNTER_AGGREGATE_FUNC,
};

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
    pub instance_type: InstanceType,
    pub counters: Vec<Counter>,
    pub instances: Option<Vec<Instance>>,
}

#[derive(Debug)]
#[repr(u32)]
pub enum InstanceType {
    SingleInstance = PERF_COUNTERSET_SINGLE_INSTANCE,
    MultiInstances = PERF_COUNTERSET_MULTI_INSTANCES,
    SingleAggregate = PERF_COUNTERSET_SINGLE_AGGREGATE,
    MultiAggregate = PERF_COUNTERSET_MULTI_INSTANCES | PERF_COUNTERSET_SINGLE_AGGREGATE,
}

impl InstanceType {
    pub fn from_bits(bits: u32) -> Result<Self> {
        const SINGLE_INSTANCE: u32 = InstanceType::SingleInstance as _;
        const MULTI_INSTANCES: u32 = InstanceType::MultiInstances as _;
        const SINGLE_AGGREGATE: u32 = InstanceType::SingleAggregate as _;
        const MULTI_AGGREGATE: u32 = InstanceType::MultiAggregate as _;

        Ok(match bits {
            SINGLE_INSTANCE => Self::SingleInstance,
            MULTI_INSTANCES => Self::MultiInstances,
            SINGLE_AGGREGATE => Self::SingleAggregate,
            MULTI_AGGREGATE => Self::MultiAggregate,
            _ => return Err(Error::from(WIN32_ERROR(RPC_X_ENUM_VALUE_OUT_OF_RANGE as _))),
        })
    }
}

/// A counter in a counterset.
/// Uniquely identified by the combination of name and id. (I have seen duplicate ids in practice, but not duplicate names.)
/// Normally represents a category of something, like "Bytes Read", and seems to generally be fixed for a given counterset.
#[derive(Debug)]
pub struct Counter {
    pub id: u32,
    pub name: String,
    pub help: String,
    pub base_counter_id: u32,
    pub multi_counter_id: u32,
    pub aggregate_func: PERF_COUNTER_AGGREGATE_FUNC,
}

/// An instance of a counterset.
/// Not all countersets have instances.
/// Instances are generally things like "2.5GB Ethernet Adapter", and so are not fixed.
#[derive(Debug)]
pub struct Instance {
    pub id: u32,
    pub name: String,
}
