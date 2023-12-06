use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn get_winning_numbers() -> usize {
    let path = Path::new("src/day4/numbers.txt");
    let file = File::open(&path);
    let reader = BufReader::new(file.unwrap());

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut splitter = line.split(":");
        let card_number = splitter.next().unwrap();
        let all_numbers = splitter.next().unwrap();

        let mut splitter = all_numbers.split("|");
        let my_numbers = splitter.next().unwrap();
        let lucky_numbers = splitter.next().unwrap();

        let vec_my_numbers: Vec<&str> = my_numbers.split_whitespace().collect();
        let vec_lucky_numbers: Vec<&str> = lucky_numbers.split_whitespace().collect();

        let my_lucky_numbers: Vec<i32> = vec_my_numbers
            .iter()
            .filter(|x| vec_lucky_numbers.contains(x))
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let count = my_lucky_numbers.len() as i32;
        let mut power = 0;

        if count > 0 {
            power = 2_i32.pow((count - 1).try_into().unwrap());
        }

        sum += power;
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winning_numbers() {
        assert_eq!(get_winning_numbers(), 23441);
    }
}
