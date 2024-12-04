#![feature(if_let_guard)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn make_line_vec(path: &Path) -> Vec<String> {
    BufReader::new(File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| line.expect("Failed to read line in file"))
        .collect::<Vec<String>>()
}

fn parse_nums(data: &[String]) -> Vec<Vec<u32>> {
    data.iter()
        .map(|line| {
            line.split(" ")
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Couldn't parse digit: {:?}", num))
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn in_bounds(sub: u32) -> bool {
    sub > 0 && sub < 4
}

struct AnalyzeResult(u32, Vec<bool>);

fn analyze(data: &[Vec<u32>]) -> AnalyzeResult {
    let map = data.iter().map(|ln| -> bool {
        let mut increasing: Option<bool> = None;
        for (i, me) in ln.iter().enumerate() {
            if i == 0 || i == ln.len() - 1 {
                continue;
            }

            let prev = &ln[i - 1];
            let next = &ln[i + 1];

            if increasing.is_none() {
                increasing = Some(me > prev)
            }

            match increasing {
                Some(increasing) if increasing == (next > me) && increasing == (me > prev) => {}
                _ => return false,
            }

            let sub_next = match increasing {
                Some(increasing) if increasing => next
                    .checked_sub(*me)
                    .unwrap_or_else(|| panic!("Failed to subtract: {} - {}", next, me)),
                Some(_) => me - next,
                None => return false,
            };

            let sub_prev = match increasing {
                Some(increasing) if increasing => me
                    .checked_sub(*prev)
                    .unwrap_or_else(|| panic!("Failed to subtract: {} - {}", me, prev)),
                Some(_) => prev - me,
                None => return false,
            };

            match in_bounds(sub_next) && in_bounds(sub_prev) {
                true => continue,
                false => return false,
            }
        }
        true
    });
    let collected: Vec<bool> = map.clone().collect();
    AnalyzeResult(
        map.fold(0, |acc, e| if e { acc + 1 } else { acc }),
        collected,
    )
}

fn try_drop(ln: &[u32]) -> bool {
    let mut increasing: Option<bool> = None;
    println!("Try drop: {:?}", ln);
    for (i, me) in ln.iter().enumerate() {
        if i == 0 || i == ln.len() - 1 {
            continue;
        }

        let prev = &ln[i - 1];
        let next = &ln[i + 1];
        println!("try_drop(prev): {}", prev);
        println!("try_drop(next): {}", next);
        if increasing.is_none() {
            increasing = Some(me > prev);
            println!("try_drop: set increasing to {:?}", increasing);
        }

        match increasing {
            Some(increasing) if increasing == (next > me) && increasing == (me > prev) => {}
            _ => return false,
        }

        let sub_next = get_sub_next(increasing, next, me);
        let sub_prev = get_sub_prev(increasing, me, prev);
        if in_bounds(sub_next) && in_bounds(sub_prev) {
            continue;
        }

        return false;
    }
    true
}

fn rectify(eval: &[bool], data: &[Vec<u32>]) -> u32 {
    data.iter()
        .enumerate()
        .map(|(glob_i, ln)| -> bool {
            if eval[glob_i] {
                return true;
            }

            for (i, _) in ln.iter().enumerate() {
                if let Some(true) = try_rectify(ln, i) {
                    return true;
                }
            }
            false
        })
        .fold(0, |acc, e| if e { acc + 1 } else { acc })
}

fn try_rectify(ln: &[u32], i: usize) -> Option<bool> {
    let mut clone = ln.to_owned();
    clone.remove(i);
    let tried = try_drop(&clone);
    if !tried {
        println!("  Failed to pass when dropped.\n");
        Some(false)
    } else {
        Some(true)
    }
}

fn get_sub_next(increasing: Option<bool>, next: &u32, me: &u32) -> u32 {
    match increasing {
        Some(increasing) if increasing => next
            .checked_sub(*me)
            .unwrap_or_else(|| panic!("Failed to subtract: {} - {}", next, me)),
        Some(_) => me - next,
        None => unreachable!(),
    }
}

fn get_sub_prev(increasing: Option<bool>, me: &u32, prev: &u32) -> u32 {
    match increasing {
        Some(increasing) if increasing => me
            .checked_sub(*prev)
            .unwrap_or_else(|| panic!("Failed to subtract: {} - {}", me, prev)),
        Some(_) => prev - me,
        None => unreachable!(),
    }
}

fn main() {
    let path = Path::new("crates/day_2/samples/real.txt");
    let str_vec = make_line_vec(path);
    let parsed_nums = parse_nums(&str_vec);
    let AnalyzeResult {
        0: sum,
        1: bool_vec,
    } = analyze(&parsed_nums);

    let r = rectify(&bool_vec, &parsed_nums);

    println!("Analysis: {:?}", sum);
    println!("Rectification: {:?}", r);
    println!("Total lines: {:?}", str_vec.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sub_prev_test() {
        let increasing: Option<bool> = Some(true);
        let me: u32 = 1;
        let prev: u32 = 0;

        assert_eq!(get_sub_prev(increasing, &me, &prev), 1);

        let increasing = Some(false);
        let me = 2;
        let prev = 4;

        assert_eq!(get_sub_prev(increasing, &me, &prev), 2);
    }

    #[test]
    fn get_sub_next_test() {
        let increasing: Option<bool> = Some(true);
        let next: u32 = 1;
        let me: u32 = 0;

        assert_eq!(get_sub_next(increasing, &next, &me), 1);

        let increasing = Some(false);
        let next = 2;
        let me = 4;

        assert_eq!(get_sub_prev(increasing, &next, &me), 2);
    }

    #[test]
    fn try_rectify_test() {
        let ln = vec![1, 2, 3, 4, 4];
        let i = 4;
        assert_eq!(try_rectify(&ln, i), Some(true));
        let ln = vec![4, 3, 2, 1, 1];
        let i = 4;
        assert_eq!(try_rectify(&ln, i), Some(true));
        let ln = vec![4, 3, 3, 1, 3];
        let i = 2;
        assert_eq!(try_rectify(&ln, i), Some(false));
        let ln = vec![4, 3, 3, 4, 3];
        let i = 2;
        assert_eq!(try_rectify(&ln, i), Some(false));
    }

    #[test]
    fn rectify_test_a() {
        let path = Path::new("samples/test.txt");
        let str_vec = make_line_vec(path);
        let parsed_nums = parse_nums(&str_vec);
        let AnalyzeResult {
            0: sum,
            1: bool_vec,
        } = analyze(&parsed_nums);
        assert_eq!(sum, 2);
        assert_eq!(rectify(&bool_vec, &parsed_nums), 4);
    }

    #[test]
    fn rectify_test_b() {
        let path = Path::new("samples/test.txt");
        let str_vec = make_line_vec(path);
        let mut parsed_nums = parse_nums(&str_vec);
        parsed_nums.push(vec![44, 47, 48, 49, 48]);
        let AnalyzeResult {
            0: sum,
            1: bool_vec,
        } = analyze(&parsed_nums);
        assert_eq!(sum, 2);
        assert_eq!(rectify(&bool_vec, &parsed_nums), 5);
    }

    #[test]
    fn rectify_test_c() {
        let parsed_nums = vec![vec![44, 47, 48, 49, 48]];
        let AnalyzeResult {
            0: sum,
            1: bool_vec,
        } = analyze(&parsed_nums);
        assert_eq!(sum, 0);
        assert!(bool_vec.len() == 1);
        assert!(parsed_nums.len() == 1);
        dbg!(&parsed_nums[0]);
        let test_tried = try_rectify(&parsed_nums[0], 4);
        assert_eq!(test_tried, Some(true));
        assert_eq!(rectify(&bool_vec, &parsed_nums), 1);
    }

    #[test]
    fn rectify_test_d() {
        let parsed_nums = vec![vec![48, 49, 48, 47, 44]];
        let AnalyzeResult {
            0: sum,
            1: bool_vec,
        } = analyze(&parsed_nums);
        assert_eq!(sum, 0);
        assert_eq!(rectify(&bool_vec, &parsed_nums), 1);
    }
}
