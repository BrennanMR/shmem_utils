# shmem_utils: A Wrapper for Shared Memory

<small>
<b>NOTE:</b> Is this crate useful to you? I accept donations.<br>
<br>
<b>ETH</b>: 0x389C80c3cBe69a174fe021B5d79ccC2723Bfc85A<br>
<b>BTC</b>: 3MuoGzhSMaLBRJ3e3u4i3cgo38tTgDNk3o<br>
<b>USDC</b>: 0xCD51c1C606845bb4EeEdE230AB9d2EBa6E4b30df (on Base)<br>
<br>
I'm releasing a huge project soon — donations are the only way I can keep contributing :)
</small>

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
