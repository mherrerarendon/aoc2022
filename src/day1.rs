use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    let mut max_calories = 0;
    let mut lines = input.lines();
    while let Some(_) = lines.next() {
        let elf_calories = lines
            .take_while_ref(|line| !line.is_empty())
            .map(|line| line.parse::<usize>().unwrap())
            .sum();
        max_calories = max_calories.max(elf_calories);
    }
    max_calories
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_name() {
        let solution = solve(&fs::read_to_string("./input_data/day1.txt").unwrap());
        assert_eq!(solution, 67633);
    }
}
