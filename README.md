# MacExt

## Mac memory management made simple through rust

### Usage

```rust
use macext::*;

fn main() {
    let pid = get_pid("your_program");
    let base_address = get_base_address(pid);
    // Using fullprep:
    // let (pid, base_address) = fullprep("your_program")
    let offsets = vec![0x1d9ef0, 0x0, 0x418]; // Example offsets
    let mut value = 0; // To store the value found at address
    let final_value = 9999; // The value to write to address

    // Usage of read and patch
    value = read(&offsets, base_address, pid);
    patch(&offsets, base_address, pid, final_value);
}
```
