/// This module contains all I/O related helper utilities
///
/// # Example:
///
/// ```no_run
/// use cp_rs::io::*;
/// let mut io = Io::new();
/// let a: usize = io.read();
/// io.write(a);
/// ```
///
pub mod io;

/// This module contains all sorts of utilities which can be helpful for competitive programming.
pub mod utils;

/// This module contains geometry related structs (Point2D, Segment, ...) and methods for them
pub mod geometry;

#[cfg(test)]
mod tests {
    mod io {
        #[test]
        fn playground() {
            use crate::io::*;
            // use std::io::*;
            let s = "5, 4, 3, 2, 1";
            let mut io = Io::from_str(s);
            let a: usize = io.read();
            io.write(a);
        }
    }
}
