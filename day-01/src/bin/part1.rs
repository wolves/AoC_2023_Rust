fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut vals: Vec<u32> = Vec::new();
    for line in input.lines() {
        dbg!(line);
        let mut nums: Vec<u32> = Vec::new();
        for char in line.chars() {
            if char.is_ascii_digit() {
                nums.push(char.to_digit(10).unwrap());
            }
        }

        if nums.len() >= 2 {
            let result = [nums[0], *nums.last().unwrap()]
                .iter()
                .map(|&num| num.to_string())
                .collect::<String>();
            vals.push(result.parse::<u32>().unwrap());
        } else {
            let result = [nums[0], nums[0]]
                .iter()
                .map(|&num| num.to_string())
                .collect::<String>();
            vals.push(result.parse::<u32>().unwrap());
        }
    }

    let result: u32 = vals.iter().sum();
    dbg!(result);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142".to_string())
    }
}
