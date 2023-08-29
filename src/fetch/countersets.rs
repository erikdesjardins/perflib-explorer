use crate::types::InstanceType;
use crate::winapi::{decode_utf16_until_null, invoke_with_buf};
use std::mem;
use windows::core::{Result, GUID};
use windows::Win32::System::Performance::{
    PerfEnumerateCounterSet, PerfQueryCounterSetRegistrationInfo, PERF_COUNTERSET_REG_INFO,
    PERF_REG_COUNTERSET_HELP_STRING, PERF_REG_COUNTERSET_NAME_STRING, PERF_REG_COUNTERSET_STRUCT,
};

pub fn all_ids() -> Result<Vec<GUID>> {
    let mut buf = Vec::new();
    let ids = invoke_with_buf(&mut buf, |buf, len| unsafe {
        PerfEnumerateCounterSet(None, Some(buf), len)
    })?;

    // Probably we can reuse `buf` here instead of cloning, but idk if the size estimate is always right,
    // so just do it anyways (it's not that big).
    let ids = ids.to_vec();

    Ok(ids)
}

pub fn name(buf: &mut Vec<u8>, id: &GUID) -> Result<String> {
    let name = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            id,
            PERF_REG_COUNTERSET_NAME_STRING,
            0,
            Some(buf),
            len,
        )
    })?;

    let name = decode_utf16_until_null(name);

    Ok(name)
}

pub fn help(buf: &mut Vec<u8>, id: &GUID) -> Result<String> {
    let name = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            id,
            PERF_REG_COUNTERSET_HELP_STRING,
            0,
            Some(buf),
            len,
        )
    })?;

    let name = decode_utf16_until_null(name);

    Ok(name)
}

pub fn instance_type(buf: &mut Vec<u8>, id: &GUID) -> Result<InstanceType> {
    let reg_info = reg_info(buf, id)?;
    let instance_type = InstanceType::from_bits(reg_info.InstanceType)?;
    Ok(instance_type)
}

fn reg_info(buf: &mut Vec<u8>, id: &GUID) -> Result<PERF_COUNTERSET_REG_INFO> {
    let buf = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(None, id, PERF_REG_COUNTERSET_STRUCT, 0, Some(buf), len)
    })?;

    assert!(buf.len() >= mem::size_of::<PERF_COUNTERSET_REG_INFO>());

    // SAFETY: PERF_COUNTERSET_REG_INFO is valid for all bit patterns (and buf is initialized) so this is safe even if it didn't write anything to the buffer.
    let buf = unsafe {
        buf.as_ptr()
            .cast::<PERF_COUNTERSET_REG_INFO>()
            .read_unaligned()
    };

    Ok(buf)
}
