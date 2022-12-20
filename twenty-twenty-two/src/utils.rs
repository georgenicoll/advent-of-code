use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufReader, Error, BufRead};
use std::str::{Chars, FromStr};


/// Processes a file line by line
///
/// Type Parameters relate to:
/// - S: type that a line is parsed into
/// - I: intermediate 'accumulator' type that an S is converted into
/// - R: result type
pub fn process_file<S, T, R>(filename: &str,
                             line_parse_func: fn (String) -> S,
                             zero: T,
                             accumulator_func: fn (T, S) -> T,
                             reduction_func: fn (T) -> R) -> Result<R, Error> {
    File::open(filename).map(BufReader::new).map(|reader| {
        let mut acc: T = zero;
        for (_, line_res) in reader.lines().enumerate()  {
            let line = line_res.unwrap();
            let parsed: S = line_parse_func(line);
            acc = accumulator_func(acc, parsed);
        }
        reduction_func(acc)
    })
}

///skip n characters in chars
pub fn skip(chars: &mut Chars<'_>, n: usize) {
    for _ in 0..n {
        chars.next();
    }
}

///read in a string, this is expected to be terminated by ',', ':', ' ', ';' or end of the chars
pub fn parse_next_string(chars: &mut Chars<'_>) -> String
{
    let mut s = String::new();
    for c in chars {
        let finish = match c {
            ',' | ':' | ' ' | ';' => true,
            _ => {
                s.push(c);
                false
            },
        };
        if finish {
            break;
        }
    }
    s
}

///read in a number, this is expected to be terminated by ',', ':', ' ', ';' or end of the chars
pub fn parse_next_number<T: FromStr>(chars: &mut Chars<'_>) -> Result<T, T::Err>
{
    let s = parse_next_string(chars);
    s.parse()
}

struct WriteAdapter<W>(W);

impl<W> fmt::Write for WriteAdapter<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args).map_err(|_| fmt::Error)
    }
}


pub fn output_into_iter<W: fmt::Write, I>(f: &mut W, separator: &str, iter: &mut I)
where
    I: Iterator,
    I::Item: Display,
{
    if let Some(thing) = iter.next() {
        write!(f, "{}", thing).unwrap();
    }
    let mut next_thing = iter.next();
    while next_thing.is_some() {
        write!(f, "{}{}", separator, next_thing.unwrap()).unwrap();
        next_thing = iter.next();
    }
}

pub fn output_into_iter_io<W: io::Write, I>(f: W, separator: &str, iter: &mut I)
where
    I: Iterator,
    I::Item: Display,
{
    output_into_iter(&mut WriteAdapter(f), separator, iter)
}
