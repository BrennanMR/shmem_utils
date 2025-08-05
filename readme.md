# shmem_utils: A Wrapper for Shared Memory

## Usage

Add this to your `Cargo.toml` dependencies:

```toml
shmem_utils = "0.1.0"
```

### Import and initialize the SHMEM



```rust
use shmem_utils::SharedMem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut shmem = SharedMem::new("shmem", 128, true)?;

    Ok(())
}
```

### Write to the SHMEM

```rust
use shmem_utils::SharedMem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut shmem = SharedMem::new("shmem", 128, true)?;
    let slice = shmem.shmem_mut();
    write_shmem::write_shmem(slice, &[0b11111111]);

    Ok(())
}
```

### Read from the SHMEM

```rust
use shmem_utils::SharedMem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut shmem = SharedMem::new("shmem", 128, true)?;
    let slice = shmem.shmem_mut();
    write_shmem::write_shmem(slice, &[0b11111111]);
    println!("{}", read_shmem::read_shmem(slice));
    
    Ok(())
}
```
