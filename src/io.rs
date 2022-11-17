use std::{
    fs::File,
    io::{stdin, stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    str::from_utf8,
};

/// This struct provides a layer of abstraction over all I/O operations for you.
///
/// You can construct it with a custom reader and writer, the cli or with a file.
///
/// Io is not safe! It is only intended to be used for competitive programming
/// and hence often uses expect.
#[derive(Debug)]
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
        self.writer
            .write_all(s.to_string().as_bytes())
            .expect("could not write to I/O output buffer");
    }
    /// Use this function to write to the previously given output writer. The output will be
    /// flushed and a newline character will be appended.
    pub fn writeln<S: ToString>(&mut self, s: S) {
        self.write(s);
        self.write('\n');
        self.flush();
    }
    /// This function flushes the output. Do not call this function often inside of a loop as that
    /// will lead to bad performance.
    pub fn flush(&mut self) {
        self.writer
            .flush()
            .expect("could not flush I/O output buffer");
    }
    /// This function can be used to read in any given type of variable. It will automatically
    /// ignore spaces, commas, newlines and tabs and will try to convert the next tokens into the specified
    /// type. It uses unwrap and is therefore unsafe.
    pub fn read<T: std::str::FromStr<Err = impl std::fmt::Debug>> (&mut self) -> T
    {
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.expect("could not read bytes in io read operation"))
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b',')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t' && b != b',')
            .collect::<Vec<_>>();
        from_utf8(&buf)
            .expect("data was not valid UTF-8 and could not be converted to a String")
            .parse()
            .unwrap()
    }
    /// This function reads the entire contents in the reader to a String to be used outside of the
    /// I/O helper. Note that it will ignore whitespaces and other characters and will keep on
    /// reading until it reaches EOF.
    pub fn read_all(&mut self) -> String {
        let mut res = String::new();
        self.reader
            .read_to_string(&mut res)
            .expect("data was not valid UTF-8 and could not be converted to a String");
        res
    }
    /// This function reads the next line into a String. It only checks for \n and \r.
    pub fn read_line(&mut self) -> String {
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.expect("could not read bytes in io read operation"))
            .take_while(|&b| b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        from_utf8(&buf)
            .expect("data was not valid UTF-8 and could not be converted to a String")
            .to_owned()
    }
    /// This function can be used to read a single char.
    pub fn read_char(&mut self) -> char {
        self.reader
            .by_ref()
            .bytes()
            .map(|b| b.expect("could not read bytes in io read operation"))
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b',')
            .next()
            .unwrap() as char
    }
    /// This function can be used to read indexes which are 1-based. It will subtract 1 and convert
    /// them into usize which can be used with Vectors.
    pub fn idx(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    /// This function can be used to read a Vector. It will read tokens of the given type *n*
    /// times.
    pub fn vec<T: std::str::FromStr<Err = impl std::fmt::Debug>> (&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read::<T>()).collect()
    }
    /// This function reads the whole file and then returns a Vector with I/O handlers for each line.
    pub fn line_io(&mut self) -> impl std::iter::Iterator<Item = Io<Cursor<String>, Stdout>> {
        let file = self.read_all();
        file.lines()
            .map(move |line| Io::from_string(line.to_string()))
            .collect::<Vec<Io<Cursor<String>, Stdout>>>()
            .into_iter()
    }
    /// This function reads the whole file and then returns a Vector with Strings for each line.
    pub fn lines(&mut self) -> Vec<String> {
        let file = self.read_all();
        file.lines().map(|line| line.to_string()).collect()
    }
    /// This function can be used to read the next string into a char array.
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
    /// This function writes a newline character \n.
    pub fn nl(&mut self) {
        self.write('\n');
    }
}

impl Io<Stdin, Stdout> {
    /// This functions creates the default I/O handler using stdin and stdout as reader and writer.
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
            panic!(
                "You cannot create an I/O handler which writes to and reads from the same file!"
            );
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

impl Io<&[u8], Stdout> {
    /// This function creates an io handler from a &str which can be used to make parsing easier.
    pub fn from_str(input: &str) -> Io<&[u8], Stdout> {
        Io {
            reader: BufReader::new(input.as_bytes()),
            writer: BufWriter::new(stdout()),
        }
    }
    /// This function creates an io handler from a String which can be used to parse lines easier.
    pub fn from_string(input: String) -> Io<Cursor<String>, Stdout> {
        Io {
            reader: BufReader::new(Cursor::new(input)),
            writer: BufWriter::new(stdout()),
        }
    }
}
