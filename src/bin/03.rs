advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                let mut total = 0;
                for cap in re.captures_iter(line) {
                    total += cap[1].parse::<u32>().unwrap()
                        * cap[2].parse::<u32>().unwrap()
                }
                total
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re =
        Regex::new(r"(?:do\(\))|(?:don't\(\))|(?:mul\((\d+),(\d+)\))").unwrap();
    let mut enabled = true;
    Some(
        input
            .lines()
            .map(|line| {
                let mut total = 0;
                for cap in re.captures_iter(line) {
                    match &cap[0] {
                        "do()" => enabled = true,
                        "don't()" => enabled = false,
                        _ => {
                            if enabled {
                                total += cap[1].parse::<u32>().unwrap()
                                    * cap[2].parse::<u32>().unwrap()
                            }
                        }
                    }
                }
                total
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
