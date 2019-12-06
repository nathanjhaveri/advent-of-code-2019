use std::collections::HashMap;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;
const COM: &str = "COM";
const SAN: &str = "SAN";
const YOU: &str = "YOU";

pub fn parse_input(input: &str) -> OrbitMap {
    let orbit_map: HashMap<_, _> = input
        .lines()
        .map(|line| -> (&str, &str) {
            let objects: Vec<&str> = line.trim().split(')').collect();
            // Orbitor second, orbitee first
            (objects[1], objects[0])
        })
        .collect();

    orbit_map
}

fn count_all_orbits(orbit_map: &OrbitMap) -> u32 {
    orbit_map
        .keys()
        .map(|object| count_distance(orbit_map, object, COM))
        .sum()
}

fn count_distance(orbit_map: &OrbitMap, start: &str, end: &str) -> u32 {
    let mut count = 0;
    let mut current = start;
    while current != end && current != COM {
        current = orbit_map[current];
        count += 1;
    }

    count
}

fn distance_to_santa(orbit_map: &OrbitMap) -> u32 {
    let mut you_pos = YOU;
    let mut san_pos = SAN;

    let you_com_distance = count_distance(orbit_map, you_pos, COM);
    let san_com_distance = count_distance(orbit_map, san_pos, COM);

    if you_com_distance > san_com_distance {
        let steps = you_com_distance - san_com_distance;
        for _ in 0..steps {
            you_pos = orbit_map[you_pos]
        }
    } else if san_com_distance > you_com_distance {
        let steps = san_com_distance - you_com_distance;
        for _ in 0..steps {
            san_pos = orbit_map[san_pos]
        }
    }

    while you_pos != san_pos {
        you_pos = orbit_map[you_pos];
        san_pos = orbit_map[san_pos];
    }

    let common_parent = you_pos;
    let you_parent_dist = count_distance(orbit_map, YOU, common_parent);
    let san_parent_dist = count_distance(orbit_map, SAN, common_parent);

    you_parent_dist + san_parent_dist - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn six_one() {
        let filename = "6-1-input.txt";
        let input = read_to_string(filename).unwrap();
        let orbit_map = parse_input(&input);
        let sum = count_all_orbits(&orbit_map);

        assert_eq!(227_612, sum);
    }

    #[test]
    fn six_two() {
        let filename = "6-1-input.txt";
        let input = read_to_string(filename).unwrap();
        let orbit_map = parse_input(&input);
        let dist = distance_to_santa(&orbit_map);

        assert_eq!(454, dist);
    }

    #[test]
    fn example_input() {
        let input = "COM)B
                     B)C
                     C)D
                     D)E
                     E)F
                     B)G
                     G)H
                     D)I
                     E)J
                     J)K
                     K)L";
        let orbit_map = parse_input(input);
        let sum = count_all_orbits(&orbit_map);

        println!("orbit_map {:?}", orbit_map);
        assert_eq!(sum, 42);
    }

    #[test]
    fn example_santa() {
        let input = "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN";
        let orbit_map = parse_input(input);
        let sum = distance_to_santa(&orbit_map);

        assert_eq!(sum, 4);
    }
}
