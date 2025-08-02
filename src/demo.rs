// demo
use shmem_utils::init_shmem;
use crate::init_shmem::SharedMem;
mod read_shmem;
mod write_shmem;

fn demo() -> Result<(), Box<dyn std::error::Error>> {

    //
    let mut shmem = SharedMem::new("shmem", 128, true)?;
    let slice = shmem.shmem_mut();
    write_shmem::write_shmem(slice, &[0b11111111]);
    println!("{}", read_shmem::read_shmem(slice));
    //
    
    Ok(())
}
