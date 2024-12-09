advent_of_code::solution!(7);

#[derive(Debug)]
struct Eqn {
    target: usize,
    operands: Vec<usize>,
}
impl Eqn {
    fn from_str(line: &str) -> Option<Self> {
        let mut iter = line.split(' ');
        let target_str = iter.next()?;

        let target = target_str[..target_str.len()-1]
            .parse::<usize>().ok()?;
        let operands = iter
            .map(|s| s.parse::<usize>().ok())
            .collect::<Option<Vec<usize>>>()?;

        Some(Eqn {
            target,
            operands,
        })
    }

    /// Use DFS to traverse *backwards*.
    /// We attempt to either subtract or divide
    /// at each level until we reach zero.
    fn is_satisfiable(&self) -> bool {
        let mut stack = vec![(self.operands.len(), self.target)];

        loop {
            if let Some((depth, curr)) = stack.pop() {
                if depth == 0 && curr == 0 {
                    // We found our answer
                    break true
                } else if depth > 0 && curr != 0 {
                    // Otherwise, keep trying
                    let op = self.operands[depth - 1];
                    if let (next, 0) = (curr / op , curr % op)  {
                        stack.push((depth - 1, next));
                    }
                    if let Some(next) = curr.checked_sub(op) {
                        stack.push((depth - 1, next));
                    }
                }
            } else {
                // If stack is empty, couldn't find answer
                break false
            }
        }
    }

    /// Same idea
    fn is_satisfiable_2(&self) -> bool {
        let mut stack = vec![(self.operands.len(), self.target)];

        loop {
            if let Some((depth, curr)) = stack.pop() {
                if depth == 0 && curr == 0 {
                    // We found our answer
                    break true
                } else if depth > 0 && curr != 0 {
                    // Otherwise, keep trying
                    let op = self.operands[depth - 1];
                    if let (next, 0) = (curr / op , curr % op)  {
                        stack.push((depth - 1, next));
                    }
                    if let Some(next) = curr.checked_sub(op) {
                        stack.push((depth - 1, next));
                    }
                    let pow10 = 10usize.pow(op.ilog10() + 1);
                    if curr % pow10 == op {
                        stack.push((depth - 1, curr / pow10));
                    }
                }
            } else {
                // If stack is empty, couldn't find answer
                break false
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input.lines()
        .filter_map(Eqn::from_str)
        .filter(Eqn::is_satisfiable)
        .map(|eqn| eqn.target as usize)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input.lines()
        .filter_map(Eqn::from_str)
        .filter(Eqn::is_satisfiable_2)
        .map(|eqn| eqn.target as usize)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
