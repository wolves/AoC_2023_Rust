fn process(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";
        assert_eq!("", process(input))
    }
}
