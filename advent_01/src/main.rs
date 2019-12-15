use std::fs::File;
use std::io::prelude::*;

fn calculate_fuel(mass: i32) -> i32 {
    let result = (mass / 3) - 2;
    return result;
}

fn calculate_fuel_plus_fuel_mass(mass: i32) -> i32 {
    let mut result = 0;
    let mut curr_mass = mass;

    loop {
        curr_mass = calculate_fuel(curr_mass);
        if curr_mass > 0 {
            result += curr_mass;
        } else {
            break;
        }
    }

    return result;
}

fn string_to_ints(str: String, sep: char) -> Vec<i32> {
    let parts = str.split(sep);
    let vec = parts.flat_map(|x| x.parse::<i32>()).collect::<Vec<i32>>();
    return vec;
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let ints = string_to_ints(contents, '\n');
    // Part 1
    // let result = ints
    //     .iter()
    //     .map(|&x| calculate_fuel(x))
    //     .collect::<Vec<i32>>()
    //     .iter()
    //     .sum::<i32>();

    // Part2
    let result = ints
        .iter()
        .fold(0, |result, &x| result + calculate_fuel_plus_fuel_mass(x));

    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate_fuel;
    use crate::calculate_fuel_plus_fuel_mass;
    use crate::string_to_ints;

    #[test]
    fn it_works_for_example_1() {
        let result = calculate_fuel(12);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_for_example_2() {
        let result = calculate_fuel(14);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_for_example_3() {
        let result = calculate_fuel(1969);
        assert_eq!(result, 654);
    }

    #[test]
    fn it_works_for_example_4() {
        let result = calculate_fuel(100756);
        assert_eq!(result, 33583);
    }

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = string_to_ints(String::from("100756\n123\n"), '\n');
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 100756);
        assert_eq!(result[1], 123);
    }

    #[test]
    fn it_works_for_example_1_recv() {
        let result = calculate_fuel_plus_fuel_mass(14);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_for_example_2_recv() {
        let result = calculate_fuel_plus_fuel_mass(1969);
        assert_eq!(result, 966);
    }

    #[test]
    fn it_works_for_example_3_recv() {
        let result = calculate_fuel_plus_fuel_mass(100756);
        assert_eq!(result, 50346);
    }
}
