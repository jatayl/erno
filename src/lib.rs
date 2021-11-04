use std::{default, fmt};

use colored::Colorize;

// do we want this??
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Color {
    White,
    Green,
    Red,
    Blue,
    Orange,
    Yellow,
}

impl Color {
    fn to_abbr(&self) -> String {
        match self {
            Color::White => "W".color("white").to_string(),
            Color::Green => "G".color("green").to_string(),
            Color::Red => "R".color("red").to_string(),
            Color::Blue => "B".color("blue").to_string(),
            Color::Orange => "O".truecolor(255, 165, 0).to_string(),
            Color::Yellow => "Y".color("yellow").to_string(),
        }
    }
}

// maybe could make a 144 bit thing instead of (192)
#[derive(Debug, PartialEq, Eq, Clone)]
struct Cube {
    // index 0 to 5 (white, blue, red, green, orange, yellow)
    // within the u32
    faces: [u32; 6],
}

// would be nice to have a convienience function for each sticker

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_top = |face: &[String]| format!(" {} {} {} ", face[0], face[1], face[2]);
        let format_mid =
            |face: &[String], c: Color| format!(" {} {} {} ", face[3], c.to_abbr(), face[4]);
        let format_bot = |face: &[String]| format!(" {} {} {} ", face[5], face[6], face[7]);

        let faces: Vec<Vec<_>> = self
            .faces
            .iter()
            .map(|face| {
                (0..8)
                    .map(|ss| (face >> (ss * 4)) % 16)
                    .map(|n| FromPrimitive::from_u32(n).unwrap())
                    .map(|c: Color| c.to_abbr())
                    .collect()
            })
            .collect();

        writeln!(f, "        +-------+")?;
        writeln!(f, "        |{}|", format_top(&faces[0]))?;
        writeln!(f, "        |{}|", format_mid(&faces[0], Color::White))?;
        writeln!(f, "        |{}|", format_bot(&faces[0]))?;
        writeln!(f, "+-------+-------+-------+-------+")?;
        writeln!(
            f,
            "|{}|{}|{}|{}|",
            format_top(&faces[1]),
            format_top(&faces[2]),
            format_top(&faces[3]),
            format_top(&faces[4])
        )?;
        writeln!(
            f,
            "|{}|{}|{}|{}|",
            format_mid(&faces[1], Color::Green),
            format_mid(&faces[2], Color::Red),
            format_mid(&faces[3], Color::Blue),
            format_mid(&faces[4], Color::Orange)
        )?;
        writeln!(
            f,
            "|{}|{}|{}|{}|",
            format_bot(&faces[1]),
            format_bot(&faces[2]),
            format_bot(&faces[3]),
            format_bot(&faces[4])
        )?;
        writeln!(f, "+-------+-------+-------+-------+")?;
        writeln!(f, "        |{}|", format_top(&faces[5]))?;
        writeln!(f, "        |{}|", format_mid(&faces[5], Color::Yellow))?;
        writeln!(f, "        |{}|", format_bot(&faces[5]))?;
        write!(f, "        +-------+")
    }
}

impl default::Default for Cube {
    fn default() -> Self {
        Cube {
            faces: [
                0, 0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c = Cube::default();

        println!("size: {} bytes", std::mem::size_of::<Cube>());

        println!("{}", c);
        panic!()
    }
}
