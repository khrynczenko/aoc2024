fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let vec: Vec<(i32, i32)> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split("   ").take(2).collect::<Vec<&str>>())
        .map(|pairs| {
            (
                pairs[0].parse::<i32>().unwrap(),
                pairs[1].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let (mut lhs, mut rhs): (Vec<i32>, Vec<i32>) = vec.into_iter().unzip();
    lhs.sort();
    rhs.sort();

    let result = calculate_total_distance(&lhs, &rhs);
    println!("{}", result);

    lhs.dedup();
    let result = find_similarities(&lhs, &rhs);
    println!("{}", result);
}

fn calculate_total_distance(lhs: &[i32], rhs: &[i32]) -> i32 {
    lhs.iter()
        .zip(rhs)
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn find_similarities(lhs: &[i32], rhs: &[i32]) -> usize {
    lhs.iter()
        .map(|unique| (unique, rhs.iter().filter(|r| **r == *unique).count()))
        .map(|(unique, count)| *unique as usize * count)
        .sum()
}
