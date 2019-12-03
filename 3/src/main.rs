use std::error::Error;
use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

struct Line {
    pub start: Point,
    pub end: Point,
}

fn main() -> Result<(), Box<dyn Error>> {

    let input = read_to_string("input.txt")?;
    let mut segments = input.split("\n");
    let wire1 = wire_segments(segments.next().unwrap());
    let wire2 = wire_segments(segments.next().unwrap());

    let intersections = find_intersections(wire1, wire2);

    println!("Hello, world!");

    Ok(())
}

fn wire_segments(line: &str) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;

    let segments: Vec<Line> = line
        .split(",")
        .map(|op| -> Line {
            let count: i32 = op[1..].parse().unwrap();
            let dir = &op[0..1];
            println!("dir {} count {}", dir, count);
            let start = Point { x, y };
            match dir {
                "U" => y += count,
                "D" => y -= count,
                "L" => x -= count,
                "R" => x += count,
                _ => panic!("unexpected direction"),
            }

            let end = Point { x, y };
            Line { start, end }
        })
        .collect();

    segments
}

fn find_intersections(wire1: Vec<Line>, wire2: Vec<Line>) -> Vec<Point> {
    Vec::new()
}

fn intersection(line1: Line, line2: Line) -> Option<Point> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_itersection() {
        let origin = Point { x: 0, y: 0 };
        let y = Point { x: 0, y: 1 };
        let x = Point { x: 1, y: 0 };
        let line1 = Line { start: origin, end: y };
        let line2 = Line { start: origin, end: x };

        let intersection = intersection(line1, line2);

        assert_eq!(intersection, Some(origin))
    }
}


