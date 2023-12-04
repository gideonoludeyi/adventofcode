use std::io;

const MAP: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut total: i32 = 0;
    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.len();
        for i in 0..n {
            let slice = &buf[i..];
            let fst = MAP
                .iter()
                .find(|(word, digit)| slice.starts_with(word) || slice.starts_with(digit))
                .and_then(|(_, digit)| digit.parse::<i32>().ok());
            if let Some(n) = fst {
                total += n * 10;
                break;
            }
        }

        for i in 0..n {
            let slice = &buf[..(n - i)];
            let lst = MAP
                .iter()
                .find(|(word, digit)| slice.ends_with(word) || slice.ends_with(digit))
                .and_then(|(_, digit)| digit.parse::<i32>().ok());
            if let Some(n) = lst {
                total += n;
                break;
            }
        }

        buf.clear();
    }
    println!("{}", total);
    Ok(())
}
