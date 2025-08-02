#![allow(warnings)]

use shared_memory::{Shmem, ShmemConf};

/// Wrapper struct that owns the `Shmem` and exposes the mutable slice
pub struct SharedMem {
    shmem: Shmem,
}

impl SharedMem {
    /// Initialize shmmem and create a mut slice
    /// 
    /// Set arg `allow_open_on_error` true if you want to try creating the shmem, and opening it on failure
    pub fn new(name: &str, size: u32, allow_open_on_error: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let shmem = ShmemConf::new()
            .size(size.try_into().unwrap())
            .os_id(name)
            .create()
            .or_else(|_| ShmemConf::new().os_id(name).open())?;

        Ok(Self { shmem })
    }

    /// Get a mutable slice to the shared memory
    pub fn shmem_mut(&mut self) -> &mut [u8] {
        let slice = unsafe { self.shmem.as_slice_mut() };
        slice
    }
}
