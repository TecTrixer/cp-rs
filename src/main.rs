use cp_rs::{io::*, utils::*};
fn main() {
    for mut io in Io::from_file("test.txt").line_io() {
        let mut num: isize = io.read();
        let rdx: u32 = io.read();
        io.nl();
        if num < 0 {
            num *= -1;
            io.write('-');
        }
        io.write(radix(num, rdx as u8));
    }
}
