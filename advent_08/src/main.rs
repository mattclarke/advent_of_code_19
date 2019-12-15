use std::fs::File;
use std::io::prelude::*;

fn find_layers(input: &Vec<usize>, width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut layers: Vec<Vec<usize>> = Vec::new();

    let mut entry: Vec<usize> = Vec::new();
    for i in input {
        if entry.len() < width * height {
            entry.push(*i);
        } else {
            layers.push(entry);
            entry = vec![*i];
        }
    }

    layers.push(entry);

    return layers;
}

fn find_total_for_layer_with_least_zeros(layers: &Vec<Vec<usize>>) -> usize {
    let mut min_zeros = 10000000;
    let mut result = 0;
    for r in layers {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        let mut a = r.to_vec();
        a.sort_unstable();
        for i in &a {
            if *i == 0 {
                zeros += 1;
            } else if *i == 1 {
                ones += 1;
            } else if *i == 2 {
                twos += 1;
            } else {
                break;
            }
        }
        if zeros < min_zeros {
            min_zeros = zeros;
            result = ones * twos;
        }
    }
    return result;
}

fn compress_the_layers(layers: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = layers[0].to_vec();

    for layer in &layers[1..] {
        for (i, v) in layer.iter().enumerate() {
            if result[i] == 2 {
                result[i] = *v;
            }
        }
    }

    return result;
}

fn string_to_ints(s: String) -> Vec<usize> {
    let vec = s
        .chars()
        .flat_map(|x| x.to_string().parse::<usize>())
        .collect::<Vec<usize>>();
    return vec;
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn print_layer(layer: &Vec<usize>, width: usize) {
    for (i, v) in layer.iter().enumerate() {
        if i % width == 0 {
            println!("");
        }
        if *v == 1 {
            print!("*");
        } else {
            print!(" ");
        }
    }
    println!("");
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let data = string_to_ints(contents);
    let layers = find_layers(&data, 25, 6);
    // Part 1 = 828
    let result_1 = find_total_for_layer_with_least_zeros(&layers);
    println!("Part 1 = {}", result_1);
    // Part 2 = ZLBJF
    let result_2 = compress_the_layers(&layers);
    print_layer(&result_2, 25);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::compress_the_layers;
    use crate::find_layers;
    use crate::find_total_for_layer_with_least_zeros;
    use crate::string_to_ints;

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = string_to_ints(String::from("123456"));
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 1);
        assert_eq!(result[5], 6);
    }

    #[test]
    fn it_works_for_example_1() {
        let input = String::from("123456789012");
        let ints = string_to_ints(input);

        let result = find_layers(&ints, 3, 2);
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(result[1], vec![7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn which_row_has_the_most_zeroes() {
        let input = String::from("120006700112");
        let data = string_to_ints(input);

        let layers = find_layers(&data, 3, 2);
        let result = find_total_for_layer_with_least_zeros(&layers);

        assert_eq!(result, 2);
    }

    #[test]
    fn compress_the_layers_works() {
        let input = String::from("0222112222120000");
        let data = string_to_ints(input);

        let layers = find_layers(&data, 2, 2);
        let result = compress_the_layers(&layers);

        assert_eq!(result, vec![0, 1, 1, 0]);
    }
}
