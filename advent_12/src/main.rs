use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

#[derive(Hash)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    xv: i32,
    yv: i32,
    zv: i32,
    x_0: i32,
    y_0: i32,
    z_0: i32,
}

fn string_to_moons(str: String) -> Vec<Moon> {
    let mut result: Vec<Moon> = Vec::new();
    let parts = str.split('\n');
    // <x=-1, y=0, z=2>
    let re = Regex::new(r"^<x=([-\d]+), y=([-\d]+), z=([-\d]+)>$").unwrap();

    for p in parts {
        for cap in re.captures_iter(p) {
            result.push(Moon {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
                z: cap[3].parse::<i32>().unwrap(),
                xv: 0,
                yv: 0,
                zv: 0,
                x_0: cap[1].parse::<i32>().unwrap(),
                y_0: cap[2].parse::<i32>().unwrap(),
                z_0: cap[3].parse::<i32>().unwrap(),
            })
        }
    }
    return result;
}

fn calculate_velocities(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() - 1 {
        for j in i + 1..moons.len() {
            if moons[i].x != moons[j].x {
                if moons[i].x > moons[j].x {
                    moons[i].xv -= 1;
                    moons[j].xv += 1;
                } else {
                    moons[i].xv += 1;
                    moons[j].xv -= 1;
                }
            }

            if moons[i].y != moons[j].y {
                if moons[i].y > moons[j].y {
                    moons[i].yv -= 1;
                    moons[j].yv += 1;
                } else {
                    moons[i].yv += 1;
                    moons[j].yv -= 1;
                }
            }

            if moons[i].z != moons[j].z {
                if moons[i].z > moons[j].z {
                    moons[i].zv -= 1;
                    moons[j].zv += 1;
                } else {
                    moons[i].zv += 1;
                    moons[j].zv -= 1;
                }
            }
        }
    }
}

fn move_moons(moons: &mut Vec<Moon>) {
    for m in moons {
        m.x += m.xv;
        m.y += m.yv;
        m.z += m.zv;
    }
}

fn calculate_energy(moons: &Vec<Moon>) -> i32 {
    let mut energy = 0;

    for m in moons {
        let p_energy = m.x.abs() + m.y.abs() + m.z.abs();
        let k_energy = m.xv.abs() + m.yv.abs() + m.zv.abs();
        energy += p_energy * k_energy;
    }

    return energy;
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn hash_moons(moons: &Vec<Moon>) -> (u64, u64, u64, u64) {
    return (
        calculate_hash(&moons[0]),
        calculate_hash(&moons[1]),
        calculate_hash(&moons[2]),
        calculate_hash(&moons[3]),
    );
}

fn print_moons(moons: &Vec<Moon>) {
    for m in moons {
        println!(
            "x={} y={} z={} | xv={} yv={} yz={}",
            m.x, m.y, m.z, m.xv, m.yv, m.zv
        );
    }
    println!("Energy {}", calculate_energy(moons));
    println!("-------------------------");
}

fn read_file() -> String {
    let mut file = File::open("input_data.txt").expect("No such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() -> std::io::Result<()> {
    let contents = read_file();
    let mut moons = string_to_moons(contents);

    // Part 1 = 8625
    // for _i in 0..1000 {
    //     calculate_velocities(&mut moons);
    //     move_moons(&mut moons);
    // }

    // let energy = calculate_energy(&moons);
    // println!("Part 1 = {}", energy);

    // Part 2 = 332477126821644
    let mut count: i64 = 0;
        let mut x_ans = 0;
        let mut y_ans = 0;
        let mut z_ans = 0;
        let mut x_count = 0;
        let mut y_count = 0;
        let mut z_count = 0;

        while count < 1000000 {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);

            if moons[0].x == moons[0].x_0
                && moons[1].x == moons[1].x_0
                && moons[2].x == moons[2].x_0
                && moons[3].x == moons[3].x_0
                && moons[0].xv == 0
                && moons[1].xv == 0
                && moons[2].xv == 0
                && moons[3].xv == 0
            {
                x_ans = count-x_count;
                x_count = count;
            }

            if moons[0].y == moons[0].y_0
                && moons[1].y == moons[1].y_0
                && moons[2].y == moons[2].y_0
                && moons[3].y == moons[3].y_0
                && moons[0].yv == 0
                && moons[1].yv == 0
                && moons[2].yv == 0
                && moons[3].yv == 0
            {
                y_ans = count-y_count;
                y_count = count;
            }

            if moons[0].z == moons[0].z_0
                && moons[1].z == moons[1].z_0
                && moons[2].z == moons[2].z_0
                && moons[3].z == moons[3].z_0
                && moons[0].zv == 0
                && moons[1].zv == 0
                && moons[2].zv == 0
                && moons[3].zv == 0
            {
                z_ans = count-z_count;
                z_count = count;
            }

            count += 1;
        }

        // To get the exact answer we need to calculate
        // the least common multiple
        // Skipping that for now, can be done at
        // https://www.calculatorsoup.com/calculators/math/lcm.php
        println!("{} {} {} ", x_ans, y_ans, z_ans);


    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calculate_energy;
    use crate::calculate_hash;
    use crate::calculate_velocities;
    use crate::move_moons;
    use crate::print_moons;
    use crate::string_to_moons;
    use crate::Moon;
    use std::collections::HashSet;

    #[test]
    fn input_to_moons() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let moons = string_to_moons(input.to_string());

        assert_eq!(moons[0].x, -1);
        assert_eq!(moons[0].y, 0);
        assert_eq!(moons[0].z, 2);
        assert_eq!(moons[1].x, 2);
        assert_eq!(moons[1].y, -10);
        assert_eq!(moons[1].z, -7);

        assert_eq!(moons[2].x, 4);
        assert_eq!(moons[2].y, -8);
        assert_eq!(moons[2].z, 8);

        assert_eq!(moons[3].x, 3);
        assert_eq!(moons[3].y, 5);
        assert_eq!(moons[3].z, -1);
    }

    #[test]
    fn calculate_first_velocities() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let mut moons = string_to_moons(input.to_string());

        calculate_velocities(&mut moons);
        print_moons(&moons);

        assert_eq!(moons[0].xv, 3);
        assert_eq!(moons[0].yv, -1);
        assert_eq!(moons[0].zv, -1);

        assert_eq!(moons[1].xv, 1);
        assert_eq!(moons[1].yv, 3);
        assert_eq!(moons[1].zv, 3);

        assert_eq!(moons[2].xv, -3);
        assert_eq!(moons[2].yv, 1);
        assert_eq!(moons[2].zv, -3);

        assert_eq!(moons[3].xv, -1);
        assert_eq!(moons[3].yv, -3);
        assert_eq!(moons[3].zv, 1);
    }

    #[test]
    fn calculate_first_positions() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let mut moons = string_to_moons(input.to_string());

        calculate_velocities(&mut moons);
        move_moons(&mut moons);
        print_moons(&moons);

        assert_eq!(moons[0].x, 2);
        assert_eq!(moons[0].y, -1);
        assert_eq!(moons[0].z, 1);

        assert_eq!(moons[1].x, 3);
        assert_eq!(moons[1].y, -7);
        assert_eq!(moons[1].z, -4);

        assert_eq!(moons[2].x, 1);
        assert_eq!(moons[2].y, -7);
        assert_eq!(moons[2].z, 5);

        assert_eq!(moons[3].x, 2);
        assert_eq!(moons[3].y, 2);
        assert_eq!(moons[3].z, 0);
    }

    #[test]
    fn calculate_energy_example_1() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let mut moons = string_to_moons(input.to_string());

        for _i in 0..10 {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);
        }

        let energy = calculate_energy(&moons);

        assert_eq!(energy, 179);
    }

    #[test]
    fn calculate_energy_example_2() {
        let input = concat!(
            "<x=-8, y=-10, z=0>\n",
            "<x=5, y=5, z=10>\n",
            "<x=2, y=-7, z=3>\n",
            "<x=9, y=-8, z=-3>"
        );

        let mut moons = string_to_moons(input.to_string());

        for _i in 0..100 {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);
        }

        let energy = calculate_energy(&moons);

        assert_eq!(energy, 1940);
    }

    #[test]
    fn repeats_example_1() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let mut moons = string_to_moons(input.to_string());
        let mut previous_states = HashSet::new();
        let mut count = 0;

        while true {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);
            let hashes = calculate_hash(&moons);
            if previous_states.contains(&hashes) {
                break;
            }
            previous_states.insert(hashes);
            count += 1;
        }

        assert_eq!(count, 2772);
    }

    #[test]
    fn repeats_example_1_clever() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>"
        );

        let mut moons = string_to_moons(input.to_string());
        let mut count: i64 = 0;
        let mut x_ans = 0;
        let mut y_ans = 0;
        let mut z_ans = 0;
        let mut x_count = 0;
        let mut y_count = 0;
        let mut z_count = 0;

        while count < 1000000 {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);

            if moons[0].x == moons[0].x_0
                && moons[1].x == moons[1].x_0
                && moons[2].x == moons[2].x_0
                && moons[3].x == moons[3].x_0
                && moons[0].xv == 0
                && moons[1].xv == 0
                && moons[2].xv == 0
                && moons[3].xv == 0
            {
                x_ans = count-x_count;
                x_count = count;
            }

            if moons[0].y == moons[0].y_0
                && moons[1].y == moons[1].y_0
                && moons[2].y == moons[2].y_0
                && moons[3].y == moons[3].y_0
                && moons[0].yv == 0
                && moons[1].yv == 0
                && moons[2].yv == 0
                && moons[3].yv == 0
            {
                y_ans = count-y_count;
                y_count = count;
            }

            if moons[0].z == moons[0].z_0
                && moons[1].z == moons[1].z_0
                && moons[2].z == moons[2].z_0
                && moons[3].z == moons[3].z_0
                && moons[0].zv == 0
                && moons[1].zv == 0
                && moons[2].zv == 0
                && moons[3].zv == 0
            {
                z_ans = count-z_count;
                z_count = count;
            }

            count += 1;
        }

        println!("{} {} {} ", x_ans, y_ans, z_ans);
        assert_eq!((x_ans * y_ans * z_ans) % 2772, 0);
    }

    #[test]
    fn repeats_example_2_clever() {
        let input = concat!(
            "<x=-8, y=-10, z=0>\n",
            "<x=5, y=5, z=10>\n",
            "<x=2, y=-7, z=3>\n",
            "<x=9, y=-8, z=-3>"
        );

        let mut moons = string_to_moons(input.to_string());
        let mut count: i64 = 0;
        let mut x_ans = 0;
        let mut y_ans = 0;
        let mut z_ans = 0;
        let mut x_count = 0;
        let mut y_count = 0;
        let mut z_count = 0;

        while count < 1000000 {
            calculate_velocities(&mut moons);
            move_moons(&mut moons);

            if moons[0].x == moons[0].x_0
                && moons[1].x == moons[1].x_0
                && moons[2].x == moons[2].x_0
                && moons[3].x == moons[3].x_0
                && moons[0].xv == 0
                && moons[1].xv == 0
                && moons[2].xv == 0
                && moons[3].xv == 0
            {
                x_ans = count-x_count;
                x_count = count;
            }

            if moons[0].y == moons[0].y_0
                && moons[1].y == moons[1].y_0
                && moons[2].y == moons[2].y_0
                && moons[3].y == moons[3].y_0
                && moons[0].yv == 0
                && moons[1].yv == 0
                && moons[2].yv == 0
                && moons[3].yv == 0
            {
                y_ans = count-y_count;
                y_count = count;
            }

            if moons[0].z == moons[0].z_0
                && moons[1].z == moons[1].z_0
                && moons[2].z == moons[2].z_0
                && moons[3].z == moons[3].z_0
                && moons[0].zv == 0
                && moons[1].zv == 0
                && moons[2].zv == 0
                && moons[3].zv == 0
            {
                z_ans = count-z_count;
                z_count = count;
            }

            count += 1;
        }

        println!("{} {} {} ", x_ans, y_ans, z_ans);
        assert_eq!((x_ans * y_ans * z_ans) % 4686774924, 0);
    }
}
