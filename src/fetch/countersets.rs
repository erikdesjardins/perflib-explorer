use crate::winapi::{decode_utf16_until_null, invoke_with_buf};
use windows::core::{Result, GUID};
use windows::Win32::System::Performance::{
    PerfEnumerateCounterSet, PerfQueryCounterSetRegistrationInfo, PERF_REG_COUNTERSET_NAME_STRING,
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
