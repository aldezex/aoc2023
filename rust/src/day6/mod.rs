fn get_winning_races() -> u32 {
    let input = String::from("
        Time:        53     71     78     80
        Distance:   275   1181   1215   1524
    ");

    for line in input.lines() {
        println!("{}", line);
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winning_secret() {
        assert_eq!(get_winning_races(), 0);
    }
}
