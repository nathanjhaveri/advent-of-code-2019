use intcode::{IntCode, Op};
use std::collections::{HashMap, HashSet, VecDeque};
use termion::cursor;

pub const FIFTEEN: &str = "3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,102,1,1034,1039,1002,1036,1,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1002,1036,1,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,101,0,1038,1043,1002,1037,1,1042,1105,1,124,1001,1034,1,1039,1008,1036,0,1041,1001,1035,0,1040,1001,1038,0,1043,1001,1037,0,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,1,1032,1006,1032,165,1008,1040,5,1032,1006,1032,165,1101,2,0,1044,1106,0,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,72,1044,1105,1,224,1101,0,0,1044,1105,1,224,1006,1044,247,1001,1039,0,1034,101,0,1040,1035,1001,1041,0,1036,1001,1043,0,1038,1001,1042,0,1037,4,1044,1106,0,0,50,46,95,30,15,91,60,70,74,3,22,60,94,68,47,99,65,61,23,17,82,21,80,87,27,62,53,46,89,98,55,64,15,41,82,13,45,78,18,28,87,17,24,22,81,92,30,70,97,22,85,71,32,73,35,93,78,54,85,45,46,75,51,97,73,85,37,87,29,92,85,75,10,21,79,60,85,31,79,73,7,81,4,77,45,17,82,78,37,85,95,83,17,56,52,85,79,78,32,91,79,37,75,51,46,20,21,16,93,87,22,42,74,87,22,84,20,69,35,97,88,76,78,85,26,64,84,80,38,92,58,87,84,98,38,20,75,78,69,80,47,54,78,95,85,90,24,44,84,74,11,1,92,80,58,12,4,97,31,49,73,9,85,55,84,49,93,82,22,47,75,44,55,83,71,21,52,94,24,79,36,88,5,43,61,40,87,83,28,28,84,83,11,43,90,99,41,87,29,76,48,93,91,58,50,29,90,13,23,6,73,97,45,98,83,93,40,85,79,66,89,5,94,50,81,65,42,81,91,97,53,99,50,88,28,54,33,79,36,31,95,70,89,87,57,94,80,97,82,68,79,38,94,2,88,8,88,45,1,98,28,91,64,85,97,34,95,47,90,70,86,13,38,68,93,74,57,73,89,31,81,34,48,80,92,39,7,83,2,77,54,77,68,86,20,64,86,32,81,6,73,37,59,82,47,86,19,86,45,92,82,56,57,94,54,9,9,76,14,9,85,81,84,42,86,60,68,89,15,75,42,49,93,2,97,83,83,64,87,85,71,73,3,36,94,5,8,25,82,11,86,36,37,93,79,31,92,84,25,90,9,83,68,71,81,28,84,17,88,71,69,87,7,87,56,98,5,66,94,80,83,43,95,92,7,73,90,23,7,11,60,3,89,92,30,95,98,1,94,27,95,68,15,86,42,92,48,8,77,91,52,76,68,41,88,94,83,25,28,75,36,87,56,39,77,68,77,96,44,85,97,14,41,73,97,52,62,99,34,54,78,87,24,92,84,95,64,45,76,11,83,98,32,98,25,76,33,79,11,93,94,46,93,27,46,75,92,43,30,11,52,96,15,8,98,94,47,73,80,54,84,18,92,64,39,92,93,95,77,64,94,28,88,49,73,43,39,82,58,41,87,91,22,32,48,87,39,61,85,74,91,17,92,90,52,78,53,49,28,22,79,51,75,53,89,28,3,81,22,64,19,51,77,34,78,88,36,83,91,40,11,74,75,19,91,27,12,34,93,24,82,90,43,42,94,66,86,85,62,93,12,78,81,57,75,81,63,54,99,97,83,6,94,90,50,66,94,39,83,35,78,76,57,79,45,27,88,53,55,18,97,4,49,89,42,51,74,46,93,87,24,97,58,35,85,89,30,90,4,89,46,91,67,99,91,91,70,24,97,30,48,77,82,46,94,63,90,89,45,82,32,88,25,37,75,85,73,68,9,94,39,68,83,54,22,87,84,42,98,41,87,65,80,54,23,54,17,83,98,17,90,1,96,55,85,63,66,95,78,84,77,73,60,27,94,21,79,90,62,90,85,11,87,83,26,88,61,75,60,47,80,6,36,84,79,99,61,79,12,38,76,17,45,88,83,15,74,66,38,88,23,44,87,77,33,78,56,23,45,52,83,89,71,52,74,17,75,52,80,95,83,28,69,87,57,52,94,80,9,90,63,91,45,85,31,90,47,78,40,74,80,75,11,95,18,97,84,73,63,87,45,74,30,81,16,95,31,93,68,81,9,79,74,94,33,83,66,76,52,80,0,0,21,21,1,10,1,0,0,0,0,0,0";
type Pos = (i32, i32);
type Dir = Op;

const NORTH: Dir = 1;
const SOUTH: Dir = 2;
const WEST: Dir = 3;
const EAST: Dir = 4;

const HIT_WALL: Op = 0;
const MOVED: Op = 1;
const FOUND_O2: Op = 2;

fn offset_pos(pos: Pos, dir: Dir) -> Pos {
    match dir {
        NORTH => (pos.0, pos.1 + 1),
        EAST => (pos.0 + 1, pos.1),
        SOUTH => (pos.0, pos.1 - 1),
        WEST => (pos.0 - 1, pos.1),
        _ => panic!("unexpected dir"),
    }
}

fn opposite_dir(dir: Dir) -> Dir {
    match dir {
        NORTH => SOUTH,
        EAST => WEST,
        SOUTH => NORTH,
        WEST => EAST,
        _ => panic!("unexpected dir"),
    }
}

fn find_distance_to_root(parents: &HashMap<Pos, Pos>, pos: Pos) -> usize {
    let mut distance = 0;
    let mut pos = pos;
    while let Some(&parent) = parents.get(&pos) {
        distance += 1;
        pos = parent;
    }

    distance
}

fn dir_from_pos_to_pos(start: Pos, end: Pos) -> Dir {
    if start.0 == end.0 {
        // Move N/S
        if end.1 - start.1 == 1 {
            return NORTH;
        } else if end.1 - start.1 == -1 {
            return SOUTH;
        }
    } else if start.1 == end.1 {
        // Move E/W
        if end.0 - start.0 == 1 {
            return EAST;
        } else if end.0 - start.0 == -1 {
            return WEST;
        }
    }

    panic!("Can't move from {:?} to {:?} in one step", start, end);
}

fn move_to_pos(computer: &mut IntCode, parents: &HashMap<Pos, Pos>, current_pos: Pos, pos: Pos) {
    //println!(
    //    "move to pos {:?}, {:?}, parents\n{:?}",
    //    current_pos, pos, parents
    //);
    // Probably not very efficent, but should be simple.  Just
    // go from current pos to root, and then from root to pos.
    let mut current_pos = current_pos;
    while let Some(&parent_pos) = parents.get(&current_pos) {
        let dir = dir_from_pos_to_pos(current_pos, parent_pos);
        computer.input(dir);
        computer.compute_output();

        print_robot(current_pos, Glyph::Path);
        current_pos = parent_pos;
        print_robot(current_pos, Glyph::Robot);
    }

    assert_eq!(current_pos, (0, 0));

    let mut pos = pos;
    let mut path_to_pos = Vec::new();
    while let Some(&parent_pos) = parents.get(&pos) {
        path_to_pos.push(pos);
        pos = parent_pos;
    }

    while let Some(step) = path_to_pos.pop() {
        let dir = dir_from_pos_to_pos(current_pos, step);
        computer.input(dir);
        computer.compute_output();

        print_robot(current_pos, Glyph::Path);
        current_pos = step;
        print_robot(current_pos, Glyph::Robot);
    }
}

enum Glyph {
    Path,
    Wall,
    Robot,
    O2,
}

fn print_robot(_pos: Pos, _glyph: Glyph) {
    //let x = (100 + pos.0) as u16;
    //let y = (15 + pos.1) as u16;

    //let loc = cursor::Goto(x, y);

    //match glyph {
    //    Glyph::Path => print!("{}.", loc),
    //    Glyph::Wall => print!("{}#", loc),
    //    Glyph::Robot => print!("{}X", loc),
    //    Glyph::O2 => print!("{}@", loc),
    //}
}

pub fn find_oxygen() -> usize {
    let mut computer = IntCode::new(FIFTEEN);
    let start = (0, 0);
    let mut current_pos = (0, 0);

    // Do a BFS with the robot
    let mut queue = VecDeque::new();
    let mut discovered = HashSet::new();
    let mut parents = HashMap::new();

    queue.push_back(start);
    discovered.insert(start);
    print_robot(start, Glyph::Robot);

    while let Some(pos) = queue.pop_front() {
        if current_pos != pos && parents.get(&pos).is_some() {
            move_to_pos(&mut computer, &parents, current_pos, pos);
            current_pos = pos;
        }

        for &dir in [NORTH, EAST, SOUTH, WEST].iter() {
            let new_pos = offset_pos(current_pos, dir);
            if !discovered.contains(&new_pos) {
                discovered.insert(new_pos);
                parents.insert(new_pos, current_pos);

                computer.input(dir);
                let result = computer.compute_output().unwrap();

                match result {
                    HIT_WALL => print_robot(new_pos, Glyph::Wall),
                    MOVED => {
                        // move back
                        print_robot(new_pos, Glyph::Path);
                        computer.input(opposite_dir(dir));
                        computer.compute_output();
                        queue.push_back(new_pos);
                    }
                    FOUND_O2 => {
                        print_robot(new_pos, Glyph::O2);
                        return find_distance_to_root(&parents, new_pos);
                    }
                    _ => panic!("unexpected output"),
                }
            }
        }
    }

    panic!("o2 not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find() {
        assert_eq!(412, find_oxygen());
    }
}
