use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    thread::spawn,
};

fn parse(path: &Path) -> Vec<Vec<char>> {
    BufReader::new(File::open(path).expect("Couldn't open file"))
        .lines()
        .map(|l| {
            l.expect("Failed to parse line")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

//returns the number of instances
async fn search_lines(lns: &[&[char]]) -> u32 {
    let mut total: u32 = 0;
    let mut handles = vec![];
    for ln in lns.iter() {
        handles.push(spawn(move || search_line(ln)));
    }

    for handle in handles {
        let res = handle.join().unwrap().await;
        total += res;
    }

    total
}

async fn search_line(ln: &[char]) -> u32 {
    let buf: &[char] = &[];
    let mut total: usize = 0;
    total += ln
        .windows(4)
        .filter(|w| w.iter().collect::<String>() == "xmas")
        .count();
    total += ln
        .windows(4)
        .filter(|w| w.iter().collect::<String>() == "samx")
        .count();
    total as u32
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let path = Path::new("samples/test.txt");
        let parsed = parse(path);
    }
}
