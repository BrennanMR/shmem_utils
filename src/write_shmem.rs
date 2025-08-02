#![allow(warnings)]

use shared_memory::{Shmem, ShmemConf};
use crate::init_shmem;

/// Writes to shmem slice
pub fn write_shmem(slice: &mut [u8], data: &[u8]) {
    let len = slice.len().min(data.len()); 
    slice[..len].copy_from_slice(&data[..len]);
}
