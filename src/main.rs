use std::cmp;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::process;
use std::usize;

use docopt::Docopt;
use regex::Regex;
use serde::Deserialize;

const USAGE: &'static str = "
Print a bar chart.

Usage:
  barchart [options] [<filename>]

Arguments:
  <filename>    Read data from <filename>. When ommitted, reads from stdin.

Options:
  -h, --help    Show this help message.
  --width=<n>   Restrict output to <n> characters. [default: 80]
  --bar=<char>  Use <char> to draw the bars. [default: █]

Input format:
  <count> <text>

  Leading, trailing, or extra spaces in the middle are ignored.
  <text> is allowed to have spaces; <count> is not.
  (Format is meant to be compatible with the output of `uniq -c`.)

Example:
  barchart --width 40 --bar # my-data.txt
";

#[derive(Debug, Deserialize)]
struct Options {
    flag_width: usize,
    flag_bar: char,
    arg_filename: Option<String>,
}

#[derive(Debug)]
struct Entry {
    count: usize,
    text: String,
}

fn main() -> io::Result<()> {
    let options: Options = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let reader: Box<dyn BufRead> = match options.arg_filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?)),
    };

    let mut data = Vec::new();

    let re = Regex::new(r"^\s*(\d+)\s+(.+)\s*$").unwrap();
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        match re.captures(&line) {
            None => {
                eprintln!("Malformed line. Expected '<count> <text>'. Found: {}", line);
                process::exit(1);
            }
            Some(caps) => {
                let count = caps[1].parse().unwrap();
                let text = caps[2].to_string();
                data.push(Entry { count, text });
            }
        }
    }

    if data.len() == 0 {
        eprintln!("No data provided. See help for more information.");
        process::exit(1);
    }

    let max_text_len = data
        .iter()
        .map(|entry| &entry.text)
        .fold(usize::MIN, |max, curr| cmp::max(max, curr.len()));

    let max_count = data
        .iter()
        .map(|entry| entry.count)
        .fold(usize::MIN, |max, curr| cmp::max(max, curr));

    let max_digits = ((max_count + 1) as f64).log10().ceil() as usize;

    let max_label_width = max_text_len + 2 + max_digits + 1;
    let max_width = if max_label_width >= options.flag_width {
        eprintln!(
            "The longest row's label ({}) would take up more than the requested max width ({}).",
            max_label_width, options.flag_width
        );
        let max_width = max_label_width + 10;
        eprintln!("Assuming max width of {}", max_width);
        max_width
    } else {
        options.flag_width
    };

    let max_bar_width = max_width - max_label_width;

    for entry in data {
        let frac = (entry.count as f64) / (max_count as f64);
        let bar_len = (frac * (max_bar_width as f64)).round() as usize;
        let bar = options.flag_bar.to_string().repeat(bar_len);
        println!(
            "{0:1$}  {2:3$} {4:5$}",
            entry.text, max_text_len, entry.count, max_digits, bar, max_width
        );
    }

    Ok(())
}
