struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let mut games = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if buf.trim().is_empty() {
            break;
        }
        games.push(parse_game(&buf));
    }
    let valid = validate(games);
    println!("sum: {}", valid.iter().sum::<i32>())
}

fn validate(games: Vec<Game>) -> Vec<i32> {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let mut valid = Vec::new();
    for game in games {
        if game.red > MAX_RED {
            println!("game {} impossible! red: {} > {}", game.id, game.red, MAX_RED);
            continue;
        }
        if game.green > MAX_GREEN {
            println!(
                "game {} impossible! green: {} > {}",
                game.id, game.green, MAX_GREEN
            );
            continue;
        }
        if game.blue > MAX_BLUE {
            println!("game {} impossible! blue: {} > {}", game.id, game.blue, MAX_BLUE);
            continue;
        }
        valid.push(game.id);
    }
    println!("Valid games: {:?}", valid);
    valid
}

fn parse_game(buf: &String) -> Game {
    let id = buf
        .split(' ')
        .nth(1)
        .unwrap()
        .split(':')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in buf.split(": ").last().unwrap().split(';') {
        if set.contains("red") {
            let max_red = set
                .split(" red")
                .next()
                .unwrap_or_else(|| " 0")
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            red = std::cmp::max(red, max_red);
        }
        if set.contains("green") {
            let max_green = set
                .split(" green")
                .next()
                .unwrap_or_else(|| " 0")
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            green = std::cmp::max(green, max_green);
        }
        if set.contains("blue") {
            let max_blue = set
                .split(" blue")
                .next()
                .unwrap_or_else(|| " 0")
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            blue = std::cmp::max(blue, max_blue);
        }
    }
    Game {
        id,
        red,
        green,
        blue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let mut games = Vec::new();
        for line in input.lines() {
            games.push(parse_game(&line.to_owned()))
        }
        let valid = validate(games);
        assert_eq!(valid, [1, 2, 5])
    }
}
