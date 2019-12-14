use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn calculate_total_depths(orbits: &HashMap<String, Vec<String>>) -> u32 {
    // Uses preorder traversal
    let mut result = 0;
    let mut path: Vec<(String, u32)> = Vec::new();
    path.push((String::from("COM"), 0));

    while path.len() > 0 {
        let curr = path.pop().unwrap();
        let nodes = orbits.get(&curr.0).unwrap();
        // println!("{:?}", path);
        result += curr.1;
        if nodes.len() > 0 {
            for n in nodes {
                path.push((n.to_string(), curr.1 + 1));
            }
        }
    }

    return result;
}

// fn traversal(orbits: &HashMap<String, Vec<String>>, current: String) {

// }

fn find_common_ancestor(orbits: &HashMap<String, Vec<String>>) -> usize {
    // Uses preorder traversal
    let mut result = 0;
    let mut path: Vec<(String, u32)> = Vec::new();
    path.push((String::from("COM"), 0));
    let mut route: Vec<String> = Vec::new();
    let mut route_to_santa: Vec<String> = Vec::new();
    let mut route_to_me: Vec<String> = Vec::new();


    while path.len() > 0 {
        let curr = path.pop().unwrap();
        let nodes = orbits.get(&curr.0).unwrap();
       
        if nodes.len() > 0 {
            route.push(curr.0);
            println!("route = {:?}", route);
            for n in nodes {
                if n == "SAN" {
                    // println!("Yo! Santa!");
                    route_to_santa = route.to_vec();
                }
                if n == "YOU" {
                    // println!("Here I am!");
                    route_to_me = route.to_vec();
                }
                path.push((n.to_string(), curr.1 + 1));
                println!("path = {:?}", path);
            }
        }
        else {
            route.remove(route.len()-1);
        }
    }

    println!("santa = {:?}", route_to_santa);
    println!("me = {:?}", route_to_me);

    let max_len = cmp::max(route_to_santa.len(), route_to_me.len());

    for i in 0..max_len {
        if route_to_santa[i] != route_to_me[i] {
            result = route_to_santa.len() - i + route_to_me.len() - i;
            break
        }
    }

    return result;
}

fn build_orbit_map(edges: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut orbit_map = HashMap::new();

    for pair in edges {
        let first = pair[0].to_string();
        let second = pair[1].to_string();

        if !orbit_map.contains_key(&first) {
            orbit_map.insert(first, Vec::new());
        }
        if !orbit_map.contains_key(&second) {
            orbit_map.insert(second, Vec::new());
        }
        orbit_map
            .get_mut(&pair[0].to_string())
            .unwrap()
            .push(pair[1].to_string());
    }

    return orbit_map;
}

fn string_to_edges(str: String) -> Vec<Vec<String>> {
    let lines = str.split('\n');
    let mut vec: Vec<Vec<String>> = Vec::new();
    for l in lines {
        let parts = l.split(')').map(|x| x.to_string()).collect();
        vec.push(parts);
    }
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
    let edges = string_to_edges(contents);
    let orbits = build_orbit_map(edges);
    // Part 1 = 270768
    println!("Result 1 = {}", calculate_total_depths(&orbits));
    // Part 2 = 
    println!("Result 2 = {}", find_common_ancestor(&orbits));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::build_orbit_map;
    use crate::calculate_total_depths;
    use crate::string_to_edges;
    use crate::find_common_ancestor;

    #[test]
    fn string_of_ints_gives_ints_as_vec() {
        let result = string_to_edges(String::from("COM)B\nB)C\nC)D\nD)E\nE)F"));
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], ["COM", "B"]);
        assert_eq!(result[4], ["E", "F"]);
    }

    #[test]
    fn it_works_for_example_1() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let edges = string_to_edges(input);
        let orbits = build_orbit_map(edges);

        assert_eq!(calculate_total_depths(&orbits), 42);
    }

    #[test]
    fn it_works_for_example_2() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
        let edges = string_to_edges(input);
        let orbits = build_orbit_map(edges);

        assert_eq!(find_common_ancestor(&orbits), 4);
    }

    #[test]
    fn it_works_for_example_3() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nH)YOU\nI)SAN");
        let edges = string_to_edges(input);
        let orbits = build_orbit_map(edges);

        assert_eq!(find_common_ancestor(&orbits), 46);
    }

}
