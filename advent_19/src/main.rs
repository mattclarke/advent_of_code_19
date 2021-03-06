use std::char;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
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
            // if output.len() == 3 {
            //     // println!("Output = {:?}", output);
            //     return (output, index, relative_base);
            // }
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

fn print_board_pt_1(board: &HashMap<(i32, i32), char>, x_max: i32, y_max: i32) -> i32 {
    let mut part_1 = 0;
    for y in 0..y_max {
        for x in 0..=x_max {
            // println!("{}, {}", x, y);
            let value = board[&(x, y)];
            if value == '#' {
                if x > 0 && x < x_max && y > 0 && y < y_max {
                    if board[&(x - 1, y)] == '#'
                        && board[&(x + 1, y)] == '#'
                        && board[&(x, y - 1)] == '#'
                        && board[&(x, y + 1)] == '#'
                    {
                        print!("O");
                        part_1 += x * y;
                        continue;
                    }
                }
            }
            print!("{}", value);
        }
        println!("");
    }
    return part_1;
}

fn part_1(orig_codes: Vec<i64>) {
    let mut count = 0;

    for y in 0..=49 {
        for x in 0..=49 {
            let mut memory = orig_codes.to_vec();
            let mut inputs: Vec<i64> = vec![x, y];
            let index = 0;
            let relative_base = 0;
            let output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
            if output.0[0] == 1 {
                count += 1;
            }
        }
    }

    println!("Part 1 = {}", count);
}

fn do_line(orig_codes: &Vec<i64>, start_x: i64, y: i64) -> (i64, i64) {
    loop {
        let mut found = false;
        let mut count = 0;
        let mut x = start_x;
        let mut first_start = 0;
        let mut first_end = 0;

        loop {
            let mut memory = orig_codes.to_vec();
            let mut inputs: Vec<i64> = vec![x, y];
            let index = 0;
            let relative_base = 0;
            let output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
            if output.0[0] == 1 {
                if !found {
                    found = true;
                    first_start = x;
                    count += 1;
                }
            } else {
                if found {
                    first_end = x;
                    break;
                }
            }
            x += 1;
        }
        return (first_start, first_end);
    }
}

fn print_line(first_y: i64, x: i64, start: i64, end: i64) {
    print!("{}    ", first_y);
    for i in x..=end+5 {
        if i < start || i >= end {
            print!(".");
        }
        else {
            print!("#");
        }
    }
    println!();
}

fn part_2(orig_codes: Vec<i64>) {
    let mut first_y: i64 = 1020;
    let mut final_x = 0;
    let mut final_y = 0;
    let mut first = true;
    let mut x = 100;

    loop {
        let results = do_line(&orig_codes, x, first_y);
        if first {
            first = false;
            x = results.0 - 5;
        }

        print_line(first_y, x,results.0, results.1);

        let results2 = do_line(&orig_codes, x, first_y + 99);
        print_line(first_y + 100, x,results2.0, results2.1);

        println!("{} {} {} {}", results.0, results.1, results2.0, results2.1);
        if results.1 - results2.0 == 100 {
            println!("{}", results2.0);
            final_x = results2.0;
            final_y = first_y;
            break;
        }
        first_y += 1;
    }

    println!("Part 2 = {}", final_x * 10000 + final_y);
}

fn main() -> std::io::Result< () > {
let contents = read_file();
let orig_codes = string_to_ints(contents, ',');

let mut memory = vec ! [0; orig_codes.len() * 100];
for i in 0..orig_codes.len() {
memory[i] = orig_codes[i];
}

// Part 1 = 131
// part_1(memory);

// Part 2 = 15231022
part_2(memory);

Ok(())
}
