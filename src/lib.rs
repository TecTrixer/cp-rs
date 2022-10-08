/// This module contains all IO related helper utilities
///
/// Most important are the InRead and the OutWrite structs imported from the easy\_io crate. They
/// can be used to write / read from / to a file or standard in-/output.
///
/// ```no_run
/// use cp_rs::io::*;
/// let mut io = Io::new();
/// let a: usize = io.read();
/// io.write(a);
/// ```
///
pub mod io;

#[cfg(test)]
mod tests {
    mod io {
        #[test]
        fn playground() {
        }
    }
}
