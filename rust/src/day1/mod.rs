use std::{path::Path, fs::File, io::{Read, BufReader, BufRead}, collections::HashMap};

fn calibrate() -> i32 {
    let mut total = 0;

    let file_path = Path::new("src/day1/calibration-document.txt");
    let mut file = match File::open(&file_path) {
        Err(err) => panic!("couldn't open {}: {}", file_path.display(), err),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("couldn't read {}: {}", file_path.display(), err);
    }


    for line in contents.lines() {
        let mut digits = vec![];

        for char in line.chars() {
            if char.is_numeric() {
                digits.push(char);
            }
        }

        let combined_numbers = format!("{}{}", digits[0], digits[digits.len() - 1]);
        let combined_numbers: i32 = combined_numbers.parse().unwrap();

        total += combined_numbers;
    }

    total
}

fn calibrate_v2() -> i32 {
    let file_path = Path::new("src/day1/calibration-document.txt");
    let file = File::open(&file_path).expect("couldn't open file");
    let reader = BufReader::new(file);
    
    reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|vec| {
            10 * vec[0] + vec[vec.len() - 1]
        })
        .sum::<u32>() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calibrate() {
        assert_eq!(calibrate(), 55477);
    }

    #[test]
    fn test_calibrate_v2() {
        assert_eq!(calibrate_v2(), 55477);
    }
}
