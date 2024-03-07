[<img alt="github" src="https://img.shields.io/badge/github-mangopanda455/macext-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mangopanda455/macext)
[<img alt="crates.io" src="https://img.shields.io/crates/v/macext.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/macext)
[<img alt="downloads" src="https://img.shields.io/crates/d/macext.svg?style=for-the-badge&color=aa6bb0&logo=rust" height="20">](https://crates.io/crates/macext)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-macext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/macext)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mangopanda455/macext/rust.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mangopanda455/macext/actions?query=branch%3Amaster)

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
