use itertools::Itertools;

type Level<'a> = &'a [i32];

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let levels = contents
        .lines()
        .map(|line| line.split(" ").map(|number| number.parse::<i32>().unwrap()))
        .map(|numbers| numbers.collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let good_levels_count = levels
        .into_iter()
        .map(|level| is_level_increasing(&level) || is_level_decreasing(&level))
        .fold(0, |acc, b| if b { acc + 1 } else { acc });

    println!("Good levels = {}", good_levels_count);
}

fn is_level_increasing(level: Level) -> bool {
    for (a, b) in level.into_iter().tuple_windows() {
        let diff = (b - a).abs();

        if b < a || (diff > 3 || diff < 1) {
            return false;
        }
    }
    true
}

fn is_level_decreasing(level: Level) -> bool {
    for (a, b) in level.into_iter().tuple_windows() {
        let diff = (a - b).abs();

        if b > a || (diff > 3 || diff < 1) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_level_is_increasing() {
        let increasing = [1, 2, 4, 5, 6];

        assert!(is_level_increasing(&increasing))
    }

    #[test]
    fn check_if_level_is_decreasing() {
        let increasing: Vec<i32> = (8..1).collect();

        assert!(is_level_decreasing(&increasing))
    }

    #[test]
    fn check_if_level_is_decreasing_correctly() {
        let increasing = [6, 2, 1];

        assert!(!is_level_decreasing(&increasing))
    }
}
