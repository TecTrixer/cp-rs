use std::{
    fs::File,
    io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Stdout, Write},
    str::from_utf8_unchecked,
};

/// This struct provides a layer of abstraction over all IO operations for you.
///
/// You can construct it with a custom reader and writer, the cli or with a file.
///
/// Io is not safe! It is only intended to be used for competitive programming
/// and hence often uses unwrap.
pub struct Io<R, W>
where
    R: Read,
    W: Write,
{
    reader: BufReader<R>,
    writer: BufWriter<W>,
}

impl<R: Read, W: Write> Io<R, W> {
    /// With this function you can create a new Io instance with a custom reader and writer.
    pub fn with_reader_and_writer(reader: R, writer: W) -> Io<R, W> {
        Io {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }
    /// Use this function to write to the previously given output writer. The output will be
    /// buffered to make it faster.
    pub fn write<S: ToString>(&mut self, s: S) {
        self.writer.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
    /// This function can be used to read in any given type of variable. It will automatically
    /// ignore spaces, commas, newlines and tabs and will try to convert the next tokens into the specified
    /// type. It uses unwrap and is therefore unsafe.
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b',')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t' && b != b',')
            .collect::<Vec<_>>();
        unsafe { from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    /// This function reads the entire contents in the reader to a String to be used outside of the
    /// IO helper. Note that it will ignore whitespaces and other characters and will keep on
    /// reading until it reaches EOF.
    pub fn read_all(&mut self) -> String {
        let mut res = String::new();
        self.reader.read_to_string(&mut res).unwrap();
        res
    }
    /// This function reads the next line into a String. It only checks for \n and \r.
    pub fn read_line(&mut self) -> String {
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .take_while(|&b| b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { from_utf8_unchecked(&buf).to_owned() }
    }
    /// This function can be used to read indexes which are 1-based. It will subtract 1 and convert
    /// them into usize which can be used with Vectors.
    pub fn idx(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    /// This function can be used to read a Vector. It will read tokens of the given type *n*
    /// times.
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    /// This function can be used to read the next string into a char array.
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

impl Io<Stdin, Stdout> {
    /// This functions creates the default Io handler using stdin and stdout as reader and writer.
    pub fn new() -> Io<Stdin, Stdout> {
        Io {
            reader: BufReader::new(stdin()),
            writer: BufWriter::new(stdout()),
        }
    }
}
impl Io<File, Stdout> {
    /// This function uses the given file as input and stdout as output.
    pub fn from_file(filename: &str) -> Io<File, Stdout> {
        let reader = BufReader::new(
            File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(filename)
                .unwrap(),
        );
        Io {
            reader,
            writer: BufWriter::new(stdout()),
        }
    }
}
impl Io<File, File> {
    /// This function uses the first file for reading and the second file for writing.
    pub fn from_file_to_file(filename_in: &str, filename_out: &str) -> Io<File, File> {
        if filename_in == filename_out {
            panic!("You cannot create an IO handler which writes to and reads from the same file!");
        }
        let reader = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(filename_in)
            .unwrap();
        let writer = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(filename_out)
            .unwrap();
        Io {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }
}

impl Io<Stdin, File> {
    /// This function uses stdin for reading and outputs to a file.
    pub fn from_cli_to_file(filename: &str) -> Io<Stdin, File> {
        let writer = BufWriter::new(
            File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(filename)
                .unwrap(),
        );
        Io {
            reader: BufReader::new(stdin()),
            writer,
        }
    }
}
