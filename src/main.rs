use std::io::{self, BufRead, BufReader};
use std::str;

struct Tree {
    src_start: i32,
    dst_start: i32,
    range: i32,
    branches: Vec<Tree>,
}

fn main() {
    let solution = solve(&mut BufReader::new(io::stdin())).unwrap();
    println!("{}", solution);
}

fn solve(reader: &mut dyn BufRead) -> io::Result<i32> {
    let mut buf: Vec<u8> = Vec::new();
    while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
        if bytes_read == 0 {
            break;
        }
        println!("{:?}", &str::from_utf8(&buf));
        buf.clear();
    }
    Ok(0)
    // let tree = Tree {
    //     src_start: 0,
    //     dst_start: 0,
    //     range: i32::MAX,
    //     branches: Vec::from([
    //         Tree {
    //             src_start: 50,
    //             dst_start: 98,
    //             range: 2,
    //             branches: vec![],
    //         },
    //         Tree {
    //             src_start: 52,
    //             dst_start: 50,
    //             range: 48,
    //             branches: vec![],
    //         },
    //     ]),
    // };
}

#[cfg(test)]
mod test {
    use super::*;
    use io::Cursor;

    #[test]
    fn test_solve() {
        let mut r: Cursor<&str> = Cursor::new(
            "\
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        ",
        );
        assert_eq!(35, solve(&mut r).unwrap());
    }
}
