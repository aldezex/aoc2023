use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn sum_engine_schematic() -> i32 {
    let path = Path::new("src/day3/engine.txt");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut sum_gear_ratios = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '*' {
                let gear_ratio = calculate_gear_ratio(&lines, x, y);
                if gear_ratio != 0 {
                    sum_gear_ratios += gear_ratio;
                }
            }
        }
    }

    sum_gear_ratios as i32
}

fn calculate_gear_ratio(lines: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut part_numbers = Vec::new();

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < lines[0].len()
                && (ny as usize) < lines.len()
                && lines[ny as usize][nx as usize].is_digit(10)
            {
                let num = extract_number(&lines, nx as usize, ny as usize);
                if !part_numbers.contains(&num) {
                    part_numbers.push(num);
                }
            }
        }
    }

    if part_numbers.len() == 2 {
        part_numbers[0] * part_numbers[1]
    } else {
        0
    }
}

fn extract_number(lines: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut num = 0;
    let mut nx = x;

    while nx < lines[y].len() && lines[y][nx].is_digit(10) {
        num = num * 10 + lines[y][nx].to_digit(10).unwrap();
        nx += 1;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_engine_schematic() {
        assert_eq!(sum_engine_schematic(), 0);
    }
}
