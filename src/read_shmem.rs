use shared_memory::ShmemConf;

/// Reads single byte
pub fn read_shmem(slice: &[u8]) -> u8 {
    let byte = slice[0];
    byte
}
