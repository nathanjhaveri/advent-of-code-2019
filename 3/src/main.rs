use std::error::Error;
use std::fs::read_to_string;
use std::cmp;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
    travel_steps: u32
}

#[derive(Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let distance = find_distance(input)?;

    println!("distance: {}", distance);

    Ok(())
}

fn find_distance(input: String) -> Result<u32, Box<dyn Error>> {
    let mut segments = input.split("\n");
    let wire1 = wire_segments(segments.next().unwrap());
    let wire2 = wire_segments(segments.next().unwrap());

    let mut intersections = find_intersections(wire1, wire2);
    intersections.sort_by(|a, b| a.travel_steps.cmp(&b.travel_steps));
    let first = intersections.first().unwrap();

    //println!("first {:?}", intersections.first().unwrap());

    Ok(first.travel_steps)
}


fn wire_segments(line: &str) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;
    let mut acc: u32 = 0;

    let segments: Vec<Line> = line
        .split(",")
        .map(|op| -> Line {
            let count: i32 = op[1..].parse().unwrap();
            let dir = &op[0..1];
            let (endx, endy)  = match dir {
                "U" => (x, y + count),
                "D" => (x, y - count),
                "R" => (x + count, y),
                "L" => (x - count, y),
                _ => panic!("unexpected direction"),
            };

            let travel_steps = acc + count as u32;

            // Update current cursor position
            let segment = Line {
                start: Point { x, y, travel_steps: acc },
                end: Point { x: endx, y: endy, travel_steps },
            };

            x = endx;
            y = endy;
            acc = travel_steps;

            return segment;
        })
        .collect();

    segments
}

fn find_intersections(wire1: Vec<Line>, wire2: Vec<Line>) -> Vec<Point> {
    let mut intersections = Vec::new();
    for line1 in wire1.iter() {
        for line2 in wire2.iter() {
            let point = intersection(line1, line2);
            if point.is_some() {
                intersections.push(point.unwrap());
            }
        }
    }

    intersections
}

fn intersection(line1: &Line, line2: &Line) -> Option<Point> {
    if is_vertical(line1) && is_vertical(line2) {
        if line1.start.x == line2.start.x {
            println!("overlapping x");
        }
        return None;
    } else if is_horizontal(line1) && is_horizontal(line2) {
        if line1.start.y == line2.start.y {
            println!("overlapping y")
        }
        return None;
    } else {
        let horz = if is_vertical(line1) { line2 } else { line1 };
        let vert = if is_vertical(line1) { line1 } else { line2 };

        let y = horz.start.y;
        let x = vert.start.x;

        if x == 0 && y == 0 {
            return None
        }

        let miny = cmp::min(vert.start.y, vert.end.y);
        let maxy = cmp::max(vert.start.y, vert.end.y);
        let minx = cmp::min(horz.start.x, horz.end.x);
        let maxx = cmp::max(horz.start.x, horz.end.x);

        if miny <= y && y <= maxy && minx <= x && x <= maxx {
            let travel_steps: u32 = horz.start.travel_steps + vert.start.travel_steps + ((horz.start.x - x).abs() + (vert.start.y - y).abs()) as u32;
            let overlap = Point { x, y, travel_steps, };

            //println!("found intersecton\nLine1: {:?}\nLine2: {:?}\npoint: {:?}", horz, vert, overlap);
            return Some(overlap)
        }
    }

    None
}

fn is_vertical(line: &Line) -> bool {
    line.start.x == line.end.x
}

fn is_horizontal(line: &Line) -> bool {
    line.start.y == line.end.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_itersection() {
        let origin = Point { x: 1, y: 1, travel_steps: 0 };
        let y = Point { x: 3, y: 1, travel_steps: 0, };
        let x = Point { x: 1, y: 3, travel_steps: 0, };
        let line1 = Line { start: origin, end: y };
        let line2 = Line { start: origin, end: x };

        let intersection = intersection(&line1, &line2);

        assert_eq!(intersection, Some(origin))
    }

    #[test]
    fn example0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
        let distance = find_distance(input).unwrap();

        assert_eq!(30, distance);
    }

    #[test]
    fn example1() {
        let input: String = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let distance = find_distance(input).unwrap();

        assert_eq!(610, distance);
    }
}


