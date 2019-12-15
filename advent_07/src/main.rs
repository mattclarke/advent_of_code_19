use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn calculate_max(perms: &HashSet<Vec<i32>>, commands: &Vec<i32>) -> i32 {
    let mut max_thrust: i32 = 0;
    let mut cmds = commands.to_vec();

    for p in perms {
        let ans_a = run_computer(&mut cmds, vec![p[0], 0]);
        let ans_b = run_computer(&mut cmds, vec![p[1], ans_a.0]);
        let ans_c = run_computer(&mut cmds, vec![p[2], ans_b.0]);
        let ans_d = run_computer(&mut cmds, vec![p[3], ans_c.0]);
        let ans_e = run_computer(&mut cmds, vec![p[4], ans_d.0]);
        max_thrust = cmp::max(max_thrust, ans_e.0);
    }

    return max_thrust;
}

fn calculate_max_feedback(perms: &HashSet<Vec<i32>>, commands: &Vec<i32>) -> i32 {
    let mut max_thrust: i32 = 0;

    for p in perms {
        let mut a_state = commands.to_vec();
        let mut b_state = commands.to_vec();
        let mut c_state = commands.to_vec();
        let mut d_state = commands.to_vec();
        let mut e_state = commands.to_vec();

        // First time through the loop preps the system
        let mut ans_a = run_computer(&mut a_state, vec![p[0], 0]);
        let mut ans_b = run_computer(&mut b_state, vec![p[1], ans_a.0]);
        let mut ans_c = run_computer(&mut c_state, vec![p[2], ans_b.0]);
        let mut ans_d = run_computer(&mut d_state, vec![p[3], ans_c.0]);
        let mut ans_e = run_computer(&mut e_state, vec![p[4], ans_d.0]);

        while ans_e.2 != 99 {
            ans_a = run_computer_index(&mut a_state, vec![ans_e.0], ans_a.1);
            ans_b = run_computer_index(&mut b_state, vec![ans_a.0], ans_b.1);
            ans_c = run_computer_index(&mut c_state, vec![ans_b.0], ans_c.1);
            ans_d = run_computer_index(&mut d_state, vec![ans_c.0], ans_d.1);
            ans_e = run_computer_index(&mut e_state, vec![ans_d.0], ans_e.1);
        }

        max_thrust = cmp::max(max_thrust, ans_e.0);
    }

    return max_thrust;
}

fn run_computer(commands: &mut Vec<i32>, inputs: Vec<i32>) -> (i32, usize, i32) {
    return run_computer_index(commands, inputs, 0);
}

fn run_computer_index(
    commands: &mut Vec<i32>,
    mut inputs: Vec<i32>,
    start_index: usize,
) -> (i32, usize, i32) {
    let mut index = start_index;
    let mut output = 0;

    while commands[index] != 99 {
        let opcode = commands[index] % 100;
        if opcode == 3 {
            // 3 is different because it is a write
            // no parameter modes apply
            let first = commands[index + 1];
            if inputs.len() == 0 {
                // Part 2 - suspend
                return (output, index, commands[index]);
            }
            commands[first as usize] = inputs[0];
            inputs.remove(0);
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

    return (output, 0, commands[index]);
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
    let orig_codes = string_to_ints(contents, ',');

    // Part 1 = 116680
    let perms = get_perms(&vec![0, 1, 2, 3, 4]);
    println!("Result 1 = {}", calculate_max(&perms, &orig_codes));

    // Part 2 = 89603079
    let perms = get_perms(&vec![5, 6, 7, 8, 9]);
    println!("Result 2 = {}", calculate_max_feedback(&perms, &orig_codes));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate_max;
    use crate::calculate_max_feedback;
    use crate::get_perms;
    use crate::run_computer;
    use crate::run_computer_index;
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
        let ans_b = run_computer(&mut input, vec![3, ans_a.0]);
        let ans_c = run_computer(&mut input, vec![2, ans_b.0]);
        let ans_d = run_computer(&mut input, vec![1, ans_c.0]);
        let ans_e = run_computer(&mut input, vec![0, ans_d.0]);

        assert_eq!(ans_e.0, 43210);
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

    #[test]
    fn it_works_for_part2_example_1_hard_coded() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        // let phase_settings = vec![5, 6, 7, 8, 9];
        // let perms = get_perms(&phase_settings);

        let mut a_state = input.to_vec();
        let mut ans_a = run_computer(&mut a_state, vec![9, 0]);
        let mut b_state = input.to_vec();
        let mut ans_b = run_computer(&mut b_state, vec![8, ans_a.0]);
        let mut c_state = input.to_vec();
        let mut ans_c = run_computer(&mut c_state, vec![7, ans_b.0]);
        let mut d_state = input.to_vec();
        let mut ans_d = run_computer(&mut d_state, vec![6, ans_c.0]);
        let mut e_state = input.to_vec();
        let mut ans_e = run_computer(&mut e_state, vec![5, ans_d.0]);

        while ans_e.2 != 99 {
            ans_a = run_computer_index(&mut a_state, vec![ans_e.0], ans_a.1);
            ans_b = run_computer_index(&mut b_state, vec![ans_a.0], ans_b.1);
            ans_c = run_computer_index(&mut c_state, vec![ans_b.0], ans_c.1);
            ans_d = run_computer_index(&mut d_state, vec![ans_c.0], ans_d.1);
            ans_e = run_computer_index(&mut e_state, vec![ans_d.0], ans_e.1);
        }

        assert_eq!(ans_e.0, 139629729);
    }

    #[test]
    fn it_works_for_part2_example_1() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_settings = vec![5, 6, 7, 8, 9];
        let perms = get_perms(&phase_settings);

        assert_eq!(calculate_max_feedback(&perms, &input), 139629729);
    }

    #[test]
    fn it_works_for_part2_example_2() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase_settings = vec![5, 6, 7, 8, 9];
        let perms = get_perms(&phase_settings);

        assert_eq!(calculate_max_feedback(&perms, &input), 18216);
    }
}
