use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let re = Regex::new("[0-9]{1}").unwrap();
    let input_iter = input.split('\n').into_iter();
    let mut sum: u32 = 0;
    for line_option in input_iter.enumerate() {
        let line = line_option.1;
        let matches = re.captures_iter(line).map(|x| x.extract::<0>().0).collect::<Vec<&str>>();
        let lowest_match = matches.first().unwrap();
        let highest_match = matches.last().unwrap();
        sum += format!("{}{}", lowest_match, highest_match).parse::<u32>().unwrap();
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
