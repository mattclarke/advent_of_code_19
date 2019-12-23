use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn string_to_board_layout(board: &String) -> Vec<Vec<i32>> {
    let mut layout: Vec<Vec<i32>> = Vec::new();
    let rows = board.split('\n');

    for ro in rows {
        let mut row_vec: Vec<i32> = Vec::new();
        for p in ro.chars() {
            if p == '.' {
                row_vec.push(0);
            } else {
                row_vec.push(-1);
            }
        }
        layout.push(row_vec);
    }

    return layout;
}

fn find_kills_in_order(layout: &mut Vec<Vec<i32>>, x: i32, y: i32) -> Vec<i32> {
    let mut kills: Vec<i32> = Vec::new();
        let mut result: i32 = -1; 

        while true {
            let to_kill = find_hits_on_one_rotation(&layout, x, y);
            if to_kill.len() == 0 {
                break;
            }

            for c in to_kill {
                layout[c.y as usize][c.x as usize] = 0;
                kills.push(c.x*100 + c.y);
            }

        }
    return kills;
}

fn find_hits_on_one_rotation(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> Vec<Coords> {
    let mut result: Vec<Coords> = Vec::new();

    let up = look_upwards(&layout, x, y);
    if up != (x, y) {
        result.push(Coords {
            x: up.0,
            y: up.1,
            unit: 0.0,
        });
    }

    let mut top_right = count_top_right(&layout, x as usize, y as usize);
    result.append(&mut top_right);

    let right = look_right(&layout, x, y);
    if right != (x, y) {
        result.push(Coords {
            x: right.0,
            y: right.1,
            unit: 0.0,
        });
    }

    let mut bottom_right = count_bottom_right(&layout, x as usize, y as usize);
    result.append(&mut bottom_right);

    let down = look_downwards(&layout, x, y);
    if down != (x, y) {
        result.push(Coords {
            x: down.0,
            y: down.1,
            unit: 0.0,
        });
    }

    let mut bottom_left = count_bottom_left(&layout, x as usize, y as usize);
    result.append(&mut bottom_left);

    let left = look_left(&layout, x, y);
    if left != (x, y) {
        result.push(Coords {
            x: left.0,
            y: left.1,
            unit: 0.0,
        });
    }

    let mut top_left = count_top_left(&layout, x as usize, y as usize);
    result.append(&mut top_left);

    return result;
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

fn calculate_all(layout: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for y in 0..layout.len() {
        result.push(vec![]);
        for x in 0..layout[0].len() {
            if layout[y][x] == -1 {
                result[y].push(count_for_position(layout, x as i32, y as i32));
            } else {
                result[y].push(0);
            }
        }
    }

    return result;
}

fn count_for_position(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    let mut count = 0;

    count += count_on_axis(layout, x, y);
    count += count_top_right(layout, x as usize, y as usize).len() as i32;
    count += count_top_left(layout, x as usize, y as usize).len() as i32;
    count += count_bottom_right(layout, x as usize, y as usize).len() as i32;
    count += count_bottom_left(layout, x as usize, y as usize).len() as i32;

    return count;
}

fn look_upwards(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> (i32, i32) {
    for y_pos in (0..y).rev() {
        if layout[y_pos as usize][x as usize] == -1 {
            return (x, y_pos);
        }
    }

    return (x, y);
}

fn look_downwards(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> (i32, i32) {
    for y_pos in (y + 1)..(layout.len() as i32) {
        if layout[y_pos as usize][x as usize] == -1 {
            return (x, y_pos);
        }
    }

    return (x, y);
}

fn look_right(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> (i32, i32) {
    for x_pos in (x + 1)..(layout[y as usize].len() as i32) {
        if layout[y as usize][x_pos as usize] == -1 {
            return (x_pos, y);
        }
    }

    return (x, y);
}

fn look_left(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> (i32, i32) {
    for x_pos in (0..x).rev() {
        if layout[y as usize][x_pos as usize] == -1 {
            return (x_pos, y);
        }
    }

    return (x, y);
}

fn count_on_axis(layout: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;

    if look_right(&layout, x, y) != (x, y) {
        right = 1;
    }

    if look_left(&layout, x, y) != (x, y) {
        left = 1;
    }

    if look_upwards(&layout, x, y) != (x, y) {
        up = 1;
    }

    if look_downwards(&layout, x, y) != (x, y) {
        down = 1;
    }

    return left + right + up + down;
}

struct Coords {
    x: i32,
    y: i32,
    unit: f32,
}

fn count_top_right(layout: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Coords> {
    let mut directions = HashSet::new();
    let mut hits: Vec<Coords> = Vec::new();

    for x_pos in (x + 1)..layout[0].len() {
        for y_pos in 0..y {
            if layout[y_pos][x_pos] == -1 {
                let dir =
                    find_direction_unit((x_pos as i32) - (x as i32), (y_pos as i32) - (y as i32));
                if !directions.contains(&dir) {
                    directions.insert(dir);
                    hits.push(Coords {
                        x: x_pos as i32,
                        y: y_pos as i32,
                        unit: (dir.1 as f32) / (dir.0 as f32),
                    });
                }
            }
        }
    }
    hits.sort_by(|a, b| b.unit.partial_cmp(&a.unit).unwrap());

    return hits;
}

fn count_top_left(layout: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Coords> {
    let mut directions = HashSet::new();
    let mut hits: Vec<Coords> = Vec::new();

    for y_pos in (0..y).rev() {
        for x_pos in (0..x).rev() {
            if layout[y_pos][x_pos] == -1 {
                let dir =
                    find_direction_unit((x_pos as i32) - (x as i32), (y_pos as i32) - (y as i32));
                if !directions.contains(&dir) {
                    directions.insert(dir);
                    hits.push(Coords {
                        x: x_pos as i32,
                        y: y_pos as i32,
                        unit: (dir.1 as f32) / (dir.0 as f32),
                    });
                }
            }
        }
    }
    hits.sort_by(|a, b| a.unit.partial_cmp(&b.unit).unwrap());

    return hits;
}

fn count_bottom_right(layout: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Coords> {
    let mut directions = HashSet::new();
    let mut hits: Vec<Coords> = Vec::new();

    for y_pos in (y + 1)..layout.len() {
        for x_pos in (x + 1)..layout[y_pos].len() {
            if layout[y_pos][x_pos] == -1 {
                let dir =
                    find_direction_unit((x_pos as i32) - (x as i32), (y_pos as i32) - (y as i32));
                if !directions.contains(&dir) {
                    directions.insert(dir);
                    hits.push(Coords {
                        x: x_pos as i32,
                        y: y_pos as i32,
                        unit: (dir.1 as f32) / (dir.0 as f32),
                    });
                }
            }
        }
    }

    hits.sort_by(|a, b| a.unit.partial_cmp(&b.unit).unwrap());

    return hits;
}

fn count_bottom_left(layout: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Coords> {
    let mut directions = HashSet::new();
    let mut hits: Vec<Coords> = Vec::new();

    for y_pos in (y + 1)..layout.len() {
        for x_pos in (0..x).rev() {
            if layout[y_pos][x_pos] == -1 {
                let dir =
                    find_direction_unit((x_pos as i32) - (x as i32), (y_pos as i32) - (y as i32));
                if !directions.contains(&dir) {
                    directions.insert(dir);
                    hits.push(Coords {
                        x: x_pos as i32,
                        y: y_pos as i32,
                        unit: (dir.1 as f32) / (dir.0 as f32),
                    });
                }
            }
        }
    }

    hits.sort_by(|a, b| b.unit.partial_cmp(&a.unit).unwrap());

    return hits;
}

fn print_layout(layout: &Vec<Vec<i32>>) {
    for row in layout {
        for v in row {
            if *v == (0 as i32) {
                print!(".");
            } else if *v == (-1 as i32) {
                print!("x");
            } else {
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
    let mut layout = string_to_board_layout(&contents);
    let filled_in = calculate_all(&layout);
    // Part 1 = (253, 11, 19)
    let result1 = find_max(&filled_in);
    println!("Part 1 = {:?}", result1);

    // Part 2
    let result2 = find_kills_in_order(&mut layout, result1.1 as i32, result1.2 as i32);
    println!("Part 2 = {}", result2[199]);

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
    use crate::find_hits_on_one_rotation;
    use crate::find_max;
    use crate::print_layout;
    use crate::string_to_board_layout;
    use crate::find_kills_in_order;

    #[test]
    fn can_convert_board_to_2d_array() {
        let board = String::from(".#..#\n.....\n#####\n....#\n...##");
        let layout = string_to_board_layout(&board);

        assert_eq!(layout[0], vec![0, -1, 0, 0, -1]);
        assert_eq!(layout[0], vec![0, -1, 0, 0, -1]);
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

        assert_eq!(count_on_axis(&layout, 2, 2), 4);
    }

    #[test]
    fn can_count_visible_on_axis_2() {
        let board = String::from("..#..\n..#..\n..#..\n..#..\n..#..");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_3() {
        let board = String::from(".....\n.....\n#.#.#\n.....\n.....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_4() {
        let board = String::from(".....\n.....\n#.#..\n.....\n..#..");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_5() {
        let board = String::from("..#..\n.....\n..#.#\n.....\n.....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_on_axis(&layout, 2, 2), 2);
    }

    #[test]
    fn can_count_visible_on_axis_6() {
        let board = String::from("##.##\n##.##\n.....\n##.##\n##.##");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_on_axis(&layout, 2, 2), 0);
    }

    #[test]
    fn can_count_bottom_right_1() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n....#");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 0, 0).len(), 1);
    }

    #[test]
    fn can_count_bottom_right_2() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 0, 0).len(), 2);
    }

    #[test]
    fn can_count_bottom_right_3() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 0, 0).len(), 3);
    }

    #[test]
    fn can_count_bottom_right_4() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 3, 4).len(), 0);
    }

    #[test]
    fn can_count_bottom_right_5() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 4, 3).len(), 0);
    }

    #[test]
    fn can_count_bottom_right_bounds_1() {
        let board = String::from("....#\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 4, 0).len(), 0);
    }

    #[test]
    fn can_count_bottom_right_bounds_2() {
        let board = String::from("....#\n.#...\n..#..\n....#\n....#");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_right(&layout, 4, 4).len(), 0);
    }

    #[test]
    fn can_count_bottom_left_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n#....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_left(&layout, 4, 0).len(), 1);
    }

    #[test]
    fn can_count_bottom_left_2() {
        let board = String::from("....#\n...#.\n..#..\n#....\n.#...");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_left(&layout, 4, 0).len(), 3);
    }

    #[test]
    fn can_count_bottom_left_3() {
        let board = String::from("....#\n...#.\n..#..\n#....\n.#...");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_left(&layout, 2, 2).len(), 2);
    }

    #[test]
    fn can_count_bottom_left_bounds_1() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_left(&layout, 0, 0).len(), 0);
    }

    #[test]
    fn can_count_bottom_left_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_bottom_left(&layout, 3, 4).len(), 0);
    }

    #[test]
    fn can_count_top_right_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n#....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_right(&layout, 0, 4).len(), 1);
    }

    #[test]
    fn can_count_top_right_2() {
        let board = String::from("...#.\n...#.\n.#...\n.#...\n#....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_right(&layout, 0, 4).len(), 3);
    }

    #[test]
    fn can_count_top_right_3() {
        let board = String::from("...#.\n...#.\n.#...\n.#...\n#....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_right(&layout, 1, 2).len(), 2);
    }

    #[test]
    fn can_count_top_right_bounds_1() {
        let board = String::from(".#...\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_right(&layout, 1, 0).len(), 0);
    }

    #[test]
    fn can_count_top_right_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_right(&layout, 4, 4).len(), 0);
    }

    #[test]
    fn can_count_top_left_1() {
        let board = String::from("....#\n...#.\n..#..\n.#...\n....#");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_left(&layout, 4, 4).len(), 3);
    }

    #[test]
    fn can_count_top_left_2() {
        let board = String::from("#....\n.#...\n..#..\n.#...\n....#");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_left(&layout, 4, 4).len(), 2);
    }

    #[test]
    fn can_count_top_left_3() {
        let board = String::from("#....\n.#...\n..#..\n...#.\n....#");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_left(&layout, 4, 4).len(), 1);
    }

    #[test]
    fn can_count_top_left_bounds_1() {
        let board = String::from(".#...\n.#...\n..#..\n....#\n...#.");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_left(&layout, 1, 0).len(), 0);
    }

    #[test]
    fn can_count_top_left_bounds_2() {
        let board = String::from("#....\n.#...\n..#..\n....#\n#....");
        let layout = string_to_board_layout(&board);

        assert_eq!(count_top_left(&layout, 0, 4).len(), 0);
    }

    #[test]
    fn can_count_for_positions() {
        let board = String::from(".#..#\n.....\n#####\n....#\n...##");
        let layout = string_to_board_layout(&board);

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

    #[test]
    fn hit_order_top_right() {
        let board = String::from(concat!(
            ".#....#####...#..\n",
            "##...##.#####..##\n",
            "##...#...#.#####.\n",
            "..#.....X...###..\n",
            "..#.#.....#....##"
        ));
        let mut layout = string_to_board_layout(&board);
        let filled_in = count_top_right(&layout, 8, 3);

        for (i, c) in filled_in.iter().enumerate() {
            layout[c.y as usize][c.x as usize] = (i + 2) as i32;
        }

        assert_eq!(filled_in[0].x, 9);
        assert_eq!(filled_in[0].y, 0);

        assert_eq!(filled_in[1].x, 9);
        assert_eq!(filled_in[1].y, 1);

        assert_eq!(filled_in[2].x, 10);
        assert_eq!(filled_in[2].y, 0);

        assert_eq!(filled_in[3].x, 9);
        assert_eq!(filled_in[3].y, 2);

        assert_eq!(filled_in[4].x, 11);
        assert_eq!(filled_in[4].y, 1);

        assert_eq!(filled_in[5].x, 12);
        assert_eq!(filled_in[5].y, 1);
    }

    #[test]
    fn hit_order_bottom_right() {
        let board = String::from(concat!(
            ".#....#####...#..\n",
            "##...##.#####..##\n",
            "##...#...#.#####.\n",
            "..#.....X...###..\n",
            "..#.#.....#....##"
        ));
        let mut layout = string_to_board_layout(&board);
        let filled_in = count_bottom_right(&layout, 8, 3);

        for (i, c) in filled_in.iter().enumerate() {
            layout[c.y as usize][c.x as usize] = (i + 2) as i32;
        }

        print_layout(&layout);

        assert_eq!(filled_in[0].x, 16);
        assert_eq!(filled_in[0].y, 4);

        assert_eq!(filled_in[1].x, 15);
        assert_eq!(filled_in[1].y, 4);

        assert_eq!(filled_in[2].x, 10);
        assert_eq!(filled_in[2].y, 4);
    }

    #[test]
    fn hit_order_bottom_left() {
        let board = String::from(concat!(
            ".#....#####...#..\n",
            "##...##.#####..##\n",
            "##...#...#.#####.\n",
            "..#.....X...###..\n",
            "..#.#.....#....##"
        ));
        let mut layout = string_to_board_layout(&board);
        let filled_in = count_bottom_left(&layout, 8, 3);

        for (i, c) in filled_in.iter().enumerate() {
            layout[c.y as usize][c.x as usize] = (i + 2) as i32;
        }

        print_layout(&layout);

        assert_eq!(filled_in[0].x, 4);
        assert_eq!(filled_in[0].y, 4);

        assert_eq!(filled_in[1].x, 2);
        assert_eq!(filled_in[1].y, 4);
    }

    #[test]
    fn hit_order_top_left() {
        let board = String::from(concat!(
            ".#....#####...#..\n",
            "##...##.#####..##\n",
            "##...#...#.#####.\n",
            "..#.....X...###..\n",
            "..#.#.....#....##"
        ));
        let mut layout = string_to_board_layout(&board);
        let filled_in = count_top_left(&layout, 8, 3);

        for (i, c) in filled_in.iter().enumerate() {
            layout[c.y as usize][c.x as usize] = (i + 2) as i32;
        }

        print_layout(&layout);

        assert_eq!(filled_in[0].x, 0);
        assert_eq!(filled_in[0].y, 2);

        assert_eq!(filled_in[1].x, 1);
        assert_eq!(filled_in[1].y, 2);

        assert_eq!(filled_in[2].x, 0);
        assert_eq!(filled_in[2].y, 1);

        assert_eq!(filled_in[3].x, 1);
        assert_eq!(filled_in[3].y, 1);
    }

    #[test]
    fn destroy_one_rotation() {
        let board = String::from(concat!(
            ".#....#####...#..\n",
            "##...##.#####..##\n",
            "##...#...#.#####.\n",
            "..#.....X...###..\n",
            "..#.#.....#....##"
        ));
        let mut layout = string_to_board_layout(&board);
        let to_kill = find_hits_on_one_rotation(&layout, 8, 3);

        for c in to_kill {
            layout[c.y as usize][c.x as usize] = 0;
        }

        print_layout(&layout);

        let left_over = find_hits_on_one_rotation(&layout, 8, 3);

        assert_eq!(left_over.len(), 5);

        assert_eq!(left_over[0].x, 8);
        assert_eq!(left_over[0].y, 0);

        assert_eq!(left_over[1].x, 10);
        assert_eq!(left_over[1].y, 1);

        assert_eq!(left_over[2].x, 14);
        assert_eq!(left_over[2].y, 0);

        assert_eq!(left_over[3].x, 16);
        assert_eq!(left_over[3].y, 1);

        assert_eq!(left_over[4].x, 13);
        assert_eq!(left_over[4].y, 3);
    }

    #[test]
    fn big_example() {
        let board = String::from(concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##"
        ));
        let mut layout = string_to_board_layout(&board);

        let result = find_kills_in_order(&mut layout, 11, 13);

        assert_eq!(result[199], 802);
        assert_eq!(result[298], 1101);
    }
}
