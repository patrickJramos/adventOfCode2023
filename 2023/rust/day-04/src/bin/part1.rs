fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line
                .split(':')
                .skip(1)
                .next()
                .expect("a valid game")
                .split("|");

            let good_nums: Vec<&str> = split
                .next()
                .expect("a valid game")
                .split_whitespace()
                .collect();
            let my_nums = split.next().expect("a valid game").split_whitespace();

            let matches = my_nums.filter(|num| good_nums.contains(num)).count();

            (matches != 0)
                .then(|| 2_usize.pow((matches - 1) as u32))
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }
}
