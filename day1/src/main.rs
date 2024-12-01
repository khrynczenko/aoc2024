fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let vec: Vec<(i32, i32)> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("   ")
                .take(2)
                .map(|split| split.to_string())
                .collect::<Vec<String>>()
        })
        .map(|pairs| {
            (
                pairs[0].parse::<i32>().unwrap(),
                pairs[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    vec.into_iter().for_each(|(l, r)| {
        lhs.push(l);
        rhs.push(r);
    });
    let result = calculate_total_distance(&lhs, &rhs);
    println!("{}", result);

    let result = find_similarities(&lhs, &rhs);
    println!("{}", result);
}

fn calculate_total_distance(lhs: &[i32], rhs: &[i32]) -> i32 {
    let mut sorted_lhs = lhs.to_vec();
    sorted_lhs.sort();
    let mut sorted_rhs = rhs.to_vec();
    sorted_rhs.sort();

    sorted_lhs
        .into_iter()
        .zip(sorted_rhs)
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn find_similarities(lhs: &[i32], rhs: &[i32]) -> usize {
    let mut lhs_deduped = lhs.to_vec();
    lhs_deduped.dedup();

    lhs_deduped
        .into_iter()
        .map(|unique| (unique, rhs.iter().filter(|r| **r == unique).count()))
        .map(|(unique, count)| unique as usize * count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distances_are_coorect() {
        let left = [3, 5, 6];
        let right = [1, 5, 2];

        assert_eq!(calculate_total_distance(&left, &right), 6);
    }
}
