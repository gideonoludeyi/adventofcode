use std::io::{self, BufRead, BufReader};

fn main() {
    let total = solve(&mut BufReader::new(io::stdin())).unwrap();
    println!("{:?}", total);
}

fn solve(reader: &mut dyn BufRead) -> io::Result<i32> {
    let grid = get_grid(reader)?;

    let (spans, symspans) = get_spans(&grid);

    let mut total = 0;
    for (sx, sy) in &symspans {
        let mut coords = vec![];
        for (x, ys, yf) in &spans {
            if is_adjacent((*sx, *sy), (*x, *ys, *yf)) {
                coords.push((*x, *ys, *yf));
            }
        }
        if coords.len() == 2 {
            let (fx, fy0, fy1) = coords[0];
            let (sx, sy0, sy1) = coords[1];
            total += to_decimal(&grid[fx][fy0..=fy1]) * to_decimal(&grid[sx][sy0..=sy1])
        }
    }

    Ok(total)
}

fn to_decimal(slice: &[char]) -> i32 {
    let mut num = 0;
    let len = slice.len();
    for i in 0..len {
        num += (slice[i].to_digit(10).unwrap() as i32)
            * i32::pow(10, (len - i - 1).try_into().unwrap());
    }
    num
}

fn get_grid(reader: &mut dyn BufRead) -> io::Result<Vec<Vec<char>>> {
    let mut buf = String::new();
    let mut grid: Vec<Vec<char>> = vec![];
    while reader.read_line(&mut buf)? != 0 {
        let row = buf.trim().chars().collect::<Vec<char>>();
        grid.push(row);
        buf.clear();
    }
    Ok(grid)
}

fn get_spans(grid: &Vec<Vec<char>>) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
    let mut spans: Vec<(usize, usize, usize)> = vec![];
    let mut symspans: Vec<(usize, usize)> = vec![];
    for i in 0..grid.len() {
        let mut s = 0;
        let mut e = 0;
        while s < grid[i].len() {
            if grid[i][s].is_digit(10) {
                for j in s..grid[i].len() {
                    if grid[i][j].is_digit(10) {
                        e = j;
                    } else {
                        break;
                    }
                }
                spans.push((i, s, e));
                s = e + 1;
            } else if grid[i][s] == '*' {
                symspans.push((i, s));
                s += 1;
            } else {
                s += 1;
            }
        }
    }
    (spans, symspans)
}

fn is_adjacent((sx, sy): (usize, usize), (x, ys, yf): (usize, usize, usize)) -> bool {
    let is_nearby_row = sx.abs_diff(x) <= 1;
    let is_nearby_col = usize::checked_sub(ys, 1).unwrap_or(0) <= sy && sy <= (yf + 1);
    is_nearby_row && is_nearby_col
}

#[cfg(test)]
mod test {
    use super::*;
    use io::Cursor;

    #[test]
    fn test_solve() {
        let mut r: Cursor<&str> = Cursor::new(
            "\
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ",
        );
        assert_eq!(solve(&mut r).unwrap(), 467835);
    }

    #[test]
    fn test_is_adjacent() {
        let span = (0_usize, 0_usize, 2_usize);
        let symspan = (1_usize, 3_usize);
        assert!(is_adjacent(symspan, span));

        let span = (7_usize, 6_usize, 8_usize);
        let symspan = (8_usize, 5_usize);
        assert!(is_adjacent(symspan, span));
    }
}
