use std::fs::File;
use std::io::{BufReader, Error, BufRead};


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
