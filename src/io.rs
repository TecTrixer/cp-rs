use regex::Regex;
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
    ///
    /// # Example
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::new();
    /// io.write("Test\n");
    /// io.write(5);
    /// ```
    /// The above code will output:\
    /// Test\
    /// 5
    pub fn write<S: ToString>(&mut self, s: S) {
        self.writer
            .write_all(s.to_string().as_bytes())
            .expect("could not write to I/O output buffer");
    }
    /// Use this function to write to the previously given output writer. The output will be
    /// flushed and a newline character will be appended.
    ///
    /// # Example
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::new();
    /// io.writeln("Test");
    /// io.write(5);
    /// ```
    /// The above code will output:\
    /// Test\
    /// 5
    pub fn writeln<S: ToString>(&mut self, s: S) {
        self.write(s);
        self.nl();
        self.flush();
    }
    /// Use this function to write to the previously given output writer, when the content you want
    /// to print only has the Debug trait and not the ToString / Display trait. The output will be
    /// buffered to make it faster.
    ///
    /// # Example
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::new();
    /// io.writed(vec![0, 1, 2]);
    /// ```
    /// The above code will output:\
    /// \[0, 1, 2\]
    pub fn writed<S: std::fmt::Debug>(&mut self, s: S) {
        self.writer
            .write_fmt(format_args!("{:?}", s))
            .expect("could not write to I/O output buffer");
    }
    /// Use this function to write to the previously given output writer, when the content you want
    /// to print only has the Debug trait and not the ToString / Display trait. The output will be
    /// flushed and a newline character will be appended.
    ///
    /// # Example
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::new();
    /// io.writedln(vec![0, 1, 2]);
    /// io.writed(vec![3, 4, 5]);
    /// ```
    /// The above code will output:\
    /// \[0, 1, 2\]\
    /// \[3, 4, 5\]
    pub fn writedln<S: std::fmt::Debug>(&mut self, s: S) {
        self.writed(s);
        self.nl();
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
    ///
    /// # Example
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("1, hello -5.1\n-9");
    /// let int: u32 = io.read();
    /// assert_eq!(int, 1);
    /// let string: String = io.read();
    /// assert_eq!(string, String::from("hello"));
    /// let float: f32 = io.read();
    /// assert_eq!(float, -5.1);
    /// let neg_int: i32 = io.read();
    /// assert_eq!(neg_int, -9);
    /// ```
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
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
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not parse value"))
            .unwrap()
    }
    /// This function reads the entire contents in the reader to a String to be used outside of the
    /// I/O helper. Note that it will ignore whitespaces and other characters and will keep on
    /// reading until it reaches EOF.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("test 1 +4, 1\nabc");
    /// let content = io.read_all();
    /// assert_eq!(content, String::from("test 1 +4, 1\nabc"));
    /// ```
    pub fn read_all(&mut self) -> String {
        let mut res = String::new();
        self.reader
            .read_to_string(&mut res)
            .expect("data was not valid UTF-8 and could not be converted to a String");
        res
    }
    /// This function reads the next line into a String. It only checks for \n and \r.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("test 1 +4, 1\nabc");
    /// let first_line = io.read_line();
    /// assert_eq!(first_line, String::from("test 1 +4, 1"));
    /// let second_line = io.read_line();
    /// assert_eq!(second_line, String::from("abc"));
    /// ```
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
    /// This function can be used to read a single char. Note that spaces, commas, tabs and
    /// newlines will still be skipped.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("a1, +");
    /// let first_char = io.read_char();
    /// assert_eq!(first_char, 'a');
    /// let second_char = io.read_char();
    /// assert_eq!(second_char, '1');
    /// let third_char = io.read_char();
    /// assert_eq!(third_char, '+');
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("3\n0, 1, 2");
    /// let idx = io.idx();
    /// let vec = io.nums::<usize>();
    /// assert_eq!(vec[idx], 2);
    /// ```
    pub fn idx(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    /// This function can be used to read a Vector. It will read tokens of the given type *n*
    /// times.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("3\n0, 1, 2");
    /// let len: usize = io.read();
    /// let vec = io.vec::<usize>(len);
    /// assert_eq!(vec, vec![0, 1, 2]);
    /// ```
    pub fn vec<T: std::str::FromStr<Err = impl std::fmt::Debug>>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read::<T>()).collect()
    }
    /// This function reads the whole file and then returns a Vector with I/O handlers for each line.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("1, a\n2, b");
    /// for mut line in io.line_io() {
    ///     let (n, c): (usize, char) = line.tuple();
    ///     // n, c = 1, a in first iteration
    ///     // n, c = 2, b in second iteration
    /// }
    /// ```
    pub fn line_io(&mut self) -> impl std::iter::Iterator<Item = Io<Cursor<String>, Stdout>> {
        let file = self.read_all();
        file.lines()
            .map(move |line| Io::from_string(line.to_string()))
            .collect::<Vec<Io<Cursor<String>, Stdout>>>()
            .into_iter()
    }
    /// This function reads the whole file and then returns a Vector with Strings for each line.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("1, a\n2, b");
    /// for line in io.lines() {
    ///     // line = String::from("1, a") in first iteration
    ///     // line = String::from("2, 5") in first iteration
    /// }
    /// ```
    pub fn lines(&mut self) -> Vec<String> {
        let file = self.read_all();
        file.lines().map(|line| line.to_string()).collect()
    }
    /// This function can be used to read the next string into a char array.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("abc, def");
    /// let first_chars = io.chars();
    /// assert_eq!(first_chars, vec!['a', 'b', 'c']);
    /// let second_chars = io.chars();
    /// assert_eq!(second_chars, vec!['d', 'e', 'f']);
    /// ```
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
    /// This function writes a newline character \n.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::new();
    /// io.write("Test");
    /// io.nl();
    /// io.nl();
    /// io.write(5);
    /// ```
    ///
    /// The above code will output:\
    /// Test\
    /// \
    /// 5
    pub fn nl(&mut self) {
        self.write('\n');
    }
    /// This function reads the whole file and then returns all numbers matching the regex r'-?\d+'
    /// as a vector.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("a: 12, b: -1 and d = 2");
    /// let nums = io.nums::<isize>();
    /// assert_eq!(nums, vec![12, -1, 2]);
    /// ```
    pub fn nums<T: std::str::FromStr<Err = impl std::fmt::Debug>>(&mut self) -> Vec<T> {
        let file = self.read_all();
        let re = Regex::new(r"(-?\d+)").unwrap();
        re.captures_iter(&file)
            .map(|x| x.get(1).unwrap().as_str().parse::<T>().unwrap())
            .collect::<Vec<T>>()
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

/// Trait automatically implemented for Io struct which allows to get tuples with only one function
/// call.
pub trait Tuple<T> {
    /// With this function, you can read multiple elements into a tuple:
    /// ```
    /// use crate::cp_rs::io::*;
    /// let mut io = Io::from_str("1, hello, -5.1");
    /// let (num, string, float): (u32, String, f32) = io.tuple();
    ///
    /// assert_eq!(num, 1);
    /// assert_eq!(string, String::from("hello"));
    /// assert_eq!(float, -5.1);
    /// ```
    ///
    /// It works for tuples with up to 6 elements
    fn tuple(&mut self) -> T;
}

impl<T1, T2, R, W> Tuple<(T1, T2)> for Io<R, W>
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    R: Read,
    W: Write,
{
    fn tuple(&mut self) -> (T1, T2) {
        let t1: T1 = self.read();
        let t2: T2 = self.read();
        (t1, t2)
    }
}

impl<T1, T2, T3, R, W> Tuple<(T1, T2, T3)> for Io<R, W>
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    R: Read,
    W: Write,
{
    fn tuple(&mut self) -> (T1, T2, T3) {
        let t1: T1 = self.read();
        let t2: T2 = self.read();
        let t3: T3 = self.read();
        (t1, t2, t3)
    }
}

impl<T1, T2, T3, T4, R, W> Tuple<(T1, T2, T3, T4)> for Io<R, W>
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    T4: std::str::FromStr,
    R: Read,
    W: Write,
{
    fn tuple(&mut self) -> (T1, T2, T3, T4) {
        let t1: T1 = self.read();
        let t2: T2 = self.read();
        let t3: T3 = self.read();
        let t4: T4 = self.read();
        (t1, t2, t3, t4)
    }
}

impl<T1, T2, T3, T4, T5, R, W> Tuple<(T1, T2, T3, T4, T5)> for Io<R, W>
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    T4: std::str::FromStr,
    T5: std::str::FromStr,
    R: Read,
    W: Write,
{
    fn tuple(&mut self) -> (T1, T2, T3, T4, T5) {
        let t1: T1 = self.read();
        let t2: T2 = self.read();
        let t3: T3 = self.read();
        let t4: T4 = self.read();
        let t5: T5 = self.read();
        (t1, t2, t3, t4, t5)
    }
}

impl<T1, T2, T3, T4, T5, T6, R, W> Tuple<(T1, T2, T3, T4, T5, T6)> for Io<R, W>
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    T4: std::str::FromStr,
    T5: std::str::FromStr,
    T6: std::str::FromStr,
    R: Read,
    W: Write,
{
    fn tuple(&mut self) -> (T1, T2, T3, T4, T5, T6) {
        let t1: T1 = self.read();
        let t2: T2 = self.read();
        let t3: T3 = self.read();
        let t4: T4 = self.read();
        let t5: T5 = self.read();
        let t6: T6 = self.read();
        (t1, t2, t3, t4, t5, t6)
    }
}
