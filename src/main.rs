use std::io::{self, BufRead, BufReader};

fn main() {
    let solution = solve(&mut BufReader::new(io::stdin())).unwrap();
    println!("{:?}", solution);
}

fn solve(reader: &mut dyn BufRead) -> io::Result<i32> {
    reader.lines().map(|x| x.map(solve_line)).sum()
}

fn solve_line(line: String) -> i32 {
    let vec = line.split([':', '|']).skip(1).collect::<Vec<&str>>(); // skip "Card n"
    if vec.len() < 2 {
        return 0;
    }
    let mut winning = vec[0]
        .trim()
        .split_ascii_whitespace()
        .map(str::parse::<u8>)
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    winning.sort();
    let mut received = vec[1]
        .trim()
        .split_ascii_whitespace()
        .map(str::parse::<u8>)
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    received.sort();

    let mut points = 0_i32;
    let mut r = 0_usize;
    let mut w = 0_usize;
    loop {
        if r >= received.len() {
            break;
        }
        if w >= winning.len() {
            break;
        }
        if received[r] == winning[w] {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
            r += 1;
            w += 1;
        } else if received[r] < winning[w] {
            r += 1;
        } else {
            w += 1;
        }
    }

    points
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
        assert_eq!(13, solve(&mut r).unwrap());
    }
}
