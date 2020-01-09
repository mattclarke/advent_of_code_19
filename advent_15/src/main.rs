use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn get_modes(command: i64, number: i64) -> Vec<i64> {
    let mut c: i64 = command / 100;
    let mut modes: Vec<i64> = Vec::new();

    for _i in 0..number {
        modes.push(c % 10);
        c = c / 10;
    }

    return modes;
}

fn get_operands(
    commands: &Vec<i64>,
    modes: &Vec<i64>,
    index: usize,
    relative_base: i64,
    number: i64,
) -> Vec<i64> {
    let mut operands: Vec<i64> = Vec::new();
    let mut temp_index = index + 1;

    for i in 0..number {
        if modes[i as usize] == 0 {
            // Position mode
            operands.push(commands[commands[temp_index] as usize]);
        } else if modes[i as usize] == 1 {
            // Parameter mode
            operands.push(commands[temp_index]);
        } else if modes[i as usize] == 2 {
            // Relative mode
            let first = commands[(relative_base + commands[temp_index]) as usize];
            operands.push(first);
        }
        temp_index += 1;
    }
    return operands;
}

fn get_write_index(commands: &Vec<i64>, mode: &i64, index: usize, relative_base: i64) -> usize {
    if *mode == 0 {
        // Position mode
        // println!("output = position {}", index);
        return commands[index] as usize;
    } else if *mode == 1 {
        // Parameter mode
        // println!("output = parameter {}", index);
        return commands[index] as usize;
    } else {
        // Relative mode
        return (relative_base + commands[index]) as usize;
    }
}

fn get_jump_index(commands: &Vec<i64>, mode: &i64, index: usize, relative_base: i64) -> usize {
    if *mode == 0 {
        // Position mode
        // println!("output = position {}", index);
        return commands[commands[index] as usize] as usize;
    } else if *mode == 1 {
        // // Parameter mode
        // println!("output = parameter {}", index);
        return commands[index] as usize;
    } else {
        // Relative mode
        return commands[(relative_base + commands[index]) as usize] as usize;
    }
}

fn run_computer(commands: &mut Vec<i64>, inputs: &mut Vec<i64>) -> Vec<i64> {
    return run_computer_index(commands, inputs, 0, 0).0;
}

fn run_computer_index(
    commands: &mut Vec<i64>,
    inputs: &mut Vec<i64>,
    mut index: i64,
    mut relative_base: i64,
) -> (Vec<i64>, i64, i64) {
    let mut output: Vec<i64> = Vec::new();

    while commands[index as usize] != 99 {
        let opcode = commands[index as usize] % 100;
        if opcode == 1 {
            // Addition
            let modes = get_modes(commands[index as usize], 3);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 3);
            let out_index =
                get_write_index(commands, &modes[2], (index + 3) as usize, relative_base);
            commands[out_index] = operands[0] + operands[1];
            index += 4;
        } else if opcode == 2 {
            // Multiply
            let modes = get_modes(commands[index as usize], 3);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 3);
            let out_index =
                get_write_index(commands, &modes[2], (index + 3) as usize, relative_base);
            commands[out_index] = operands[0] * operands[1];
            index += 4;
        } else if opcode == 3 {
            // Input
            // println!("Inputs");
            if inputs.len() == 0 {
                // println!("Out of inputs");
                return (output, index, relative_base);
            }
            let modes = get_modes(commands[index as usize], 1);
            let out_index =
                get_write_index(commands, &modes[0], (index + 1) as usize, relative_base);
            commands[out_index] = inputs[0];
            inputs.remove(0);
            index += 2;
        } else if opcode == 4 {
            // Output
            let modes = get_modes(commands[index as usize], 1);
            let out_index =
                get_jump_index(commands, &modes[0], (index + 1) as usize, relative_base);
            output.push(out_index as i64);
            index += 2;
            if output.len() == 3 {
                // println!("Output = {:?}", output);
                return (output, index, relative_base);
            }
        } else if opcode == 5 {
            // Jump-if-true
            let modes = get_modes(commands[index as usize], 2);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 1);
            let out_index =
                get_jump_index(commands, &modes[1], (index + 2) as usize, relative_base);
            if operands[0] != 0 {
                index = out_index as i64;
            } else {
                index += 3;
            }
        } else if opcode == 6 {
            // Jump-if-false
            let modes = get_modes(commands[index as usize], 2);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 1);
            let out_index =
                get_jump_index(commands, &modes[1], (index + 2) as usize, relative_base);
            if operands[0] == 0 {
                index = out_index as i64;
            } else {
                index += 3;
            }
        } else if opcode == 7 {
            // Less than
            let modes = get_modes(commands[index as usize], 3);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 2);
            let out_index =
                get_write_index(commands, &modes[2], (index + 3) as usize, relative_base);
            if operands[0] < operands[1] {
                commands[out_index] = 1;
            } else {
                commands[out_index] = 0;
            }
            index += 4;
        } else if opcode == 8 {
            // Equal
            let modes = get_modes(commands[index as usize], 3);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 2);
            let out_index =
                get_write_index(commands, &modes[2], (index + 3) as usize, relative_base);
            if operands[0] == operands[1] {
                commands[out_index] = 1;
            } else {
                commands[out_index] = 0;
            }
            index += 4;
        } else if opcode == 9 {
            // Adjust relative_base
            let modes = get_modes(commands[index as usize], 1);
            let operands = get_operands(commands, &modes, index as usize, relative_base, 1);
            relative_base += operands[0];
            index += 2;
        }
    }

    return (output, index, relative_base);
}

fn string_to_ints(str: String, sep: char) -> Vec<i64> {
    let parts = str.split(sep);
    let vec = parts.flat_map(|x| x.parse::<i64>()).collect::<Vec<i64>>();
    return vec;
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

struct Location {
    connections: Vec<i64>,
    visited: usize,
    is_oxygen: bool,
    count: i32,
}

fn get_x(x_pos: i32, direction: i64) -> i32 {
    if direction == 3 {
        return x_pos - 1;
    } else if direction == 4 {
        return x_pos + 1;
    }
    return x_pos;
}

fn get_y(y_pos: i32, direction: i64) -> i32 {
    if direction == 1 {
        return y_pos - 1;
    } else if direction == 2 {
        return y_pos + 1;
    }
    return y_pos;
}

fn get_backwards(direction: i64) -> i64 {
    if direction == 1 {
        return 2;
    } else if direction == 2 {
        return 1;
    } else if direction == 3 {
        return 4;
    } else if direction == 4 {
        return 3;
    }

    return -1;
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let orig_codes = string_to_ints(contents, ',');

    let mut memory = vec![0; orig_codes.len() * 100];
    for i in 0..orig_codes.len() {
        memory[i] = orig_codes[i];
    }

    let mut layout: HashMap<(i32, i32), Location> = HashMap::new();
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut index = 0;
    let mut relative_base = 0;
    let mut last_dir = 0;
    let mut inputs: Vec<i64> = vec![];
    let mut x_min = 0;
    let mut y_min = 0;
    let mut x_max = 0;
    let mut y_max = 0;

    let mut x_oxygen = 0;
    let mut y_oxygen = 0;

    let mut count = 0;

    // north (1), south (2), west (3), and east (4)

    // 0: The repair droid hit a wall. Its position has not changed.
    // 1: The repair droid has moved one step in the requested direction.
    // 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.

    while true {
        x_min = cmp::min(x_min, x_pos);
        x_max = cmp::max(x_max, x_pos);

        y_min = cmp::min(y_min, y_pos);
        y_max = cmp::max(y_max, y_pos);

        let current_square = layout.entry((x_pos, y_pos)).or_insert(Location {
            connections: Vec::new(),
            visited: 0,
            is_oxygen: false,
            count: std::i32::MAX,
        });

        if count < current_square.count {
            current_square.count = count;
        } else {
            count = current_square.count;
        }

        // Try all four directions except the one it came from
        for direction in 1..=4 {
            if last_dir != 0 && get_backwards(last_dir) == direction {
                // Skip over the one it came from
                continue;
            }

            inputs.push(direction);
            let mut raw_outputs =
                run_computer_index(&mut memory, &mut inputs, index, relative_base);
            index = raw_outputs.1;
            relative_base = raw_outputs.2;

            if raw_outputs.0[0] == 1 {
                if !current_square.connections.contains(&direction) {
                    current_square.connections.push(direction);
                }
            } else if raw_outputs.0[0] == 2 {
                if !current_square.connections.contains(&direction) {
                    current_square.connections.push(direction);
                }
                x_oxygen = get_x(x_pos, direction);
                y_oxygen = get_y(y_pos, direction);
            } else {
                // It is a wall
                continue;
            }

            // Go back
            inputs.push(get_backwards(direction));

            raw_outputs = run_computer_index(&mut memory, &mut inputs, index, relative_base);
            index = raw_outputs.1;
            relative_base = raw_outputs.2;
        }

        // Add the one it came from
        if last_dir != 0 {
            current_square.connections.push(get_backwards(last_dir));
        }

        // Move to one of the positions
        if current_square.visited < current_square.connections.len() {
            last_dir = current_square.connections[current_square.visited];
            inputs.push(last_dir);
            current_square.visited += 1;
            x_pos = get_x(x_pos, last_dir);
            y_pos = get_y(y_pos, last_dir);

            let raw_outputs = run_computer_index(&mut memory, &mut inputs, index, relative_base);
            index = raw_outputs.1;
            relative_base = raw_outputs.2;

            count += 1;
        }

        if x_pos == 0 && y_pos == 0 {
            break;
        }
    }

    assert!(layout.contains_key(&(x_oxygen, y_oxygen)));

    let mut oxy_square = layout.entry((x_oxygen, y_oxygen)).or_insert(Location {
        connections: Vec::new(),
        visited: 0,
        is_oxygen: false,
        count: 0,
    });
    oxy_square.is_oxygen = true;
    // Part 1 = 220
    println!("Part 1 = {}", oxy_square.count);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if x == 0 && y == 0 {
                print!("O");
            } else if x == x_oxygen && y == y_oxygen {
                print!("X");
            } else if layout.contains_key(&(x, y)) {
                print!(".")
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    // Part 2
    let mut queue: Vec<Vec<(i32, i32)>> = vec![vec![(x_oxygen, y_oxygen)]];
    let mut oxygenated: HashSet<(i32, i32)> = HashSet::new();
    oxygenated.insert((x_oxygen, y_oxygen));
    let mut time = 0;

    while queue.len() > 0 {
        let current_time = queue.remove(0);
        let mut new_items: Vec<(i32, i32)> = Vec::new();

        for coords in current_time {
            let curr_square = layout.entry(coords).or_insert(Location {
                connections: Vec::new(),
                visited: 0,
                is_oxygen: false,
                count: 0,
            });
            println!("num connections = {}", curr_square.connections.len());
            for conn in &curr_square.connections {
                let x = get_x(coords.0, *conn);
                let y = get_y(coords.1, *conn);
                if !oxygenated.contains(&(x, y)) {
                    new_items.push((x, y));
                    oxygenated.insert((x, y));
                }
            }
        }
        if new_items.len() > 0 {
            queue.push(new_items);
            time += 1;
        }

        // Stuff for printing it out
        // for y in y_min..=y_max {
        //     for x in x_min..=x_max {
        //         if oxygenated.contains(&(x,y)) {
        //             print!("O");
        //         } else if layout.contains_key(&(x, y)) {
        //             print!(".")
        //         } else {
        //             print!(" ");
        //         }
        //     }
        //     println!("");
        // }
        //
        // let stdin = io::stdin();
        // for line in stdin.lock().lines() {
        //     break;
        // }
    }

    // Part 2 = 334
    println!("Part 2 = {}", time);

    Ok(())
}

#[cfg(test)]
mod tests {
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
    fn it_works_for_day_2_example_1() {
        let mut cmds = vec![1, 0, 0, 0, 99];
        run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(cmds, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn it_works_for_day_2_example_2() {
        let mut cmds = vec![2, 3, 0, 3, 99];
        run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(cmds, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn it_works_for_day_2_example_3() {
        let mut cmds = vec![2, 4, 4, 5, 99, 0];
        run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(cmds, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn it_works_for_day_2_example_4() {
        let mut cmds = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(cmds, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn it_works_for_day_5_example_1() {
        let mut cmds = vec![3, 0, 4, 0, 99, 0];
        let mut inputs = vec![6];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result[0], 6);
    }

    #[test]
    fn it_works_for_day_5_example_2() {
        let mut cmds = vec![1002, 6, 3, 6, 4, 6, 33, 0];
        let mut inputs = vec![99];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result[0], 99);
    }

    #[test]
    fn it_works_for_day_5_example_3() {
        let mut cmds = vec![1101, 100, -1, 6, 4, 6, 0, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result[0], 99);
    }

    #[test]
    fn it_works_for_no_leading_value() {
        let mut cmds = vec![101, 93, 3, 6, 4, 6, 0, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result[0], 99);
    }

    #[test]
    fn it_jumps_on_a_five_if_true() {
        let mut cmds = vec![4, 1, 1105, 1, 7, 4, 2, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn no_jump_on_a_five_if_false() {
        let mut cmds = vec![4, 1, 1105, 0, 7, 4, 2, 99, 0];

        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1, 1105]);
    }

    #[test]
    fn it_jumps_on_a_six_if_false() {
        let mut cmds = vec![4, 1, 1106, 0, 7, 4, 2, 99, 0];

        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn no_jump_on_a_six_if_true() {
        let mut cmds = vec![4, 1, 1106, 1, 7, 4, 2, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1, 1106]);
    }

    #[test]
    fn seven_if_true_stores_one() {
        let mut cmds = vec![1107, 1, 2, 5, 4, 5, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn seven_if_false_stores_zero() {
        let mut cmds = vec![1107, 2, 1, 5, 4, 5, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1107]);
    }

    #[test]
    fn eight_if_true_stores_one() {
        let mut cmds = vec![1108, 1, 1, 5, 4, 5, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn eight_if_false_stores_zero() {
        let mut cmds = vec![1108, 2, 1, 5, 4, 5, 99, 0];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1108]);
    }

    #[test]
    fn it_works_for_example_4_a() {
        let mut cmds = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![8];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_4_b() {
        let mut cmds = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_5_a() {
        let mut cmds = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![7];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_5_b() {
        let mut cmds = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_6_a() {
        let mut cmds = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, 0];
        let mut inputs = vec![8];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_6_b() {
        let mut cmds = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, 0];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_7_a() {
        let mut cmds = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99, 0];
        let mut inputs = vec![7];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_7_b() {
        let mut cmds = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99, 0];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_8_a() {
        let mut cmds = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_8_b() {
        let mut cmds = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_9_a() {
        let mut cmds = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_9_b() {
        let mut cmds = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_10_a() {
        let mut cmds = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut inputs = vec![0];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn it_works_for_example_10_b() {
        let mut cmds = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut inputs = vec![123];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works_for_example_11_a() {
        let mut cmds = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut inputs = vec![7];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![999]);
    }

    #[test]
    fn it_works_for_example_11_b() {
        let mut cmds = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut inputs = vec![8];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1000]);
    }

    #[test]
    fn it_works_for_example_11_c() {
        let mut cmds = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut inputs = vec![9];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![1001]);
    }

    #[test]
    fn it_works_for_simple_relative_example_1() {
        let mut cmds = vec![22201, 7, 8, 9, 4, 9, 99, 100, 101, 0];
        let mut inputs = vec![];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![201]);
    }

    #[test]
    fn it_works_for_simple_relative_example_2() {
        let mut cmds = vec![22201, 7, 8, 9, 204, 9, 99, 100, 101, 0];
        let mut inputs = vec![];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![201]);
    }

    #[test]
    fn it_works_for_simple_relative_example_3() {
        let mut cmds = vec![109, 0, 22201, 9, 10, 11, 204, 11, 99, 100, 101, 0];
        let mut inputs = vec![];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![201]);
    }

    #[test]
    fn it_works_for_simple_relative_example_4() {
        let mut cmds = vec![109, 2, 22201, 7, 8, 9, 204, 9, 99, 100, 101, 0];
        let mut inputs = vec![];
        let result = run_computer(&mut cmds, &mut inputs);

        assert_eq!(result, vec![201]);
    }

    #[test]
    fn it_works_for_day_9_example_1() {
        let original = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut cmds = vec![0; 300];
        for i in 0..original.len() {
            cmds[i] = original[i];
        }

        let result = run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(result, original);
    }

    #[test]
    fn it_works_for_day_9_example_2() {
        let original = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let mut cmds = vec![0; 100];
        for i in 0..original.len() {
            cmds[i] = original[i];
        }

        let result = run_computer(&mut cmds, &mut Vec::new());

        // Must be 16 digits
        assert!(result[0] > 1000000000000000);
        assert!(result[0] < 10000000000000000)
    }

    #[test]
    fn it_works_for_day_9_example_3() {
        let original = vec![104, 1125899906842624, 99];

        let mut cmds = vec![0; 1000];
        for i in 0..original.len() {
            cmds[i] = original[i];
        }

        let result = run_computer(&mut cmds, &mut Vec::new());

        assert_eq!(result[0], original[1]);
    }
}
