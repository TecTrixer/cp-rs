# cp-rs

![ci-workflow](https://github.com/tectrixer/cp-rs/actions/workflows/rust.yml/badge.svg)

This library is intended to be used for competitive programming. It will be extended further and further with solutions to math, graph, geometry, ... problems in competitive programming. It also contains useful helpers for cp e.g. an io helper.

## Usage Example

To read an integer and a float from each line in a file `input.txt` you can write the following code:
```rust
use cp_rs::io::*;

fn main() {
    let mut io = Io::from_file("input.txt");
    for mut line in io.line_io() {
        let (a, b): (u32, f32) = line.tuple();
    }
}
```

## Features

- [x] General I/O handler
- [x] Utility for radix changes (will be reworked and put in the math module)
- [x] Utility for memoization
- [ ] Utility for cryptographic functions
- [ ] Utility for arbitrary precition ints + floats
- [ ] Utility for random numbers
- [ ] Support for graphs and graph algorithms
- [ ] Support for math related algorithms / useful functions
- [ ] Support for geometry related problems


