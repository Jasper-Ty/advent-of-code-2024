advent_of_code::solution!(3);

use std::str;

pub fn part_one(input: &str) -> Option<u32> {
    let mut bytes = input.as_bytes();
    let mut sum = 0;

    let b10 = |slice| str::from_utf8(slice).unwrap().parse::<u32>().unwrap();

    loop {
        match bytes {
            [b'm', b'u', b'l', args @ ..] => {
                match args {
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=3]);
                        bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=4]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=4]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=6]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=6]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=7]); bytes = rest;
                    }
                    _ => { bytes = args; }
                }
            }
            [_, rest @ ..] => {
                bytes = rest;
            },
            [] => break,
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bytes = input.as_bytes();
    let mut sum = 0;
    let mut parsemul = true;

    let b10 = |slice| str::from_utf8(slice).unwrap().parse::<u32>().unwrap();

    loop {
        match bytes {
            [b'd', b'o', b'(', b')', rest @ ..] => {
                parsemul = true;
                bytes = rest;
            },
            [b'd', b'o', b'n', b'\'', b't', b'(', b')', rest @ ..] => {
                parsemul = false;
                bytes = rest;
            },
            [b'm', b'u', b'l', args @ ..] if parsemul => {
                match args {
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=3]);
                        bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=4]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=1])*b10(&args[3..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=4]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=2])*b10(&args[4..=6]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=5]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=6]); bytes = rest;
                    }
                    [b'(', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b',', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')', rest @ ..] => {
                        sum += b10(&args[1..=3])*b10(&args[5..=7]); bytes = rest;
                    }
                    _ => { bytes = args; }
                }
            }
            [_, rest @ ..] => {
                bytes = rest;
            },
            [] => break,
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
