use crate::winapi::{decode_utf16_until_null, invoke_with_buf};
use std::mem;
use windows::core::{Result, GUID};
use windows::Win32::System::Performance::{
    PerfQueryCounterSetRegistrationInfo, PERF_REG_PROVIDER_GUID, PERF_REG_PROVIDER_NAME,
};

pub fn id_from_counterset(buf: &mut Vec<u8>, counterset_id: &GUID) -> Result<GUID> {
    let guid = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            counterset_id,
            PERF_REG_PROVIDER_GUID,
            0,
            Some(buf),
            len,
        )
    })?;

    assert_eq!(guid.len(), mem::size_of::<GUID>());

    // SAFETY: GUID is valid for all bit patterns (and buf is initialized) so this is safe even if it didn't write anything to the buffer.
    let guid = unsafe { guid.as_ptr().cast::<GUID>().read_unaligned() };

    Ok(guid)
}

pub fn name_from_counterset(buf: &mut Vec<u8>, counterset_id: &GUID) -> Result<String> {
    let name = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            counterset_id,
            PERF_REG_PROVIDER_NAME,
            0,
            Some(buf),
            len,
        )
    })?;

    let name = decode_utf16_until_null(name);

    Ok(name)
}
