use crate::types::Instance;
use crate::winapi::{decode_utf16_until_null, invoke_with_buf};
use std::mem;
use std::slice;
use windows::core::{Result, GUID, HRESULT};
use windows::Win32::Foundation::ERROR_WMI_INSTANCE_NOT_FOUND;
use windows::Win32::System::Performance::{PerfEnumerateCounterSetInstances, PERF_INSTANCE_HEADER};

pub fn of_counterset(buf: &mut Vec<u8>, counterset_id: &GUID) -> Result<Option<Vec<Instance>>> {
    let res = invoke_with_buf(buf, |buf, len| {
        // Note: this might result in windows writing to the (unaligned) buffer, but it's a huge pain to deal with this API otherwise.
        let buf_len = buf.len().try_into().unwrap();
        let buf = buf.as_mut_ptr().cast::<PERF_INSTANCE_HEADER>();
        unsafe { PerfEnumerateCounterSetInstances(None, counterset_id, Some(buf), buf_len, len) }
    });

    let buf = match res {
        Ok(buf) => buf,
        // Some countersets don't have instances.
        Err(e) if e.code() == HRESULT::from(ERROR_WMI_INSTANCE_NOT_FOUND) => return Ok(None),
        Err(e) => return Err(e),
    };

    let mut instances = Vec::new();

    // SAFETY: Everything here and below depends on the Windows API being implemented as documented.
    // https://learn.microsoft.com/en-us/windows/win32/api/perflib/nf-perflib-perfenumeratecountersetinstances
    // https://learn.microsoft.com/en-us/windows/win32/api/perflib/ns-perflib-perf_instance_header

    let range = buf.as_ptr_range();

    let mut instance_ptr = range.start;

    while instance_ptr < range.end {
        // "Each PERF_INSTANCE_HEADER block consists of a PERF_INSTANCE_HEADER structure..."
        let instance = unsafe { instance_ptr.cast::<PERF_INSTANCE_HEADER>().read_unaligned() };

        let size = instance.Size.try_into().unwrap();

        // "...immediately followed by a null-terminated UTF-16LE instance name..."
        let name_ptr = unsafe {
            slice::from_raw_parts(
                instance_ptr.add(mem::size_of::<PERF_INSTANCE_HEADER>()),
                size,
            )
        };
        let name = decode_utf16_until_null(name_ptr);

        instances.push(Instance {
            id: instance.InstanceId,
            name,
        });

        // "...followed by padding so that the size of the PERF_INSTANCE_HEADER block is a multiple of 8 bytes."
        // Which is included in the size field:
        // "This total size is the sum of the sizes of the PERF_INSTANCE_HEADER structures, the string that contains the instance name, and the padding."
        instance_ptr = unsafe { instance_ptr.add(size) };
    }

    Ok(Some(instances))
}
