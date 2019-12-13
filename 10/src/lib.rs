mod points;

use points::Point;
use std::collections::HashSet;

const ASTROID: char = '#';

pub fn max_visible_points(input: &str) -> (Point, usize) {
    let points = parse(input);
    find_max_visible_points(&points)
}

pub fn find_max_visible_points(points: &[Point]) -> (Point, usize) {
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
    parse_with_factory(input, Point::from_cords)
}

fn parse_with_factory<T>(input: &str, factory: fn(usize, usize) -> T) -> Vec<T> {
    let mut map = Vec::new();
    let lines = input.split('\n').map(|line| line.trim());
    for (y, line) in lines.enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == ASTROID {
                map.push(factory(x, y));
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    const PUZZLE_10: &str = "###..#.##.####.##..###.#.#..
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

    #[test]
    fn ten_1() {
        let (point, visible) = max_visible_points(PUZZLE_10);
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
