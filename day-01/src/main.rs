fn main() {
    let mut input = String::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if buf.trim().is_empty() {
            break;
        }
        input.push_str(&buf);
    }

    let mut output = 0;
    for line in input.lines() {
        let res = get_calibration(line);
        output += res;
    }
    println!("{}", output)
}

fn get_calibration(input: &str) -> i32 {
    let mut res = String::new();
    for char in input.chars() {
        if char.is_ascii_digit() {
            res.push(char);
            break;
        }
    }
    for char in input.chars().rev() {
        if char.is_ascii_digit() {
            res.push(char);
            break;
        }
    }
    println!("{}", res);
    res.parse().unwrap()
}
// nah
fn get_calibration_string(input: &str) -> i32 {
    let input = input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .replace("zero", "0");
    let mut res = String::new();
    for char in input.chars() {
        if char.is_ascii_digit() {
            res.push(char);
            break;
        }
    }
    for char in input.chars().rev() {
        if char.is_ascii_digit() {
            res.push(char);
            break;
        }
    }
    println!("{}", res);
    res.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let mut output = 0;
        for line in input.lines() {
            output += get_calibration(line);
        }
        assert_eq!(output, 142)
    }

    #[test]
    fn replace_digits() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let mut output = 0;
        for line in input.lines() {
            output += get_calibration_string(line);
        }
        assert_eq!(output, 281)
    }
}
