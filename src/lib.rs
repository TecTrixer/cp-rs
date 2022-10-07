/// This module contains all IO related helper utilities
///
/// Most important are the InRead and the OutWrite structs imported from the easy\_io crate. They
/// can be used to write / read from / to a file or standard in-/output.
pub mod io;

#[cfg(test)]
mod tests {
    mod io {
        use crate::*;
        #[test]
        fn playground() {
            use io::*;
            let mut input = InRead::from_file("input.txt");
            let a = input.next::<isize>();
            assert_eq!(a, 5);
        }
    }
}
