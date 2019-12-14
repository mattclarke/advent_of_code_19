use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn calculate_max(perms: &HashSet<Vec<i32>>, commands: &mut Vec<i32>) -> i32 {
    let mut max_thrust: i32 = 0;

    for p in perms {
        let ans_a = run_computer(commands, vec![p[0], 0]);

        // println!("a = {}", ans_a);

        let ans_b = run_computer(commands, vec![p[1], ans_a]);
        // println!("b = {}", ans_b);

        let ans_c = run_computer(commands, vec![p[2], ans_b]);
        // println!("c = {}", ans_c);

        let ans_d = run_computer(commands, vec![p[3], ans_c]);
        // println!("d = {}", ans_d);

        let ans_e = run_computer(commands, vec![p[4], ans_d]);
        // println!("d = {}", ans_e);
        max_thrust = cmp::max(max_thrust, ans_e);
    }

    return max_thrust;
}

fn run_computer(commands: &mut Vec<i32>, inputs: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut output = 0;
    let mut input_index = 0;

    while commands[index] != 99 {
        let opcode = commands[index] % 100;
        if opcode == 3 {
            // 3 is different because it is a write
            // no parameter modes apply
            let first = commands[index + 1];
            commands[first as usize] = inputs[input_index];
            input_index += 1;
            index += 2;
        } else if opcode == 4 {
            let c: i32 = commands[index] / 100;
            if c % 10 == 1 {
                output = commands[index + 1];
            } else {
                let first = commands[index + 1];
                output = commands[first as usize];
            }
            index += 2;
        } else {
            let mut c: i32 = commands[index] / 100;
            let mut operands: Vec<i32> = Vec::new();
            index += 1;

            for _i in 0..2 {
                if c % 10 == 1 {
                    operands.push(commands[index])
                } else {
                    operands.push(commands[commands[index] as usize]);
                }
                c = c / 10;
                index += 1;
            }

            let out_par = commands[index] as usize;

            if opcode == 1 {
                commands[out_par] = operands.iter().fold(0, |acc, x| acc + x);
                index += 1;
            } else if opcode == 2 {
                commands[out_par] = operands.iter().fold(1, |acc, x| acc * x);
                index += 1;
            } else if opcode == 5 {
                if operands[0] != 0 {
                    index = operands[1] as usize;
                }
            } else if opcode == 6 {
                if operands[0] == 0 {
                    index = operands[1] as usize;
                }
            } else if opcode == 7 {
                if operands[0] < operands[1] {
                    commands[out_par] = 1;
                } else {
                    commands[out_par] = 0;
                }
                index += 1;
            } else if opcode == 8 {
                if operands[0] == operands[1] {
                    commands[out_par] = 1;
                } else {
                    commands[out_par] = 0;
                }
                index += 1;
            }
        }
    }

    return output;
}

fn recursive_combo(input: &Vec<i32>, builder: &mut Vec<i32>, results: &mut HashSet<Vec<i32>>) {
    if builder.len() == input.len() {
        results.insert(builder.to_vec());
        return;
    }

    for i in input {
        if !builder.contains(&i) {
            builder.push(*i);
            recursive_combo(input, builder, results);
            builder.pop();
        }
    }
}

fn get_perms(input: &Vec<i32>) -> HashSet<Vec<i32>> {
    let mut results: HashSet<Vec<i32>> = HashSet::new();
    let mut builder: Vec<i32> = Vec::new();
    recursive_combo(input, &mut builder, &mut results);

    return results;
}

fn string_to_ints(str: String, sep: char) -> Vec<i32> {
    let parts = str.split(sep);
    let vec = parts
        .flat_map(|x| x.parse::<i32>())
        .collect::<Vec<i32>>();
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
    let mut orig_codes = string_to_ints(contents, ',');

    // Part 1 = 
    let perms = get_perms(&vec![0,1,2,3,4]);
    println!("Result 1 = {}", calculate_max(&perms, &mut orig_codes));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate_max;
    use crate::get_perms;
    use crate::run_computer;
    use crate::string_to_ints;

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = string_to_ints(String::from("1,0,0,0,99"), ',');
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1);
        assert_eq!(result[4], 99);
    }

    #[test]
    fn can_generate_all_combos_1() {
        let phase_settings = vec![0, 1, 2];
        let result = get_perms(&phase_settings);
        println!("{:?}", result);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn can_generate_all_combos_2() {
        let phase_settings = vec![0, 1, 2, 3, 4];
        let result = get_perms(&phase_settings);
        println!("{:?}", result);
        assert_eq!(result.len(), 120);
    }

    #[test]
    fn it_works_for_example_1_hard_coded() {
        let mut input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let ans_a = run_computer(&mut input, vec![4, 0]);

        println!("a = {}", ans_a);

        let ans_b = run_computer(&mut input, vec![3, ans_a]);
        println!("b = {}", ans_b);

        let ans_c = run_computer(&mut input, vec![2, ans_b]);
        println!("c = {}", ans_c);

        let ans_d = run_computer(&mut input, vec![1, ans_c]);
        println!("d = {}", ans_d);

        let ans_e = run_computer(&mut input, vec![0, ans_d]);
        println!("d = {}", ans_e);

        assert_eq!(ans_e, 43210);
    }

    #[test]
    fn it_works_for_example_1() {
        let mut input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phase_settings = vec![0, 1, 2, 3, 4];
        let perms = get_perms(&phase_settings);

        assert_eq!(calculate_max(&perms, &mut input), 43210);
    }

    #[test]
    fn it_works_for_example_2() {
        let mut input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phase_settings = vec![0, 1, 2, 3, 4];
        let perms = get_perms(&phase_settings);

        assert_eq!(calculate_max(&perms, &mut input), 54321);
    }

    #[test]
    fn it_works_for_example_3() {
        let mut input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phase_settings = vec![0, 1, 2, 3, 4];
        let perms = get_perms(&phase_settings);

        assert_eq!(calculate_max(&perms, &mut input), 65210);
    }
}
