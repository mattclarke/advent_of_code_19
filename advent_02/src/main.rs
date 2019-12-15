use std::fs::File;
use std::io::prelude::*;

fn calculate(mut vec: Vec<usize>) -> Vec<usize> {
    let mut index = 0;
    loop {
        if vec[index] == 1 {
            let first = vec[vec[index + 1]];
            let second = vec[vec[index + 2]];
            let out_par = vec[index + 3];

            vec[out_par] = first + second;
        } else if vec[index] == 2 {
            let first = vec[vec[index + 1]];
            let second = vec[vec[index + 2]];
            let out_par = vec[index + 3];

            vec[out_par] = first * second;
        } else if vec[index] == 99 {
            break;
        }
        index += 4;
    }

    return vec;
}

fn string_to_ints(str: String, sep: char) -> Vec<usize> {
    let parts = str.split(sep);
    let vec = parts
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();
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
    let orig_codes = string_to_ints(contents, ',');
    // Part 1
    // codes[1] = 12;
    // codes[2] = 2;
    // let result = calculate(codes);
    // println!("{}", result[0]);

    // Part 2
    let mut found = false;
    for n in 0..=99 {
        for v in 0..=99 {
            let mut codes = orig_codes.clone();
            codes[1] = n;
            codes[2] = v;
            let result = calculate(codes);
            if result[0] == 19690720 {
                println!("{}", 100 * n + v);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate;
    use crate::string_to_ints;

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = string_to_ints(String::from("1,0,0,0,99"), ',');
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1);
        assert_eq!(result[4], 99);
    }

    #[test]
    fn it_works_for_example_1() {
        let input = vec![1, 0, 0, 0, 99];

        assert_eq!(calculate(input), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn it_works_for_example_2() {
        let input = vec![2, 3, 0, 3, 99];

        assert_eq!(calculate(input), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn it_works_for_example_3() {
        let input = vec![2, 4, 4, 5, 99, 0];

        assert_eq!(calculate(input), [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn it_works_for_example_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        assert_eq!(calculate(input), [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
