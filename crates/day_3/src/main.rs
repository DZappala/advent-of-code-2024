#![feature(iter_array_chunks)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::Path,
};

use spdlog::debug;

fn get_line(path: &Path) -> String {
    let mut buf: String = String::new();
    BufReader::new(File::open(path).expect("Could not open file"))
        .read_line(&mut buf)
        .expect("Failed to read string to buffer");
    buf
}

fn filter_line(ln: String) -> u32 {
    let mut stack: Vec<Vec<u32>> = Vec::new();
    let mut pre_stack: Vec<u32> = Vec::new();
    let mut buf: Vec<char> = Vec::new();
    let mut mul_is_on: bool = true;

    for ltr in ln.chars() {
        match ltr {
            'm' if mul_is_on && buf.is_empty() => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            'u' if mul_is_on && buf.last() == Some(&'m') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            'l' if mul_is_on && buf.last() == Some(&'u') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '(' if mul_is_on && buf.last() == Some(&'l') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '0'..='9' if mul_is_on && buf.last() == Some(&'(') || buf.last() == Some(&',') => {
                buf.clear();
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '0'..='9' if mul_is_on && has_digit(&buf) => buf.push(ltr),
            ',' if has_digit(&buf) && pre_stack.is_empty() => {
                pre_stack.push(parse_buffer(&buf));
                debug!("Pushed to pre-stack {:?}", pre_stack.last().unwrap());
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            ')' if mul_is_on && has_digit(&buf) && !pre_stack.is_empty() => {
                pre_stack.push(parse_buffer(&buf));
                debug!("Pushed to pre-stack {:?}", pre_stack.last().unwrap());
                stack.push(pre_stack.clone());
                debug!("Pushed to stack {:?}", pre_stack);
                pre_stack.clear();
                buf.clear()
            }
            'd' if buf.is_empty() => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            'o' if buf.last() == Some(&'d') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '(' if !mul_is_on && buf.last() == Some(&'o') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            ')' if !mul_is_on && buf.iter().collect::<String>() == "do(" => {
                buf.clear();
                mul_is_on = true;
                debug!("mul_is_on {mul_is_on:?}");
            }
            'n' if mul_is_on && buf.last() == Some(&'o') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '\'' if mul_is_on && buf.last() == Some(&'n') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            't' if mul_is_on && buf.last() == Some(&'\'') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            '(' if mul_is_on && buf.last() == Some(&'t') => {
                buf.push(ltr);
                debug!("Pushed {:?}", ltr);
            }
            ')' if mul_is_on && buf.iter().collect::<String>() == "don't(" => {
                buf.clear();
                mul_is_on = false;
                debug!("mul_is_on {mul_is_on:?}");
            }
            _ => {
                buf.clear();
                pre_stack.clear();
            }
        }
    }
    stack
        .iter()
        .map(|c: &Vec<u32>| c.iter().product::<u32>())
        .sum()
}

fn parse_buffer(buf: &[char]) -> u32 {
    buf.iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or_else(|a: ParseIntError| panic!("filter line: failed to parse digit {:?}", a))
}

fn has_digit(buf: &[char]) -> bool {
    buf.last().is_some_and(|c| c.is_ascii_digit())
}

fn main() {
    let ln = get_line(Path::new("crates/day_3/samples/real.txt"));
    let out = filter_line(ln);
    println!("Part 1: {out:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_a() {
        spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);
        let ln = get_line(Path::new("samples/test.txt"));
        let out = filter_line(ln);
        assert_eq!(out, 48)
    }

    // #[test]
    // fn test_data_b() {
    //     let ln = get_line(Path::new("samples/test.txt"));
    //     let ln = ln + "mul(30,469)*mul(308,586)mul(15,330)~@mul(803,223)";
    //     let ln = ln + "mul(248,461)+^mul(412,666)[/:]>@from()mul(815,918)/select()%]select()]'~mul(681,852)mul(994,337)";
    //     let out = filter_line(ln);
    //     assert_eq!(
    //         out,
    //         (161 + 30 * 469
    //             + 308 * 586
    //             + 15 * 330
    //             + 803 * 223
    //             + 248 * 461
    //             + 412 * 666
    //             + 815 * 918
    //             + 681 * 852
    //             + 994 * 337)
    //     )
    // }
}
