fn get_winning_races() -> u32 {
    let input = String::from("Time:        53     71     78     80
        Distance:   275   1181   1215   1524");

    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    let mut sum = 1;

    // This is done for fun, we can create the vector of tuples and have less fun
    for line in input.lines() {
        let mut splitter = line.split(":");
        let predicate = splitter.next().unwrap();
        let data = splitter.next().unwrap();

        if predicate.contains("Time") {
            for time in data.split_whitespace() {
                times.push(time.parse::<i32>().unwrap());
            }
        } else if predicate.contains("Distance") {
            for distance in data.split_whitespace() {
                distances.push(distance.parse::<i32>().unwrap());
            }
        }
    }

    let zipped = times.iter().zip(distances.iter());
    for (time, distance) in zipped {
        let mut count = 0;

        for t in 0..*time {
            let speed = t;
            let travel_time = time - t;
            let travel_distance = speed * travel_time;

            if travel_distance > *distance {
                count += 1;
            }
        }

        sum *= count;
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winning_secret() {
        assert_eq!(get_winning_races(), 449820);
    }
}
