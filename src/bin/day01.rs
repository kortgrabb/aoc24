fn iter_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        left_list.push(numbers.next().unwrap().parse().unwrap());
        right_list.push(numbers.next().unwrap().parse().unwrap());
    }

    // sort the lists
    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn part1(input: &str) -> String {
    let (left_list, right_list) = iter_lists(input);

    let mut total = 0;
    for i in 0..left_list.len() {
        let left = left_list[i];
        let right = right_list[i];

        let diff = (right - left).abs();
        total += diff;
    }

    total.to_string()
}

fn part2(input: &str) -> String {
    let (left_list, right_list) = iter_lists(input);

    let mut total = 0;
    for left in &left_list {
        let mut occurences = 0;

        for right in &right_list {
            if right == left {
                occurences += 1;
            }
        }

        total += occurences * left;
    }

    total.to_string()
}

fn main() {
    let input = include_str!("../../input/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "";
        assert_eq!(part1(test_input), "Not implemented");
    }

    #[test]
    fn test_part2() {
        let test_input = "";
        assert_eq!(part2(test_input), "Not implemented");
    }
}
