use std::char;
use std::collections::VecDeque;
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


fn part_1(orig_codes: &Vec<i64>) {
    let mut memories: Vec<Vec<i64>> = vec![];
    let mut indexs: Vec<i64> = vec![];
    let mut bases: Vec<i64> = vec![];
    let mut queues: Vec<VecDeque<(i64, i64)>> = vec![];

    for _i in 0..50 {
        queues.push(VecDeque::new());
    }

    // Set up the individual computers
    for i in 0..50 {
        let mut memory = orig_codes.to_vec();
        let mut inputs: Vec<i64> = vec![i];
        let mut index = 0;
        let mut relative_base = 0;
        let output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
        index = output.1;
        relative_base = output.2;

        memories.push(memory.to_vec());
        indexs.push(index);
        bases.push(relative_base);
    }

    loop {
        for i in 0..50 {
            let mut memory = memories[i as usize].to_vec();
            let mut index = indexs[i as usize];
            let mut relative_base = bases[i as usize];
            let mut inputs: Vec<i64> = vec![-1];
            let mut output: (Vec<i64>, i64, i64);

            if queues[i as usize].len() == 0 {
                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;
            } else {
                let msg = queues[i as usize].pop_front().unwrap();
                inputs  = vec![msg.0];

                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;

                inputs.push(msg.1);
                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;
            }

            if output.0.len() == 3 {
                let addr =  output.0[0];
                let x = output.0[1];
                let y = output.0[2];

                if addr == 255 {
                    println!("Part 1 = {}", y);
                    return;
                }

                queues[addr as usize].push_back((x, y));
            }

            memories[i as usize] = memory.to_vec();
            indexs[i as usize] = index;
            bases[i as usize] = relative_base;
        }
    }
}


fn part_2(orig_codes: &Vec<i64>) {
    let mut memories: Vec<Vec<i64>> = vec![];
    let mut indexes: Vec<i64> = vec![];
    let mut bases: Vec<i64> = vec![];
    let mut queues: Vec<VecDeque<(i64, i64)>> = vec![];
    let mut states: Vec<bool> = Vec::new();
    let mut nat: (i64, i64) = (-1, -1);
    let mut old_nat = (-1, -1);

    for _i in 0..50 {
        queues.push(VecDeque::new());
        states.push(false);
    }

    // Set up the individual computers
    for i in 0..50 {
        let mut memory = orig_codes.to_vec();
        let mut inputs: Vec<i64> = vec![i];
        let mut index = 0;
        let mut relative_base = 0;
        let output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
        index = output.1;
        relative_base = output.2;

        memories.push(memory.to_vec());
        indexes.push(index);
        bases.push(relative_base);
    }

    loop {
        // Check NAT
        if !states.contains(&false) && nat.0 != -1 {
            println!("IDLE!");

            if nat.1  == old_nat.1{
                println!("Part 2 = {}", nat.1);
                return;
            }

            queues[0].push_back(nat);
            old_nat = nat;
        }

        for i in 0..50 {
            let mut memory = memories[i as usize].to_vec();
            let mut index = indexes[i as usize];
            let mut relative_base = bases[i as usize];
            let mut inputs: Vec<i64> = vec![-1];
            let mut output: (Vec<i64>, i64, i64);

            if queues[i as usize].len() == 0 {
                states[i] = true;
                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;
            } else {
                states[i] = false;
                let msg = queues[i as usize].pop_front().unwrap();
                inputs  = vec![msg.0];

                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;

                inputs.push(msg.1);
                output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
                index = output.1;
                relative_base = output.2;
            }

            if output.0.len() == 3 {
                let addr =  output.0[0];
                let x = output.0[1];
                let y = output.0[2];

                if addr == 255 {
                    nat = (x, y);
                }
                else {
                    queues[addr as usize].push_back((x, y));
                }
            }

            memories[i as usize] = memory.to_vec();
            indexes[i as usize] = index;
            bases[i as usize] = relative_base;
        }
    }
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let orig_codes = string_to_ints(contents, ',');

    let mut memory = vec![0; orig_codes.len() * 100];
    for i in 0..orig_codes.len() {
        memory[i] = orig_codes[i];
    }

    // Part 1 = 23815
    part_1(&memory);

    // Part 2 = 16666
    part_2(&memory);

    Ok(())
}
