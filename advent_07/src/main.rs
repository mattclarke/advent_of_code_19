use std::fs::File;
use std::io::prelude::*;

fn calculate(mut vec: Vec<usize>) -> Vec<usize> {

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
}
