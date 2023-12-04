use std::io;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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
    let mut total = 0;
    let mut nfails = 0;
    while io::stdin().read_line(&mut buf)? != 0 {
        let mut line = buf.split([':', ';', ',']);
        let game_num = (&line.next().unwrap()[5..]).parse::<i32>().unwrap();
        total += game_num;
        for color in line.map(|s| s.trim()) {
            let colordef = color.split(' ').collect::<Vec<&str>>();
            let n = colordef[0].parse::<i32>().unwrap();
            let clr = colordef[1];
            if (clr == "red" && n > MAX_RED)
                || (clr == "green" && n > MAX_GREEN)
                || (clr == "blue" && n > MAX_BLUE)
            {
                nfails += game_num;
                break;
            }
        }
        buf.clear();
    }
    println!("{}", total - nfails);
    Ok(())
}
