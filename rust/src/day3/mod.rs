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
    let mut sum = 0;
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x].is_digit(10) && !visited[y][x] {
                let num = extract_number(&lines, &mut visited, x, y);
                if num != 0 {
                    sum += num;
                }
            }
        }
    }

    sum as i32
}

fn extract_number(lines: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut num = 0;
    let mut nx = x;

    while nx < lines[y].len() && lines[y][nx].is_digit(10) {
        num = num * 10 + lines[y][nx].to_digit(10).unwrap();
        visited[y][nx] = true;
        nx += 1;
    }

    if is_adjacent_to_symbol(lines, x, y, nx - 1) {
        num
    } else {
        0
    }
}

fn is_adjacent_to_symbol(lines: &Vec<Vec<char>>, x: usize, y: usize, end_x: usize) -> bool {
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];

    for i in 0..8 {
        for nx in x..=end_x {
            let adj_x = nx as i32 + dx[i];
            let adj_y = y as i32 + dy[i];

            if adj_x >= 0
                && adj_y >= 0
                && adj_x < lines[0].len() as i32
                && adj_y < lines.len() as i32
            {
                if lines[adj_y as usize][adj_x as usize] != '.'
                    && !lines[adj_y as usize][adj_x as usize].is_digit(10)
                {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_engine_schematic() {
        assert_eq!(sum_engine_schematic(), 0);
    }
}
