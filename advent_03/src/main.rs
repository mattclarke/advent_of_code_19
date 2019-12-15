use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_positions_visited(moves: Vec<String>) -> HashMap<(i32, i32), i32> {
    let mut positions = HashMap::<(i32, i32), i32>::new();
    let mut x_position = 0;
    let mut y_position = 0;
    let mut steps = 0;
    positions.insert((x_position, y_position), steps);
    let null_op = |x: i32| x;
    let increase = |x: i32| x + 1;
    let decrease = |x: i32| x - 1;
    let mut x_op: Box<dyn Fn((i32)) -> (i32)> = Box::new(null_op);
    let mut y_op: Box<dyn Fn((i32)) -> (i32)> = Box::new(null_op);

    for m in moves {
        let pair = decode_move(m);
        if pair.0 == 'U' {
            x_op = Box::new(null_op);
            y_op = Box::new(increase);
        } else if pair.0 == 'D' {
            x_op = Box::new(null_op);
            y_op = Box::new(decrease);
        } else if pair.0 == 'L' {
            x_op = Box::new(decrease);
            y_op = Box::new(null_op);
        } else if pair.0 == 'R' {
            x_op = Box::new(increase);
            y_op = Box::new(null_op);
        }

        for _n in 0..pair.1 {
            x_position = x_op(x_position);
            y_position = y_op(y_position);
            steps += 1;
            if !positions.contains_key(&(x_position, y_position)) {
                positions.insert((x_position, y_position), steps);
            }
        }
    }
    return positions;
}

fn decode_move(move_code: String) -> (char, i32) {
    let mut c1 = move_code.chars();
    let direction = c1.nth(0).unwrap();
    let stepsize = c1.as_str().parse::<i32>().unwrap();
    return (direction, stepsize);
}

fn split_string(str: String, sep: char) -> Vec<String> {
    let parts: Vec<String> = str.split(sep).flat_map(|x| x.parse::<String>()).collect();
    return parts;
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn find_closest(first_codes: Vec<String>, second_codes: Vec<String>) -> (i32, i32) {
    let first_set = get_positions_visited(first_codes);
    let mut x_position = 0;
    let mut y_position = 0;
    let mut steps = 0;
    let mut least_dist: i32 = std::i32::MAX;
    let mut least_steps: i32 = std::i32::MAX;
    let null_op = |x: i32| x;
    let increase = |x: i32| x + 1;
    let decrease = |x: i32| x - 1;
    let mut x_op: Box<dyn Fn((i32)) -> (i32)> = Box::new(null_op);
    let mut y_op: Box<dyn Fn((i32)) -> (i32)> = Box::new(null_op);

    for m in second_codes {
        let pair = decode_move(m);
        if pair.0 == 'U' {
            x_op = Box::new(null_op);
            y_op = Box::new(increase);
        } else if pair.0 == 'D' {
            x_op = Box::new(null_op);
            y_op = Box::new(decrease);
        } else if pair.0 == 'L' {
            x_op = Box::new(decrease);
            y_op = Box::new(null_op);
        } else if pair.0 == 'R' {
            x_op = Box::new(increase);
            y_op = Box::new(null_op);
        }

        for _n in 0..pair.1 {
            x_position = x_op(x_position);
            y_position = y_op(y_position);
            steps += 1;
            if first_set.contains_key(&(x_position, y_position)) {
                let dist = x_position.abs() + y_position.abs();
                least_dist = cmp::min(dist, least_dist);
                least_steps = cmp::min(least_steps, first_set[&(x_position, y_position)] + steps);
            }
        }
    }

    return (least_dist, least_steps);
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let inputs = split_string(contents, '\n');
    let first_codes = split_string(inputs[0].to_string(), ',');
    let second_codes = split_string(inputs[1].to_string(), ',');
    let result = find_closest(first_codes, second_codes);
    // Should be 860 for part 1
    println!("{}", result.0);

    // Should be 9238 for part 2
    println!("{}", result.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::decode_move;
    use crate::find_closest;
    use crate::get_positions_visited;
    use crate::split_string;

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = split_string(String::from("R8,U5,L5,D3"), ',');
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "R8");
        assert_eq!(result[1], "U5");
        assert_eq!(result[2], "L5");
        assert_eq!(result[3], "D3");
    }

    #[test]
    fn decode_move_works_1() {
        assert_eq!(decode_move(String::from("R1")), ('R', 1));
        assert_eq!(decode_move(String::from("R12")), ('R', 12));
        assert_eq!(decode_move(String::from("R123")), ('R', 123));
    }

    #[test]
    fn try_get_positions_visited() {
        let moves: Vec<String> = vec![
            String::from("U7"),
            String::from("R6"),
            String::from("D4"),
            String::from("L8"),
        ];
        let results = get_positions_visited(moves);
        assert_eq!(results.contains_key(&(0, 0)), true);
        assert_eq!(results.contains_key(&(0, 7)), true);
        assert_eq!(results.contains_key(&(6, 7)), true);
        assert_eq!(results.contains_key(&(6, 3)), true);
        assert_eq!(results.contains_key(&(-2, 3)), true);
    }

    #[test]
    fn try_example_1() {
        let first_codes = split_string("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), ',');
        let second_codes = split_string("U62,R66,U55,R34,D71,R55,D58,R83".to_string(), ',');
        let result = find_closest(first_codes, second_codes);
        assert_eq!(result.0, 159);
    }

    #[test]
    fn try_example_2() {
        let first_codes = split_string(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            ',',
        );
        let second_codes = split_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(), ',');
        let result = find_closest(first_codes, second_codes);
        assert_eq!(result.0, 135);
    }

    #[test]
    fn try_example_1_part2() {
        let first_codes = split_string("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), ',');
        let second_codes = split_string("U62,R66,U55,R34,D71,R55,D58,R83".to_string(), ',');
        let result = find_closest(first_codes, second_codes);
        assert_eq!(result.1, 610);
    }

    #[test]
    fn try_example_2_part2() {
        let first_codes = split_string(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            ',',
        );
        let second_codes = split_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(), ',');
        let result = find_closest(first_codes, second_codes);
        assert_eq!(result.1, 410);
    }
}
