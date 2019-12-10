#[derive(Debug, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Line {
    m: f64,
    b: f64,
}

const ASTROID: char = '#';

pub fn ten(input: &str) -> i32 {
    let points = parse(input);
    for point_a in points.iter() {
        for point_b in points.iter() {
            if point_a != point_b: {

            }
        }
    }
    1
}

fn lines(point: &Point, points: [&Point]) {
    for point_b in points {

    }
}

fn line(point_a: &Point, point_b: &Point) -> Line {
    Line { m: 0.0, b: 0.0 }
}

fn parse(input: &str) -> Vec<Point> {
    let mut map = Vec::new();
    let lines = input.split('\n').map(|line| line.trim());
    for (y, line) in lines.enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == ASTROID {
                map.push(Point { x, y });
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_parse() {
        let input = ".#..#
                     .....
                     #####
                     ....#
                     ...##";
        let points = parse(input);
        let expected = [
            Point { x: 1, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 2 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 },
        ];
        assert_eq!(points, expected);
    }

    #[test]
    fn ten_1() {
        let input = ".#..#
                     .....
                     #####
                     ....#
                     ...##";

        let out = ten(input);
        assert_eq!(8, out);
    }
}
