advent_of_code::solution!(2);

fn is_good<T>(iter: T) -> bool
where
    T: Clone + Iterator<Item = i64>,
{
    let diffs = iter.clone().zip(iter.skip(1)).map(|(a, b)| b - a);

    let is_ascending = diffs.clone().all(|x| x > 0);

    let is_descending = diffs.clone().all(|x| x < 0);

    let is_monotone = is_ascending || is_descending;

    let is_bounded = diffs.clone().all(|x| x.abs() <= 3);

    is_monotone && is_bounded
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let report = line.split(' ').filter_map(|s| s.parse::<i64>().ok());

        total += is_good(report) as u32;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let report = line
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        let is_good = (0..report.len()).any(|i| {
            let report_iter = report
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|it| *it.1);

            is_good(report_iter)
        });

        total += is_good as u32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
