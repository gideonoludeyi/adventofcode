use std::io;

/*
* example
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut result = 0;
    while io::stdin().read_line(&mut buf)? != 0 {
        let line = buf.split([':', ';', ',']);

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for color in line.skip(1).map(str::trim) {
            let colordef = color.split(' ').collect::<Vec<&str>>();
            let n = colordef[0].trim().parse::<i32>().unwrap();
            match colordef[1] {
                "red" => min_red = std::cmp::max(min_red, n),
                "green" => min_green = std::cmp::max(min_green, n),
                "blue" => min_blue = std::cmp::max(min_blue, n),
                _ => continue,
            }
        }
        result += min_red * min_green * min_blue;
        buf.clear();
    }
    println!("{}", result);
    Ok(())
}
