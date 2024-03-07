![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/mangopanda455/macext/.github%2Fworkflows%2Frust.yml)
![Crates.io](https://img.shields.io/crates/d/macext)
![Crates.io](https://img.shields.io/crates/v/macext)

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
    let mut value: u64 = 0; // To store the value found at address
    let final_value: u64 = 9999; // The value to write to address

    // Usage of read and patch
    value = read(&offsets, base_address, pid); // Returns a u64
    patch(&offsets, base_address, pid, final_value); // Must patch in a u64
}
```

### Running

To access process memory, you must run the program as root.

```bash
sudo cargo run
```

### Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
macext = "0.2.0"
```

Or run the following command:

```bash
cargo add macext
```
