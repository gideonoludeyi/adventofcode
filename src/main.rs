use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let solution = solve(&mut BufReader::new(io::stdin())).unwrap();
    println!("{}", solution);
}

fn solve(reader: &mut dyn BufRead) -> io::Result<i32> {
    let values: Vec<_> = reader
        .lines()
        .filter(|r| r.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(true))
        .map(|r: io::Result<String>| r.map(solve_line))
        .collect::<io::Result<Vec<_>>>()?;

    let mut card_counts: Vec<i32> = vec![1; values.len()];
    for i in 0..values.len() {
        for j in (i + 1)..=(i + values[i] as usize) {
            card_counts[j] += card_counts[i];
        }
    }
    Ok(card_counts.iter().sum())
}

fn solve_line(line: String) -> i32 {
    let vec = line.split([':', '|']).skip(1).collect::<Vec<&str>>(); // skip "Card n"
    if vec.len() < 2 {
        return 0;
    }
    let mut winning = parse_whitespace_separated_nums::<u8>(vec[0]).unwrap();
    let mut received = parse_whitespace_separated_nums::<u8>(vec[1]).unwrap();
    winning.sort();
    received.sort();

    let mut wins = 0_i32;
    let (mut r, mut w) = (0_usize, 0_usize); // pointer indices to elements of `received` and
                                             // `winning` respectively
    while r < received.len() && w < winning.len() {
        if received[r] == winning[w] {
            wins += 1;
            r += 1;
            w += 1;
        } else if received[r] < winning[w] {
            r += 1;
        } else {
            w += 1;
        }
    }

    wins
}

fn parse_whitespace_separated_nums<T: FromStr>(text: &str) -> Result<Vec<T>, T::Err> {
    text.trim()
        .split_ascii_whitespace()
        .map(T::from_str)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use io::Cursor;

    #[test]
    fn test_solve() {
        let mut r: Cursor<&str> = Cursor::new(
            "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ",
        );
        assert_eq!(30, solve(&mut r).unwrap());
    }
}
