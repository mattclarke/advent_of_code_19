use std::fs::File;
use std::io::prelude::*;

pub fn calculate(mut vec: Vec<i32>, input: i32) -> i32 {
    let mut index = 0;
    let mut output = 0;

    loop {
        if vec[index] == 99 {
            break;
        }

        if vec[index] >= 10000 {
            println!("OMG!!!!!!!!!!!!!!!!");
            break;
        }

        println!("code = {}", vec[index]);

        let opcode = vec[index] % 100;
        if opcode == 1 || opcode == 2 {
            let mut c: i32 = vec[index] / 100;
            let mut operands: Vec<i32> = Vec::new();
            index += 1;

            for _i in 0..2 {
                if c % 10 == 1 {
                    operands.push(vec[index])
                } else {
                    operands.push(vec[vec[index] as usize]);
                }
                c = c / 10;
                index += 1;
            }

            let out_par = vec[index];

            if opcode == 1 {
                vec[out_par as usize] = operands.iter().fold(0, |acc, x| acc + x);
            } else if opcode == 2 {
                vec[out_par as usize] = operands.iter().fold(1, |acc, x| acc * x);
            }

            index += 1;
        } else if opcode == 3 {
            let first = vec[index + 1];
            vec[first as usize] = input;
            index += 2;
        } else if opcode == 4 {
            let c: i32 = vec[index] / 100;
            if c % 10 == 1 {
                output = vec[index + 1];
            } else {
                let first = vec[index + 1];
                output = vec[first as usize];
            }
            index += 2;

        } else if opcode == 5 || opcode == 6 {
            let mut c: i32 = vec[index] / 100;
            let mut operands: Vec<i32> = Vec::new();
            index += 1;

            for _i in 0..2 {
                if c % 10 == 1 {
                    operands.push(vec[index])
                } else {
                    operands.push(vec[vec[index] as usize]);
                }
                c = c / 10;
                index += 1;
            }

            println!("vec = {:?}", operands);

            if operands[0] == 0 {
                if opcode == 6 {
                    index = operands[1] as usize;
                }
            } else {
                if opcode == 5 {
                    index = operands[1] as usize;
                }
            }
        } else if opcode == 7 || opcode == 8 {
            let mut c: i32 = vec[index] / 100;
            let mut operands: Vec<i32> = Vec::new();
            index += 1;

            for _i in 0..2 {
                if c % 10 == 1 {
                    operands.push(vec[index])
                } else {
                    operands.push(vec[vec[index] as usize]);
                }
                c = c / 10;
                index += 1;
            }

            println!("ops {:?}", operands);

            let out_par = vec[index] as usize;
            index += 1;

            if opcode == 7 {
                if operands[0] < operands[1] {
                    vec[out_par as usize] = 1;
                } else {
                    vec[out_par as usize] = 0;
                }
            } else {
                if operands[0] == operands[1] {
                    vec[out_par as usize] = 1;
                } else {
                    vec[out_par as usize] = 0;
                }
            }
        }

        // println!("vec = {:?}", vec);
        println!("index = {}", index);
    }

    return output;
}

pub fn string_to_ints(str: String, sep: char) -> Vec<i32> {
    let parts = str.split(sep);
    println!("{:?}", parts);
    let vec = parts.flat_map(|x| x.parse::<i32>()).collect::<Vec<i32>>();
    println!("{:?}", vec);
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
    // let result = calculate(orig_codes, 1);
    // println!("{}", result);

    // Part 2
    println!("{}", orig_codes.len());
    let result = calculate(orig_codes, 5);
    println!("{}", result);
    
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
        let input = vec![3, 0, 4, 0, 99];

        assert_eq!(calculate(input, 6), 6);
    }

    #[test]
    fn it_works_for_example_2() {
        let input = vec![1002, 6, 3, 6, 4, 6, 33];
        calculate(input, 99);
    }

    #[test]
    fn it_works_for_example_3() {
        let input = vec![1101, 100, -1, 6, 4, 6, 0];
        assert_eq!(calculate(input, 0), 99);
    }

    #[test]
    fn it_works_for_no_leading_value() {
        let input = vec![101, 93, 3, 6, 4, 6, 0];
        assert_eq!(calculate(input, 0), 99);
    }

    #[test]
    fn it_jumps_on_a_five_if_true() {
        let input = vec![4, 1, 1105, 1, 7, 4, 2, 99];
        assert_eq!(calculate(input, 0), 1);
    }

    #[test]
    fn no_jump_on_a_five_if_false() {
        let input = vec![4, 1, 1105, 0, 7, 4, 2, 99];
        assert_eq!(calculate(input, 0), 1105);
    }

    #[test]
    fn it_jumps_on_a_six_if_false() {
        let input = vec![4, 1, 1106, 0, 7, 4, 2, 99];
        assert_eq!(calculate(input, 0), 1);
    }

    #[test]
    fn no_jump_on_a_six_if_true() {
        let input = vec![4, 1, 1106, 1, 7, 4, 2, 99];
        assert_eq!(calculate(input, 0), 1106);
    }

    #[test]
    fn seven_if_true_stores_one() {
        let input = vec![1107, 1, 2, 5, 4, 5, 99];
        assert_eq!(calculate(input, 0), 1);
    }

    #[test]
    fn seven_if_false_stores_zero() {
        let input = vec![1107, 2, 1, 5, 4, 5, 99];
        assert_eq!(calculate(input, 0), 1107);
    }

    #[test]
    fn eight_if_true_stores_one() {
        let input = vec![1108, 1, 1, 5, 4, 5, 99];
        assert_eq!(calculate(input, 0), 1);
    }

    #[test]
    fn eight_if_false_stores_zero() {
        let input = vec![1108, 2, 1, 5, 4, 5, 99];
        assert_eq!(calculate(input, 0), 1108);
    }

    #[test]
    fn it_works_for_example_4_a() {
        let input = vec![3,9,8,9,10,9,4,9,99,-1,8];
        assert_eq!(calculate(input, 8), 1);
    }

    #[test]
    fn it_works_for_example_4_b() {
        let input = vec![3,9,8,9,10,9,4,9,99,-1,8];
        assert_eq!(calculate(input, 123), 0);
    }

    #[test]
    fn it_works_for_example_5_a() {
        let input = vec![3,9,7,9,10,9,4,9,99,-1,8];
        assert_eq!(calculate(input, 7), 1);
    }

    #[test]
    fn it_works_for_example_5_b() {
        let input = vec![3,9,7,9,10,9,4,9,99,-1,8];
        assert_eq!(calculate(input, 123), 0);
    }

    #[test]
    fn it_works_for_example_6_a() {
        let input = vec![3,3,1108,-1,8,3,4,3,99];
        assert_eq!(calculate(input, 8), 1);
    }

    #[test]
    fn it_works_for_example_6_b() {
        let input = vec![3,3,1108,-1,8,3,4,3,99];
        assert_eq!(calculate(input, 123), 0);
    }

    #[test]
    fn it_works_for_example_7_a() {
        let input = vec![3,3,1107,-1,8,3,4,3,99];
        assert_eq!(calculate(input, 7), 1);
    }

    #[test]
    fn it_works_for_example_7_b() {
        let input = vec![3,3,1107,-1,8,3,4,3,99];
        assert_eq!(calculate(input, 123), 0);
    }

    #[test]
    fn it_works_for_example_8_a() {
        let input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(calculate(input, 0), 0);
    }

    #[test]
    fn it_works_for_example_8_b() {
        let input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(calculate(input, 123), 1);
    }

    #[test]
    fn it_works_for_example_9_a() {
        let input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(calculate(input, 0), 0);
    }

    #[test]
    fn it_works_for_example_9_b() {
        let input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(calculate(input, 123), 1);
    }

    #[test]
    fn it_works_for_example_10_a() {
        let input = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        assert_eq!(calculate(input, 0), 0);
    }

    #[test]
    fn it_works_for_example_10_b() {
        let input = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        assert_eq!(calculate(input, 123), 1);
    }

    #[test]
    fn it_works_for_example_11_a() {
        let input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(calculate(input, 7), 999);
    }

    #[test]
    fn it_works_for_example_11_b() {
        let input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(calculate(input, 8), 1000);
    }

    #[test]
    fn it_works_for_example_11_c() {
        let input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(calculate(input, 9), 1001);
    }
}
