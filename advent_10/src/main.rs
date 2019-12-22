use std::collections::HashSet;
use std::fs::File;
use std::cmp;
use std::io::prelude::*;

fn string_to_board_layout(board: &String) -> Vec<Vec<char>> {
    let mut layout: Vec<Vec<char>> = Vec::new();
    let rows = board.split('\n');

    for ro in rows {
        let mut row_vec: Vec<char> = Vec::new();
        for p in ro.chars() {
            row_vec.push(p);
        }
        layout.push(row_vec);
    }

    return layout;
}

fn find_max(layout: &Vec<Vec<i32>>) -> (i32, usize, usize) {
    let mut max_val = 0;
    let mut x_best = 0;
    let mut y_best = 0;

    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            if layout[y][x] > max_val {
                max_val = layout[y][x];
                x_best = x;
                y_best = y;
            }
        }
    }

    return (max_val, x_best, y_best);

}

fn calculate_all(layout: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for y in 0..layout.len() {
        result.push(vec![]);
        for x in 0..layout[0].len() {
            if layout[y][x] == '#' {
                result[y].push(count_for_position(layout, x as i32, y as i32));
            } else {
                result[y].push(0);
            }
        }
    }

    return result;
}

fn count_for_position(layout: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut count = 0;

    count += count_on_axis(layout, x, y);
    count += count_top_right(layout, x as usize, y as usize);
    count += count_top_left(layout, x as usize, y as usize);
    count += count_bottom_right(layout, x as usize, y as usize);
    count += count_bottom_left(layout, x as usize, y as usize);

    return count;
}

fn count_on_axis(layout: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;

    for x_pos in 0..layout[y as usize].len() {
        if x_pos < x as usize && layout[y as usize][x_pos as usize] == '#' {
            left = 1;
        } else if x_pos > x as usize && layout[y as usize][x_pos as usize] == '#' {
            right = 1;
        }
    }

    for y_pos in 0..layout.len() {
        if y_pos < y as usize && layout[y_pos as usize][x as usize] == '#' {
            up = 1;
        } else if y_pos > y as usize && layout[y_pos as usize][x as usize] == '#' {
            down = 1;
        }
    }

    return left + right + up + down;
}

fn count_top_right(layout: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return count_quadrant(layout, x, y, x + 1, layout[0].len(), 0, y);
}

fn count_top_left(layout: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return count_quadrant(layout, x, y, 0, x, 0, y);
}

fn count_bottom_right(layout: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return count_quadrant(layout, x, y, x + 1, layout[0].len(), y + 1, layout.len());
}

fn count_bottom_left(layout: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return count_quadrant(layout, x, y, 0, x, y + 1, layout.len());
}

fn count_quadrant(
    layout: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) -> i32 {
    let mut count = 0;
    let mut directions = HashSet::new();

    for y_pos in y_start..y_end {
        for x_pos in x_start..x_end {
            if layout[y_pos][x_pos] == '#' {
                let dir =
                    find_direction_unit((x_pos as i32) - (x as i32), (y_pos as i32) - (y as i32));
                if !directions.contains(&dir) {
                    directions.insert(dir);
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn print_layout(layout: &Vec<Vec<char>>) {
    for row in layout {
        println!("{:?}", row);
    }
}

fn print_result(layout: &Vec<Vec<i32>>) {
    for row in layout {
        for v in row {
            if *v == (0 as i32) {
                print!(".");
            }
            else {
                print!("{}", v);
            }
        }
        println!("");
    }
}

fn find_direction_unit(x: i32, y: i32) -> (i32, i32) {
    if x == 0 {
        return (0, 1);
    }
    if y == 0 {
        return (1, 0);
    }

    let mut x_val = i32::abs(x);
    let mut y_val = i32::abs(y);
    let primes: Vec<i32> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    ];

    let mut x_primes: Vec<i32> = Vec::new();
    let mut y_primes: Vec<i32> = Vec::new();

    while x_val != 1 || y_val != 1 {
        for p in &primes {
            if x_val % p == 0 && y_val % p == 0 {
                x_val /= p;
                y_val /= p;
            } else if x_val % p == 0 {
                x_primes.push(*p);
                x_val /= p;
            } else if y_val % p == 0 {
                y_primes.push(*p);
                y_val /= p;
            }
        }
    }

    x_val = x_primes.iter().fold(1, |acc, x| acc * x);
    y_val = y_primes.iter().fold(1, |acc, x| acc * x);

    return (x_val, y_val);
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let layout = string_to_board_layout(&contents);
    let filled_in = calculate_all(&layout);
    // Part 1 = (253, 11, 19)
    let result = find_max(&filled_in);
    println!("Part 1 = {:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate_all;
    use crate::count_bottom_left;
    use crate::count_bottom_right;
    use crate::count_for_position;
    use crate::count_on_axis;
    use crate::count_top_left;
    use crate::count_top_right;
    use crate::find_direction_unit;
    use crate::print_layout;
    use crate::print_result;
    use crate::string_to_board_layout;
    use crate::find_max;

    #[test]
    fn can_convert_board_to_2d_array() {
        let board = String::from(".#..#\n.....\n#####\n....#\n...##");
        let layout = string_to_board_layout(&board);

        assert_eq!(layout[0], vec!['.', '#', '.', '.', '#']);
        assert_eq!(layout[0], vec!['.', '#', '.', '.', '#']);
    }

    #[test]
    fn find_unit_direction() {
        assert_eq!(find_direction_unit(1, 2), (1, 2));
        assert_eq!(find_direction_unit(2, 4), (1, 2));
        assert_eq!(find_direction_unit(12, 4), (3, 1));
        assert_eq!(find_direction_unit(15, 5), (3, 1));
        assert_eq!(find_direction_unit(21, 20), (21, 20));
        assert_eq!(find_direction_unit(104, 100), (26, 25));

        assert_eq!(find_direction_unit(2, 1), (2, 1));
        assert_eq!(find_direction_unit(4, 2), (2, 1));
        assert_eq!(find_direction_unit(4, 12), (1, 3));
        assert_eq!(find_direction_unit(5, 15), (1, 3));
        assert_eq!(find_direction_unit(20, 21), (20, 21));
        assert_eq!(find_direction_unit(100, 104), (25, 26));

        assert_eq!(find_direction_unit(100, 0), (1, 0));
        assert_eq!(find_direction_unit(0, 10), (0, 1));

        assert_eq!(find_direction_unit(-2, -1), (2, 1));
        assert_eq!(find_direction_unit(-2, 1), (2, 1));
        assert_eq!(find_direction_unit(2, -1), (2, 1));
    }

    #[test]
    fn can_count_visible_on_axis_1() {
        let board = String::from("..#..\n..#..\n#####\n..#..\n..#..");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 4);
    }

    #[test]
    fn can_count_visible_on_axis_2() {
        let board = String::from("..#..\n..#..\n..#..\n..#..\n..#..");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_3() {
        let board = String::from(".....\n.....\n#.#.#\n.....\n.....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_4() {
        let board = String::from(".....\n.....\n#.#..\n.....\n..#..");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_5() {
        let board = String::from("..#..\n.....\n..#.#\n.....\n.....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_6() {
        let board = String::from("##.##\n##.##\n.....\n##.##\n##.##");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_on_axis(&layout, 2, 2), 0);
    }

    #[test]
    fn can_count_bottom_right_1() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n....#");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 0, 0), 1);
    }

    #[test]
    fn can_count_bottom_right_2() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 0, 0), 2);
    }

    #[test]
    fn can_count_bottom_right_3() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 0, 0), 3);
    }

    #[test]
    fn can_count_bottom_right_4() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 3, 4), 0);
    }

    #[test]
    fn can_count_bottom_right_5() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 4, 3), 0);
    }

    #[test]
    fn can_count_bottom_right_bounds_1() {
        let board = String::from("....#\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 4, 0), 0);
    }

    #[test]
    fn can_count_bottom_right_bounds_2() {
        let board = String::from("....#\n.#...\n..#..\n....#\n....#");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_right(&layout, 4, 4), 0);
    }

    #[test]
    fn can_count_bottom_left_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n#....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_left(&layout, 4, 0), 1);
    }

    #[test]
    fn can_count_bottom_left_2() {
        let board = String::from("....#\n...#.\n..#..\n#....\n.#...");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_left(&layout, 4, 0), 3);
    }

    #[test]
    fn can_count_bottom_left_3() {
        let board = String::from("....#\n...#.\n..#..\n#....\n.#...");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_left(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_bottom_left_bounds_1() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_left(&layout, 0, 0), 0);
    }

    #[test]
    fn can_count_bottom_left_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_bottom_left(&layout, 3, 4), 0);
    }

    #[test]
    fn can_count_top_right_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n#....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_right(&layout, 0, 4), 1);
    }

    #[test]
    fn can_count_top_right_2() {
        let board = String::from("...#.\n...#.\n.#...\n.#...\n#....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_right(&layout, 0, 4), 3);
    }

    #[test]
    fn can_count_top_right_3() {
        let board = String::from("...#.\n...#.\n.#...\n.#...\n#....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_right(&layout, 1, 2), 2);
    }

    #[test]
    fn can_count_top_right_bounds_1() {
        let board = String::from(".#...\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_right(&layout, 1, 0), 0);
    }

    #[test]
    fn can_count_top_right_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_right(&layout, 4, 4), 0);
    }

    #[test]
    fn can_count_top_left_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n....#");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_left(&layout, 4, 4), 3);
    }

    #[test]
    fn can_count_top_left_2() {
        let board = String::from("#....\n.#...\n..#..\n.#...\n....#");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_left(&layout, 4, 4), 2);
    }

    #[test]
    fn can_count_top_left_3() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n....#");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_left(&layout, 4, 4), 1);
    }

    #[test]
    fn can_count_top_left_bounds_1() {
        let board = String::from(".#...\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_left(&layout, 1, 0), 0);
    }

    #[test]
    fn can_count_top_left_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n#....");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_top_left(&layout, 0, 4), 0);
    }

    #[test]
    fn can_count_for_positions() {
        let board = String::from(".#..#\n.....\n#####\n....#\n...##");
        let layout = string_to_board_layout(&board);
        print_layout(&layout);

        assert_eq!(count_for_position(&layout, 1, 0), 7);
        assert_eq!(count_for_position(&layout, 4, 0), 7);
        assert_eq!(count_for_position(&layout, 0, 2), 6);
        assert_eq!(count_for_position(&layout, 1, 2), 7);
        assert_eq!(count_for_position(&layout, 2, 2), 7);
        assert_eq!(count_for_position(&layout, 3, 2), 7);
        assert_eq!(count_for_position(&layout, 4, 2), 5);

        assert_eq!(count_for_position(&layout, 4, 3), 7);

        assert_eq!(count_for_position(&layout, 3, 4), 8);
        assert_eq!(count_for_position(&layout, 4, 4), 7);
    }

    #[test]
    fn example_1() {
        let board = String::from(".#..#\n.....\n#####\n....#\n...##");
        let layout = string_to_board_layout(&board);
        let filled_in = calculate_all(&layout);

        let result = find_max(&filled_in);

        assert_eq!(result.0, 8);
        assert_eq!(result.1, 3);
        assert_eq!(result.2, 4);
    }

    #[test]
    fn example_2() {
        let board = String::from("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####");
        let layout = string_to_board_layout(&board);
        let filled_in = calculate_all(&layout);

        let result = find_max(&filled_in);

        assert_eq!(result.0, 33);
        assert_eq!(result.1, 5);
        assert_eq!(result.2, 8);
    }
}
