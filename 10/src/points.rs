type Pt = i32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Pt,
    pub y: Pt,
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

    pub fn from_cords(x: usize, y: usize) -> Point {
        Point {
            x: x as Pt,
            y: y as Pt,
        }
    }

    // New point which represents this point relative
    // to a different orign point
    pub fn relative_to(self, origin: Point) -> Point {
        Point {
            x: self.x - origin.x,
            y: self.y - origin.y,
        }
    }
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
}
