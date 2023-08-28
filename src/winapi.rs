use windows::core::{Error, Result};
use windows::Win32::Foundation::{ERROR_NOT_ENOUGH_MEMORY, ERROR_SUCCESS, WIN32_ERROR};

/// Call a windows perflib function with a buffer.
///
/// Will invoke the function first with the provided buffer;
/// and if it's too small (signaled by returning ERROR_NOT_ENOUGH_MEMORY),
/// it will resize the buffer to the required size and try again.
///
/// Returns the segment of the buffer containing the resulting data.
pub fn invoke_with_buf<T>(buf: &mut Vec<T>, f: impl Fn(&mut [T], &mut u32) -> u32) -> Result<&[T]> where T: Default {
    let mut actual = 0;

    let res = WIN32_ERROR(f(buf, &mut actual));

    match res {
        ERROR_SUCCESS => return Ok(&buf[..actual.try_into().unwrap()]),
        ERROR_NOT_ENOUGH_MEMORY => {
            buf.resize_with(actual.try_into().unwrap(), T::default);
        }
        _ => return Err(Error::from(res)),
    }

    let res = WIN32_ERROR(f(buf, &mut actual));

    match res {
        ERROR_SUCCESS => return Ok(&buf[..actual.try_into().unwrap()]),
        _ => return Err(Error::from(res)),
    }
}

pub fn decode_utf16_until_null(buf: &[u8]) -> String {
    let pairs_until_null = buf
        .chunks_exact(2)
        .map(|c| u16::from_ne_bytes([c[0], c[1]]))
        .take_while(|c| *c != 0);

    char::decode_utf16(pairs_until_null)
        .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<String>()
}
