advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines().count();
    let n = input.lines().next().unwrap().len();

    let mut total = 0;

    let arr: Vec<u8> = input.lines()
        .flat_map(str::bytes)
        .collect();

    for i in 0..m {
        for j in 0..n {
            let c = arr[i*m + j];
            if j <= n - 4 {
                let r1 = arr[i*m + (j+1)];
                let r2 = arr[i*m + (j+2)];
                let r3 = arr[i*m + (j+3)];
                match (c, r1, r2, r3) {
                    (b'X', b'M', b'A', b'S')
                        | (b'S', b'A', b'M', b'X') => { total += 1 }
                    _ => {}
                }
            }
            if i <= n - 4 {
                let d1 = arr[(i+1)*m + j];
                let d2 = arr[(i+2)*m + j];
                let d3 = arr[(i+3)*m + j];
                match (c, d1, d2, d3) {
                    (b'X', b'M', b'A', b'S')
                        | (b'S', b'A', b'M', b'X') => { total += 1 }
                    _ => {}
                }
            }
            if j <= n - 4 && i <= n - 4 {
                let dd1 = arr[(i+1)*m + (j+1)];
                let dd2 = arr[(i+2)*m + (j+2)];
                let dd3 = arr[(i+3)*m + (j+3)];
                match (c, dd1, dd2, dd3) {
                    (b'X', b'M', b'A', b'S')
                        | (b'S', b'A', b'M', b'X') => { total += 1 }
                    _ => {}
                }
            }
            if j >= 3 && i <= n - 4 {
                let dd1 = arr[(i+1)*m + (j-1)];
                let dd2 = arr[(i+2)*m + (j-2)];
                let dd3 = arr[(i+3)*m + (j-3)];
                match (c, dd1, dd2, dd3) {
                    (b'X', b'M', b'A', b'S')
                        | (b'S', b'A', b'M', b'X') => { total += 1 }
                    _ => {}
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = input.lines().count();
    let n = input.lines().next().unwrap().len();

    let mut total = 0;

    let arr: Vec<u8> = input.lines()
        .flat_map(str::bytes)
        .collect();

    for i in 1..m-1 {
        for j in 1..n-1 {
            let c = arr[i*m + j];
            let ul = arr[(i-1)*m + (j-1)];
            let ur = arr[(i-1)*m + (j+1)];
            let bl = arr[(i+1)*m + (j-1)];
            let br = arr[(i+1)*m + (j+1)];

            match (c, ul, ur, bl, br) {
                (b'A', b'M', b'M', b'S', b'S')
                | (b'A', b'S', b'S', b'M', b'M')
                | (b'A', b'M', b'S', b'M', b'S')
                | (b'A', b'S', b'M', b'S', b'M') => { total += 1 }
                _ => {}
            }
        }
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
