#![feature(map_try_insert)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct ParseListsResult(Vec<u32>, Vec<u32>);
struct ParseListsForSimilarityResult(Vec<u32>, HashMap<u32, u32>);

fn parse_lists(lines: &[String]) -> ParseListsResult {
    let mut res = ParseListsResult(Vec::<u32>::new(), Vec::<u32>::new());

    lines.iter().for_each(|l: &String| {
        let v = l.split("   ").collect::<Vec<&str>>();
        assert!(v.len() == 2);

        res.0
            .push(v[0].parse::<u32>().expect("Failed to parse l_vec number"));
        res.1
            .push(v[1].parse::<u32>().expect("Failed to parse r_vec number"));
    });
    res
}

fn parse_lists_for_simalarity(lines: &[String]) -> ParseListsForSimilarityResult {
    let mut res = ParseListsForSimilarityResult(Vec::<u32>::new(), HashMap::<u32, u32>::new());
    lines.iter().for_each(|l: &String| {
        let v = l.split("   ").collect::<Vec<&str>>();
        assert!(v.len() == 2);

        res.0
            .push(v[0].parse::<u32>().expect("Failed to parse l_vec number"));
        let map_num: u32 = v[1].parse::<u32>().expect("Failed to parse r_map number");
        match res.1.try_insert(map_num, 1) {
            Ok(_) => {}
            Err(mut a) => *a.entry.get_mut() += 1,
        }
    });
    res
}

fn compute_similarities(res: ParseListsForSimilarityResult) -> u32 {
    res.0
        .iter()
        .map(|n: &u32| match res.1.get(n) {
            Some(x) => n * x,
            None => 0,
        })
        .reduce(|acc, e| acc + e)
        .expect("Failed to reduce similarity results")
}

fn distance(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn compute_distances(mut res: ParseListsResult) -> u32 {
    res.0.sort_unstable();
    res.1.sort_unstable();
    let mut t: u32 = 0;
    res.0
        .into_iter()
        .enumerate()
        .for_each(|(i, num): (usize, u32)| {
            t += distance(num, res.1[i]);
        });
    t
}

fn main() {
    let input: &Path = Path::new("crates/day_1/samples/real.txt");
    let v = make_line_vec(input);
    let out = parse_lists(&v);
    let res = compute_distances(out);

    println!("Final value: {:?}", res);

    let out = parse_lists_for_simalarity(&v);
    let res = compute_similarities(out);

    println!("Final value: {:?}", res);
}

fn make_line_vec(input: &Path) -> Vec<String> {
    let f = File::open(input).expect("Failed to open file");
    BufReader::new(f)
        .lines()
        .map(|l| -> String { l.expect("failed to read line!") })
        .collect::<Vec<String>>()
}
