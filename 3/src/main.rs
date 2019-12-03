use std::error::Error;
use std::fs::read_to_string;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let distance = find_distance(input)?;

    println!("distnace: {}", distance);

    Ok(())
}

fn find_distance(input: String) -> Result<i32, Box<dyn Error>> {
    let mut segments = input.split("\n");
    let wire1 = wire_segments(segments.next().unwrap());
    let wire2 = wire_segments(segments.next().unwrap());

    let mut intersections = find_intersections(wire1, wire2);
    intersections.sort_by(|a, b| a.distance().cmp(&b.distance()));
    let first = intersections.first().unwrap();

    println!("first {}", intersections.first().unwrap().distance());
    println!("last {}", intersections.last().unwrap().distance());

    Ok(first.distance())
}


fn wire_segments(line: &str) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;

    let segments: Vec<Line> = line
        .split(",")
        .map(|op| -> Line {
            let count: i32 = op[1..].parse().unwrap();
            let dir = &op[0..1];
            let (startx, starty, endx, endy, newx, newy) = match dir {
                "U" => (x, y, x, y + count, x, y + count),
                "D" => (x, y - count, x, y, x, y - count),
                "R" => (x, y, x + count, y, x + count, y),
                "L" => (x - count, y, x, y, x - count, y),
                _ => panic!("unexpected direction"),
            };

            // Update current cursor position
            x = newx;
            y = newy;

            Line {
                start: Point { x: startx, y: starty },
                end: Point { x: endx, y: endy },
            }
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

        if vert.start.y <= y && y <= vert.end.y && horz.start.x <= x && x <= horz.end.x {
            //println!("found intersecton {:?} {:?}, point: {:?}", horz, vert, Point { x, y });
            return Some(Point { x, y })
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
        let origin = Point { x: 0, y: 0 };
        let y = Point { x: 0, y: 2 };
        let x = Point { x: 2, y: 0 };
        let line1 = Line { start: origin, end: y };
        let line2 = Line { start: origin, end: x };

        let intersection = intersection(&line1, &line2);

        assert_eq!(intersection, Some(origin))
    }

    #[test]
    fn example0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
        let distance = find_distance(input).unwrap();

        assert_eq!(6, distance);
    }

    #[test]
    fn example1() {
        let input: String = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let distance = find_distance(input).unwrap();

        assert_eq!(159, distance);
    }
}


