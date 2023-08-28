use crate::types::Counter;
use crate::winapi::{decode_utf16_until_null, invoke_with_buf};
use std::collections::HashMap;
use windows::core::{Result, GUID, HRESULT};
use windows::Win32::Foundation::ERROR_NOT_FOUND;
use windows::Win32::System::Performance::{
    PerfQueryCounterSetRegistrationInfo, PERF_COUNTERSET_REG_INFO, PERF_COUNTER_REG_INFO,
    PERF_REG_COUNTERSET_STRUCT, PERF_REG_COUNTER_HELP_STRINGS, PERF_REG_COUNTER_NAME_STRINGS,
    PERF_STRING_BUFFER_HEADER, PERF_STRING_COUNTER_HEADER,
};

pub fn of_counterset(buf: &mut Vec<u8>, counterset_id: &GUID) -> Result<Vec<Counter>> {
    let names = names_of_all_in_counterset(buf, counterset_id)?;
    let help = help_strings_of_all_in_counterset(buf, counterset_id)?;
    let reg_info = reg_info_of_all_in_counterset(buf, counterset_id)?;

    // Since there can be duplicate ids (and technically duplicate names, although I don't see that),
    // we can't just iterate over reg_info ids here--we need to iterate over names.
    let mut counters = names
        .into_iter()
        .map(|(id, name)| {
            // Help strings may not exist for all counters.
            let help = help.get(&id).cloned().unwrap_or_default();

            let reg_info = reg_info[&id];
            let base_counter_id = reg_info.BaseCounterId;
            let multi_counter_id = reg_info.MultiId;
            let aggregate_func = reg_info.AggregateFunc;

            Counter {
                id,
                name,
                help,
                base_counter_id,
                multi_counter_id,
                aggregate_func,
            }
        })
        .collect::<Vec<_>>();

    counters.sort_by_key(|c| c.id);

    Ok(counters)
}

fn names_of_all_in_counterset(
    buf: &mut Vec<u8>,
    counterset_id: &GUID,
) -> Result<HashMap<u32, String>> {
    let buf = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            counterset_id,
            PERF_REG_COUNTER_NAME_STRINGS,
            0,
            Some(buf),
            len,
        )
    })?;

    // SAFETY: buf has the required layout, as documented for PERF_REG_COUNTER_NAME_STRINGS.
    let names = unsafe { parse_counter_strings(buf) };

    Ok(names)
}

fn help_strings_of_all_in_counterset(
    buf: &mut Vec<u8>,
    counterset_id: &GUID,
) -> Result<HashMap<u32, String>> {
    let res = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            counterset_id,
            PERF_REG_COUNTER_HELP_STRINGS,
            0,
            Some(buf),
            len,
        )
    });

    let buf = match res {
        Ok(buf) => buf,
        Err(e) if e.code() == HRESULT::from(ERROR_NOT_FOUND) => {
            return Ok(HashMap::new());
        }
        Err(e) => return Err(e),
    };

    // SAFETY: buf has the required layout, as documented for PERF_REG_COUNTER_HELP_STRINGS.
    let names = unsafe { parse_counter_strings(buf) };

    Ok(names)
}

/// Parse a buffer of counter strings into a map by counter id.
///
/// # SAFETY
///
/// `buf` must contain a `PERF_STRING_BUFFER_HEADER` structure,
/// followed by one or more `PERF_STRING_COUNTER_HEADER` structures,
/// followed by string data that indicates the counter names.
/// As documented on https://learn.microsoft.com/en-us/windows/win32/api/perflib/ne-perflib-perfreginfotype.
unsafe fn parse_counter_strings(buf: &[u8]) -> HashMap<u32, String> {
    // SAFETY: Everything here and below depends on the Windows API being implemented as documented.
    // https://learn.microsoft.com/en-us/windows/win32/api/perflib/ne-perflib-perfreginfotype

    // "The block includes a PERF_STRING_BUFFER_HEADER structure..."
    let header_ptr = buf.as_ptr().cast::<PERF_STRING_BUFFER_HEADER>();
    let header = unsafe { header_ptr.read_unaligned() };

    let num_counters = header.dwCounters.try_into().unwrap();
    let mut strings = HashMap::with_capacity(num_counters);

    // "...followed by one or more PERF_STRING_COUNTER_HEADER structures..."
    let first_string_ptr = unsafe { header_ptr.add(1).cast::<PERF_STRING_COUNTER_HEADER>() };

    for i in 0..num_counters {
        let string_ptr = unsafe { first_string_ptr.add(i) };
        let string = unsafe { string_ptr.read_unaligned() };

        let name = match string.dwOffset {
            0xFFFFFFFF => String::new(),
            offset_from_start_of_buf => {
                // "...followed by string data that indicates the counter names."
                let name_buf = &buf[offset_from_start_of_buf as usize..];
                decode_utf16_until_null(name_buf)
            }
        };

        strings.insert(string.dwCounterId, name);
    }

    strings
}

fn reg_info_of_all_in_counterset(
    buf: &mut Vec<u8>,
    counterset_id: &GUID,
) -> Result<HashMap<u32, PERF_COUNTER_REG_INFO>> {
    let buf = invoke_with_buf(buf, |buf, len| unsafe {
        PerfQueryCounterSetRegistrationInfo(
            None,
            counterset_id,
            PERF_REG_COUNTERSET_STRUCT,
            0,
            Some(buf),
            len,
        )
    })?;

    // SAFETY: Everything here and below depends on the Windows API being implemented as documented.
    // https://learn.microsoft.com/en-us/windows/win32/api/perflib/ne-perflib-perfreginfotype

    // "The block includes a PERF_COUNTERSET_REG_INFO structure..."
    let header_ptr = buf.as_ptr().cast::<PERF_COUNTERSET_REG_INFO>();
    let header = unsafe { header_ptr.read_unaligned() };

    let num_counters = header.NumCounters.try_into().unwrap();
    let mut reg_info = HashMap::with_capacity(num_counters);

    // "...followed by one or more PERF_COUNTER_REG_INFO structures."
    let first_info_ptr = unsafe { header_ptr.add(1).cast::<PERF_COUNTER_REG_INFO>() };

    for i in 0..num_counters {
        let info_ptr = unsafe { first_info_ptr.add(i) };
        let info = unsafe { info_ptr.read_unaligned() };

        reg_info.insert(info.CounterId, info);
    }

    Ok(reg_info)
}
