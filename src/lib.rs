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
///
/// For example, with the radix function you can convert positive numbers into Strings with a given
/// radix.
///
/// ```
/// use cp_rs::utils::radix;
/// let num = 5;
/// let string_base2 = radix(num, 2).to_string();
/// assert_eq!(string_base2, "101")
/// ```
pub mod utils;

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
