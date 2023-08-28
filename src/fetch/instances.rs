use crate::types::Instance;
use windows::core::{Result, GUID};

pub fn of_counterset(buf: &mut Vec<u8>, counterset_id: &GUID) -> Result<Option<Vec<Instance>>> {
    // TODO: implement
    Ok(None)
}
