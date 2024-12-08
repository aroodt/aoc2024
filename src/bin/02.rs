advent_of_code::solution!(2);

fn is_safe(vec: Vec<u32>) -> bool {
    if vec.len() <= 1 {
        return true;
    }
    let order = vec[0].cmp(&vec[1]);
    for vals in vec.windows(2) {
        let l = vals[0];
        let r = vals[1];
        if l.cmp(&r) != order || !((1..=3).contains(&l.abs_diff(r))) {
            return false;
        }
    }
    true
}

fn is_safe_str(line: &str) -> bool {
    let nums = line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    is_safe(nums)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|x| is_safe_str(x) as u32).sum())
}

fn is_safe_with_removal(mut vec: Vec<u32>, idx: usize) -> bool {
    vec.remove(idx);
    is_safe(vec)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|x| {
                let nums = x
                    .split_whitespace()
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (is_safe(nums.clone())
                    || (0..nums.len())
                        .any(|x| is_safe_with_removal(nums.clone(), x)))
                    as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one("1 2 3 4 5 6");
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_p1_l1() {
        let result = part_one("7 6 4 2 1");
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_p1_l2() {
        let result = part_one("1 2 7 8 9");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_p1_l3() {
        let result = part_one("9 7 6 2 1");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_p1_l4() {
        let result = part_one("1 3 2 4 5");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_p1_l5() {
        let result = part_one("8 6 4 4 1");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_p1_l6() {
        let result = part_one("1 3 6 7 9");
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
