use std::fmt::{self, Debug};
use windows::core::{Error, Result, GUID};
use windows::Win32::Foundation::{RPC_X_ENUM_VALUE_OUT_OF_RANGE, WIN32_ERROR};
use windows::Win32::System::Performance::{
    PERF_AGGREGATE_AVG, PERF_AGGREGATE_MAX, PERF_AGGREGATE_MIN, PERF_AGGREGATE_TOTAL,
    PERF_AGGREGATE_UNDEFINED, PERF_COUNTERSET_MULTI_INSTANCES, PERF_COUNTERSET_SINGLE_AGGREGATE,
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
    pub base_counter_id: Option<NonMaxU32>,
    pub multi_counter_id: Option<NonMaxU32>,
    pub aggregate_func: AggregateFunc,
}

#[derive(Debug)]
#[repr(u32)]
pub enum AggregateFunc {
    Undefined = PERF_AGGREGATE_UNDEFINED.0,
    Total = PERF_AGGREGATE_TOTAL.0,
    Avg = PERF_AGGREGATE_AVG.0,
    Min = PERF_AGGREGATE_MIN.0,
    Max = PERF_AGGREGATE_MAX,
}

impl AggregateFunc {
    pub fn from_bits(bits: PERF_COUNTER_AGGREGATE_FUNC) -> Result<Self> {
        const UNDEFINED: u32 = AggregateFunc::Undefined as _;
        const TOTAL: u32 = AggregateFunc::Total as _;
        const AVG: u32 = AggregateFunc::Avg as _;
        const MIN: u32 = AggregateFunc::Min as _;
        const MAX: u32 = AggregateFunc::Max as _;

        Ok(match bits.0 {
            UNDEFINED => Self::Undefined,
            TOTAL => Self::Total,
            AVG => Self::Avg,
            MIN => Self::Min,
            MAX => Self::Max,
            _ => return Err(Error::from(WIN32_ERROR(RPC_X_ENUM_VALUE_OUT_OF_RANGE as _))),
        })
    }
}

/// An instance of a counterset.
/// Not all countersets have instances.
/// Instances are generally things like "2.5GB Ethernet Adapter", and so are not fixed.
#[derive(Debug)]
pub struct Instance {
    pub id: u32,
    pub name: String,
}

pub struct NonMaxU32(u32);

impl Debug for NonMaxU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl NonMaxU32 {
    pub fn new(value: u32) -> Option<Self> {
        match value {
            u32::MAX => None,
            _ => Some(Self(value)),
        }
    }
}
