use std::collections::HashSet;

const ASTROID: char = '#';

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn between(self, b: Point, c: Point) -> bool {
        // Returns true iff a is between c & d
        // Lower x value is "before".  If a.x == b.x == c.x, then
        // lower y value is "before".

        // before is smaller x, unless vertical line then smaller y
        let left_to_right_between = if self.x == b.x && self.x == c.x {
            let before = if b.y < c.y { b } else { c };
            let after = if b.y < c.y { c } else { b };

            before.y < self.y && self.y < after.y
        } else {
            let before = if b.x < c.x { b } else { c };
            let after = if b.x < c.x { c } else { b };

            before.x < self.x && self.x < after.x
        };

        left_to_right_between && self.collinear(b, c)
    }

    pub fn collinear(self, b: Point, c: Point) -> bool {
        points_collinear(self, b, c)
    }
}

pub fn max_visible_points(input: &str) -> (Point, usize) {
    let points = parse(input);
    // For each point P, if P sees all others then
    // the number visisble is points.len() - 1.  Each set of 3 points
    // that are on a line reduces the number visible by one.
    let mut max_visible = 0;
    let mut max_index = 0;
    let count = points.len();
    for i in 0..count {
        let mut hidden_from_i = HashSet::new();
        hidden_from_i.insert(i); // P can't see itself

        for j in 0..count {
            for k in (j + 1)..count {
                if points[j].between(points[i], points[k]) {
                    hidden_from_i.insert(k);
                } else if points[k].between(points[i], points[j]) {
                    hidden_from_i.insert(j);
                }
            }
        }

        let visible_from_i = count - hidden_from_i.len();
        if max_visible < visible_from_i {
            max_visible = visible_from_i;
            max_index = i;
        }
    }

    (points[max_index], max_visible)
}

fn parse(input: &str) -> Vec<Point> {
    let mut map = Vec::new();
    let lines = input.split('\n').map(|line| line.trim());
    for (y, line) in lines.enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == ASTROID {
                map.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    map
}

fn points_collinear(a: Point, b: Point, c: Point) -> bool {
    // Good idea from algorithm design manual:  points are on
    // a line if det(A) == 0 where
    //    | ax, ay, 1 |
    // A =| bx, by, 1 | = ax(by - cx) - ay(bx - cx) + (bxcy - cxby)
    //    | cx, cy, 1 |
    // This determinate is 2x area and can tell if point is above/below line
    // with 0 meaning on the line

    let det = a.x * (b.y - c.y) - a.y * (b.x - c.x) + b.x * c.y - c.x * b.y;
    0 == det
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_1() {
        let input = "###..#.##.####.##..###.#.#..
        #..#..###..#.......####.....
        #.###.#.##..###.##..#.###.#.
        ..#.##..##...#.#.###.##.####
        .#.##..####...####.###.##...
        ##...###.#.##.##..###..#..#.
        .##..###...#....###.....##.#
        #..##...#..#.##..####.....#.
        .#..#.######.#..#..####....#
        #.##.##......#..#..####.##..
        ##...#....#.#.##.#..#...##.#
        ##.####.###...#.##........##
        ......##.....#.###.##.#.#..#
        .###..#####.#..#...#...#.###
        ..##.###..##.#.##.#.##......
        ......##.#.#....#..##.#.####
        ...##..#.#.#.....##.###...##
        .#.#..#.#....##..##.#..#.#..
        ...#..###..##.####.#...#..##
        #.#......#.#..##..#...#.#..#
        ..#.##.#......#.##...#..#.##
        #.##..#....#...#.##..#..#..#
        #..#.#.#.##..#..#.#.#...##..
        .#...#.........#..#....#.#.#
        ..####.#..#..##.####.#.##.##
        .#.######......##..#.#.##.#.
        .#....####....###.#.#.#.####
        ....####...##.#.#...#..#.##.";

        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 282);
        assert_eq!(point, Point { x: 22, y: 19 });
    }

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
    fn collinear_points() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 2, y: 0 };
        let d = Point { x: 1, y: 1 };
        let e = Point { x: 1, y: 2 };
        let f = Point { x: 2, y: 2 };

        let horz_line = (a, b, c);
        let vert_line = (b, d, e);
        let diag_line = (a, d, f);

        let lines = vec![horz_line, vert_line, diag_line];

        for (i, line) in lines.iter().enumerate() {
            assert_eq!(
                points_collinear(line.0, line.1, line.2),
                true,
                "Points at index {} not on a line",
                i
            );
        }
    }

    #[test]
    fn non_collinear_points() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 2, y: 0 };
        let d = Point { x: 1, y: 1 };
        let e = Point { x: 1, y: 2 };
        let f = Point { x: 2, y: 2 };

        let tri1 = (a, c, f);
        let tri2 = (b, f, e);
        let tri3 = (a, d, e);

        let triangles = vec![tri1, tri2, tri3];

        for (i, triangle) in triangles.iter().enumerate() {
            assert_eq!(
                points_collinear(triangle.0, triangle.1, triangle.2),
                false,
                "Points at index {} on a line",
                i
            );
        }
    }

    #[test]
    fn ten_example_1() {
        let input = ".#..#
                     .....
                     #####
                     ....#
                     ...##";

        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 8);
        assert_eq!(point, Point { x: 3, y: 4 });
    }

    #[test]
    fn ten_example_2() {
        let input = "......#.#.
                     #..#.#....
                     ..#######.
                     .#.#.###..
                     .#..#.....
                     ..#....#.#
                     #..#....#.
                     .##.#..###
                     ##...#..#.
                     .#....####";

        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 33);
        assert_eq!(point, Point { x: 5, y: 8 });
    }

    #[test]
    fn ten_example_3() {
        let input = "#.#...#.#.
                     .###....#.
                     .#....#...
                     ##.#.#.#.#
                     ....#.#.#.
                     .##..###.#
                     ..#...##..
                     ..##....##
                     ......#...
                     .####.###.";
        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 35);
        assert_eq!(point, Point { x: 1, y: 2 });
    }

    #[test]
    fn ten_example_4() {
        let input = ".#..#..###
                     ####.###.#
                     ....###.#.
                     ..###.##.#
                     ##.##.#.#.
                     ....###..#
                     ..#.#..#.#
                     #..#.#.###
                     .##...##.#
                     .....#.#..";
        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 41);
        assert_eq!(point, Point { x: 6, y: 3 });
    }

    #[test]
    fn ten_example_5() {
        let input = ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##";

        let (point, visible) = max_visible_points(input);
        assert_eq!(visible, 210);
        assert_eq!(point, Point { x: 11, y: 13 });
    }
}
