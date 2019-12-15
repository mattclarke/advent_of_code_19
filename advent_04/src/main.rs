fn calculate(data: String) -> bool {
    let numbers = data
        .chars()
        .flat_map(|x| x.to_digit(10))
        .collect::<Vec<u32>>();
    let mut prev: u32 = *numbers.first().unwrap();
    let mut equal_count = 0;
    let mut seen_double = false;

    for i in 1..numbers.len() {
        let n = numbers[i];
        if n < prev {
            return false;
        }

        if n == prev {
            equal_count += 1;
        } else {
            if equal_count == 1 {
                seen_double = true;
            } else {
                equal_count = 0;
            }
        }

        prev = n;
    }

    return seen_double || equal_count == 1;
}

fn main() -> std::io::Result<()> {
    let mut curr = 193651;
    let mut count = 0;

    while curr <= 649729 {
        if calculate(curr.to_string()) {
            count += 1;
        }
        curr += 1;
    }

    println!("{}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn fails_if_not_ascending() {
        let input = String::from("223450");

        assert_eq!(calculate(input), false);
    }

    #[test]
    fn fails_if_no_double() {
        let input = String::from("123789");

        assert_eq!(calculate(input), false);
    }

    #[test]
    fn fails_if_does_not_contain_pure_double_1() {
        let input = String::from("111111");

        assert_eq!(calculate(input), false);
    }

    #[test]
    fn fails_if_does_not_contain_pure_double_2() {
        let input = String::from("123444");

        assert_eq!(calculate(input), false);
    }

    #[test]
    fn valid_example_passes_1() {
        let input = String::from("122345");

        assert_eq!(calculate(input), true);
    }

    #[test]
    fn valid_example_passes_2() {
        let input = String::from("112233");

        assert_eq!(calculate(input), true);
    }

    #[test]
    fn valid_example_passes_3() {
        let input = String::from("111122");

        assert_eq!(calculate(input), true);
    }
}
