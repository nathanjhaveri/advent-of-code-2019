use intcode::{IntCode, Op};
use std::collections::HashMap;
use std::convert::TryFrom;

type Coord = (i32, i32);
type Panels = HashMap<Coord, Color>;
pub type PaintError = ();

// Colors
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl TryFrom<Op> for Color {
    type Error = ();

    fn try_from(item: Op) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(()),
        }
    }
}

enum Turn {
    CounterClockwise,
    Clockwise,
}

impl TryFrom<Op> for Turn {
    type Error = ();

    fn try_from(op: Op) -> Result<Self, Self::Error> {
        match op {
            0 => Ok(Self::CounterClockwise),
            1 => Ok(Self::Clockwise),
            _ => Err(()),
        }
    }
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn paint(program: &str, initial_color: Color) -> Result<Panels, PaintError> {
    let mut panels: HashMap<Coord, Color> = HashMap::new();
    let mut computer = IntCode::new(program);
    let mut pos = (0, 0);
    let mut dir = Dir::Up;

    loop {
        let panel_color: Color;
        if let Some(color) = panels.get(&pos) {
            panel_color = *color;
        } else {
            panel_color = initial_color;
        }

        computer.input(panel_color as Op);

        let new_color: Color = match computer.compute_output() {
            Some(output) => Color::try_from(output)?,
            None => return Ok(panels),
        };

        if let Some(dir_change_op) = computer.compute_output() {
            let turn = Turn::try_from(dir_change_op)?;
            dir = match turn {
                Turn::CounterClockwise => match dir {
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Down,
                },
                Turn::Clockwise => match dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                },
            };

            panels.insert(pos, new_color);
            pos = match dir {
                Dir::Up => (pos.0, pos.1 + 1),
                Dir::Down => (pos.0, pos.1 - 1),
                Dir::Right => (pos.0 + 1, pos.1),
                Dir::Left => (pos.0 - 1, pos.1),
            };
        } else {
            return Ok(panels);
        }
    }
}

pub fn print_panels(panels: &Panels) -> String {
    let min_x = panels
        .keys()
        .map(|&coord| coord.0)
        .min()
        .expect("No min x")
        .abs();
    let min_y = panels
        .keys()
        .map(|&coord| coord.1)
        .min()
        .expect("No y")
        .abs();

    // Offset by abs(min) to move printing window around content
    let max_x = panels
        .keys()
        .map(|&coord| coord.0 + min_x)
        .max()
        .expect("No x");
    let max_y = panels
        .keys()
        .map(|&coord| coord.1 + min_y)
        .max()
        .expect("No y");
    let mut out = String::with_capacity((max_x * max_y) as usize);

    for j in (0..=max_y).rev() {
        for i in 0..=max_x {
            let c = match panels.get(&(i - min_x, j - min_y)) {
                Some(&Color::Black) => ' ',
                Some(&Color::White) => '■',
                None => ' ',
            };

            out.push(c);
        }

        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const PROGRAM_11: &str = "3,8,1005,8,324,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,29,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,101,0,8,50,1,1106,9,10,1,102,15,10,2,1003,3,10,1,3,19,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,89,1,1105,9,10,2,1103,1,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,119,1006,0,26,1,109,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,147,1006,0,75,1,1005,17,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,176,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,199,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,220,2,103,10,10,1,1,0,10,1,102,17,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,254,2,1001,10,10,1006,0,12,1,3,6,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,288,2,1106,9,10,2,1009,6,10,2,1101,18,10,2,103,8,10,101,1,9,9,1007,9,1045,10,1005,10,15,99,109,646,104,0,104,1,21101,838211318676,0,1,21102,341,1,0,1106,0,445,21101,0,838211051932,1,21101,0,352,0,1106,0,445,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,21704576195,1,21101,0,399,0,1106,0,445,21101,0,179356830951,1,21101,410,0,0,1105,1,445,3,10,104,0,104,0,3,10,104,0,104,0,21102,837897052948,1,1,21102,1,433,0,1106,0,445,21102,709052085092,1,1,21102,1,444,0,1105,1,445,99,109,2,21201,-1,0,1,21101,0,40,2,21102,476,1,3,21102,466,1,0,1105,1,509,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,471,472,487,4,0,1001,471,1,471,108,4,471,10,1006,10,503,1102,1,0,471,109,-2,2106,0,0,0,109,4,2102,1,-1,508,1207,-3,0,10,1006,10,526,21101,0,0,-3,21201,-3,0,1,21201,-2,0,2,21101,0,1,3,21101,545,0,0,1105,1,550,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,573,2207,-4,-2,10,1006,10,573,21201,-4,0,-4,1105,1,641,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,592,0,0,1105,1,550,21201,1,0,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,611,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,633,21202,-1,1,1,21101,633,0,0,106,0,508,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";

    #[test]
    fn eleven_1() -> Result<(), PaintError> {
        let panels = paint(PROGRAM_11, Color::Black)?;
        assert_eq!(panels.len(), 2478);
        Ok(())
    }

    #[test]
    fn eleven_2() -> Result<(), PaintError> {
        let panels = paint(PROGRAM_11, Color::White)?;
        let out = print_panels(&panels);
        let expected = " ■  ■  ■■  ■■■■ ■■■  ■  ■  ■■   ■■  ■■■■   \n \
                        ■  ■ ■  ■    ■ ■  ■ ■  ■ ■  ■ ■  ■    ■   \n \
                        ■■■■ ■      ■  ■  ■ ■  ■ ■    ■  ■   ■    \n \
                        ■  ■ ■     ■   ■■■  ■  ■ ■ ■■ ■■■■  ■     \n \
                        ■  ■ ■  ■ ■    ■ ■  ■  ■ ■  ■ ■  ■ ■      \n \
                        ■  ■  ■■  ■■■■ ■  ■  ■■   ■■■ ■  ■ ■■■■   \n";
        assert_eq!(out, expected);
        Ok(())
    }
}
