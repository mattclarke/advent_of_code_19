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

fn part_1(mut memory: &mut Vec<i64>) {
    let mut layout: HashMap<(i32, i32), char> = HashMap::new();
    let mut x_pos = 0;
    let mut y_pos = 0;
    let index = 0;
    let relative_base = 0;
    let mut inputs: Vec<i64> = vec![];
    let mut x_max = 0;
    let mut y_max = 0;

    let output = run_computer_index(&mut memory, &mut inputs, index, relative_base);

    for value in output.0 {
        if value == 46 {
            layout.insert((x_pos, y_pos), '.');
        } else if value == 35 {
            layout.insert((x_pos, y_pos), '#');
        } else if value == 10 {
            x_pos = 0;
            y_max = cmp::max(y_max, y_pos);
            y_pos += 1;
            continue;
        } else if value == 88 {
            layout.insert((x_pos, y_pos), 'X');
        } else if value == 60 {
            layout.insert((x_pos, y_pos), '<');
        } else if value == 62 {
            layout.insert((x_pos, y_pos), '>');
        } else if value == 94 {
            layout.insert((x_pos, y_pos), '^');
        } else if value == 118 {
            layout.insert((x_pos, y_pos), 'v');
        }
        x_max = cmp::max(x_max, x_pos);
        x_pos += 1;
    }
    let part_1 = print_board_pt_1(&layout, x_max, y_max);
    println!("Part 1 = {} {}", part_1, y_max);
}

fn do_direction(
    layout: &HashMap<(i32, i32), char>,
    direction: (i32, i32),
    x_robot: i32,
    y_robot: i32,
    moves: &mut Vec<String>,
    x_max: i32,
    y_max: i32,
) -> (i32, i32) {
    if direction == (0, -1) {
        if x_robot < x_max && layout[&(x_robot + 1, y_robot)] == '#' {
            moves.push(String::from("R"));
            return (1, 0);
        } else if x_robot > 0 && layout[&(x_robot - 1, y_robot)] == '#' {
            moves.push(String::from("L"));
            return (-1, 0);
        }
    } else if direction == (1, 0) {
        if y_robot > 0 && layout[&(x_robot, y_robot - 1)] == '#' {
            moves.push(String::from("L"));
            return (0, -1);
        } else if y_robot < y_max && layout[&(x_robot, y_robot + 1)] == '#' {
            moves.push(String::from("R"));
            return (0, 1);
        }
    } else if direction == (0, 1) {
        if x_robot > 0 && layout[&(x_robot - 1, y_robot)] == '#' {
            moves.push(String::from("R"));
            return (-1, 0);
        } else if x_robot < x_max && layout[&(x_robot + 1, y_robot)] == '#' {
            moves.push(String::from("L"));
            return (1, 0);
        }
    } else if direction == (-1, 0) {
        if y_robot > 0 && layout[&(x_robot, y_robot - 1)] == '#' {
            moves.push(String::from("R"));
            return (0, -1);
        } else if y_robot < y_max && layout[&(x_robot, y_robot + 1)] == '#' {
            moves.push(String::from("L"));
            return (0, 1);
        }
    }

    return (0, 0);
}

fn check_a(moves: &Vec<String>) -> Vec<i32> {
    let mut assigned: Vec<i32> = vec![0; moves.len()];

    let start = 0;
    let mut length = 4;
    while length < 10 {
        for i in start..length {
            assigned[i] = 1;
        }
        let ref_slice = &moves[start..length];

        // Check for reoccurances
        let mut index = length;
        while index + length < moves.len() {
            let slice = &moves[index..index + length];
            // Check assigned is 0 for whole slice
            let ass_slice = &assigned[index..index + length];
            let mut okay = true;
            for i in ass_slice {
                if *i != 0 {
                    okay = false;
                    break;
                }
            }
            if okay && slice == ref_slice {
                // println!("Slice slice baby A");
                for i in index..index + length {
                    assigned[i] = 1;
                }
            }
            index += 2;
        }
        // println!("{:?}", assigned);

        // Check b
        let ans = check_b(&moves, assigned);
        if ans.len() > 0 {
            return ans;
        }

        // Wipe it
        assigned = vec![0; moves.len()];
        length += 2;
    }
    return vec![];
}

fn check_b(moves: &Vec<String>, assigned: Vec<i32>) -> Vec<i32> {
    let mut assigned_local = assigned.to_vec();
    let mut length = 2;

    // Find first 0
    let mut start = 0;
    for i in 0..assigned_local.len() {
        if assigned[i] == 0 {
            start = i;
            break;
        }
    }
    // println!("start b {}", start);
    if start == 0 {
        // Didn't find anywhere to start = fail
        return vec![];
    }

    while length < 10 {
        for i in start..start + length {
            if assigned_local[i] != 0 {
                // Overwriting  = fail
                return vec![];
            }
            assigned_local[i] = 2;
        }
        // println!("local {:?}", assigned_local);
        let ref_slice = &moves[start..start + length];

        // Check for reoccurances
        let mut index = start + length;
        while index + length < moves.len() {
            let slice = &moves[index..index + length];
            // Check assigned is 0 for whole slice
            let ass_slice = &assigned_local[index..index + length];
            let mut okay = true;
            for i in ass_slice {
                if *i != 0 {
                    okay = false;
                    break;
                }
            }
            if okay && slice == ref_slice {
                // println!("Slice slice baby B");
                for i in index..index + length {
                    assigned_local[i] = 2;
                }
            }
            index += 2;
        }

        // println!("{:?}", assigned_local);
        // Check c
        let ans = check_c(&moves, assigned_local);
        if ans.len() > 0 {
            return ans;
        }

        // Wipe it
        assigned_local = assigned.to_vec();
        length += 2;
    }
    return vec![];
}

fn check_c(moves: &Vec<String>, assigned: Vec<i32>) -> Vec<i32> {
    let mut assigned_local = assigned.to_vec();
    let mut length = 2;
    // Find first 0
    let mut start = 0;
    for i in 0..assigned_local.len() {
        if assigned[i] == 0 {
            start = i;
            break;
        }
    }
    // println!("start {}", start);
    if start == 0 {
        // Didn't find anywhere to start = fail
        return vec![];
    }

    while length < 10 {
        for i in start..start + length {
            if assigned_local[i] != 0 {
                // Overwriting is bad
                return vec![];
            }
            assigned_local[i] = 3;
        }
        // println!("local {:?}", assigned_local);
        let ref_slice = &moves[start..start + length];

        // Check for reoccurances
        let mut index = start + length;
        while index + length <= moves.len() {
            if assigned_local[index] != 0 {
                index += 2;
                continue;
            }
            let slice = &moves[index..index + length];
            if ref_slice == slice {
                // println!("looking for {:?} found {:?}", ref_slice,slice);
            }
            // Check assigned is 0 for whole slice
            let ass_slice = &assigned_local[index..index + length];
            let mut okay = true;
            for i in ass_slice {
                if *i != 0 {
                    okay = false;
                    break;
                }
            }
            if okay && slice == ref_slice {
                for i in index..index + length {
                    assigned_local[i] = 3;
                }
            }
            index += 2;
        }

        if !assigned_local.contains(&(0 as i32)) {
            return assigned_local;
        }

        // Wipe it
        assigned_local = assigned.to_vec();
        length += 2;
    }
    return vec![];
}

fn part_2(orig_codes: Vec<i64>) {
    let mut memory = vec![0; orig_codes.len() * 100];
    for i in 0..orig_codes.len() {
        memory[i] = orig_codes[i];
    }
    // Activate robot
    memory[0] = 2;

    let mut layout: HashMap<(i32, i32), char> = HashMap::new();
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut index = 0;
    let mut relative_base = 0;
    let mut x_robot = 0;
    let mut y_robot = 0;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut direction = (0, -1);

    let mut inputs: Vec<i64> = vec![];

    let mut output = run_computer_index(&mut memory, &mut inputs, index, relative_base);

    for value in output.0 {
        if value == 46 {
            layout.insert((x_pos, y_pos), '.');
        } else if value == 35 {
            layout.insert((x_pos, y_pos), '#');
        } else if value == 10 {
            x_pos = 0;
            y_max = cmp::max(y_max, y_pos);
            y_pos += 1;
            continue;
        } else if value == 88 {
            layout.insert((x_pos, y_pos), 'X');
        } else if value == 60 {
            layout.insert((x_pos, y_pos), '<');
            x_robot = x_pos;
            y_robot = y_pos;
            direction = (-1, 0);
        } else if value == 62 {
            layout.insert((x_pos, y_pos), '>');
            x_robot = x_pos;
            y_robot = y_pos;
            direction = (1, 0);
        } else if value == 94 {
            layout.insert((x_pos, y_pos), '^');
            x_robot = x_pos;
            y_robot = y_pos;
            direction = (0, -1);
        } else if value == 118 {
            layout.insert((x_pos, y_pos), 'v');
            x_robot = x_pos;
            y_robot = y_pos;
            direction = (0, 1);
        }
        x_max = cmp::max(x_max, x_pos);
        x_pos += 1;
    }
    print_board_pt_1(&layout, x_max, y_max - 1);

    // Calculate path - assume straight ahead on junctions
    let mut moves: Vec<String> = vec![];

    // Orientate
    direction = do_direction(
        &layout, direction, x_robot, y_robot, &mut moves, x_max, y_max,
    );

    while direction != (0, 0) {
        // Walk until nothing ahead
        let mut count = 0;

        while layout.contains_key(&(x_robot + direction.0, y_robot + direction.1))
            && layout[&(x_robot + direction.0, y_robot + direction.1)] == '#'
        {
            // moves.push('=');
            count += 1;
            x_robot += direction.0;
            y_robot += direction.1;
        }
        moves.push(count.to_string());

        // Turn
        direction = do_direction(
            &layout, direction, x_robot, y_robot, &mut moves, x_max, y_max,
        );
    }

    println!("{:?}", moves);

    // Clever bit goes here
    // A = "R", "12", "R", "4", "R", "10", "R", "12"
    // B = "R", "6", "L", "8", "R", "10"
    // C = "L", "8", "R", "4", "R", "4", "R", "6"
    // Ans = A, B, A, C, A, B, C, A, B, C
    // Worked out by hand - can we compute it? Yes we can!
    let raw_result = check_a(&moves);
    println!("{:?}", raw_result);

    let mut a_start = 0;
    let mut a_stop = 0;
    let mut b_start = 0;
    let mut b_stop = 0;
    let mut c_start = 0;
    let mut c_stop = 0;

    for i in 0..raw_result.len() {
        if raw_result[i] != 1 {
            a_stop = i;
            break;
        }
    }

    for i in 0..raw_result.len() {
        if raw_result[i] == 2 {
            if b_start == 0 {
                b_start = i;
            }
        } else {
            if b_start > 0 {
                b_stop = i;
                break;
            }
        }
    }

    for i in 0..raw_result.len() {
        if raw_result[i] == 3 {
            if c_start == 0 {
                c_start = i;
            }
        } else {
            if c_start > 0 {
                c_stop = i;
                break;
            }
        }
    }

    println!("A = {:?}", &moves[a_start..a_stop]);
    println!("B = {:?}", &moves[b_start..b_stop]);
    println!("C = {:?}", &moves[c_start..c_stop]);

    inputs = vec![];
    for r in raw_result {
        let mut alpha = 65;
        if r == 2 {
            alpha = 66;
        } else if r == 3 {
            alpha = 67;
        }
        if inputs.len() == 0 {
            inputs.push(alpha);
            inputs.push(44);
        } else if inputs[inputs.len() - 2] != alpha {
            inputs.push(alpha);
            inputs.push(44);
        }
    }
    let mut end = inputs.len() - 1;
    inputs[end] = 10;
    println!("{:?}", inputs);

    // Reset the memory etc.
    memory = vec![0; orig_codes.len() * 100];
    for i in 0..orig_codes.len() {
        memory[i] = orig_codes[i];
    }
    // Activate robot
    memory[0] = 2;

    // Enter pattern

    output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
    index = output.1;
    relative_base = output.2;
    println!("===============");
    for i in output.0 {
        let c: u8 = i as u8;
        print!("{}", c as char);
    }
    println!("===============");

    // Enter A
    let mut A: String = String::from("");

    for a in &moves[a_start..a_stop] {
        A += a;
        A += ",";
    }

    inputs = vec![];
    for b in A.as_bytes() {
        inputs.push(*b as i64);
    }
    end = inputs.len() - 1;
    inputs[end] = 10;

    output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
    index = output.1;
    relative_base = output.2;
    println!("===============");
    for i in output.0 {
        let c: u8 = i as u8;
        print!("{}", c as char);
    }
    println!("===============");

    // Enter B
    let mut B: String = String::from("");

    for a in &moves[b_start..b_stop] {
        B += a;
        B += ",";
    }

    inputs = vec![];
    for b in B.as_bytes() {
        inputs.push(*b as i64);
    }
    end = inputs.len() - 1;
    inputs[end] = 10;

    output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
    index = output.1;
    relative_base = output.2;
    println!("===============");
    for i in output.0 {
        let c: u8 = i as u8;
        print!("{}", c as char);
    }
    println!("===============");

    // Enter C
    let mut C: String = String::from("");

    for a in &moves[c_start..c_stop] {
        C += a;
        C += ",";
    }

    inputs = vec![];
    for b in C.as_bytes() {
        inputs.push(*b as i64);
    }
    end = inputs.len() - 1;
    inputs[end] = 10;

    output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
    index = output.1;
    relative_base = output.2;
    println!("===============");
    for i in output.0 {
        let c: u8 = i as u8;
        print!("{}", c as char);
    }
    println!("===============");

    inputs = vec![110, 10];
    output = run_computer_index(&mut memory, &mut inputs, index, relative_base);
    println!("{:?}", output.0);
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let orig_codes = string_to_ints(contents, ',');

    let mut memory = vec![0; orig_codes.len() * 100];
    for i in 0..orig_codes.len() {
        memory[i] = orig_codes[i];
    }

    // Part 1 = 5680
    part_1(&mut memory);

    // Part 2 = 895965
    part_2(orig_codes);

    Ok(())
}
