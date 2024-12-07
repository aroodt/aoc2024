use std::collections::BinaryHeap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (BinaryHeap<u32>, BinaryHeap<u32>) {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks_exact(2)
        .for_each(|locs| {
            left.push(locs[0].parse::<u32>().expect("bad number"));
            right.push(locs[1].parse::<u32>().expect("bad number"));
        });

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut leftlocs, mut rightlocs) = parse(input);
    let mut delta = 0;
    while let (Some(l), Some(r)) = (leftlocs.pop(), rightlocs.pop()) {
        delta += l.abs_diff(r)
    }
    Some(delta)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut leftlocs, mut rightlocs) = parse(input);
    let mut simscore = 0;
    while leftlocs.len() != 0 {
        let val = leftlocs.pop().unwrap();
        let mut lcount = 1;
        let mut rcount = 0;

        while let Some(&lval) = leftlocs.peek() {
            if lval == val {
                lcount += 1;
                leftlocs.pop();
            } else {
                break;
            }
        }

        while let Some(&rval) = rightlocs.peek() {
            if rval < val {
                break;
            } else if rval == val {
                rcount += 1;
                rightlocs.pop();
            } else {
                rightlocs.pop();
                continue;
            }
        }

        simscore += val * lcount * rcount;
    }
    Some(simscore)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
