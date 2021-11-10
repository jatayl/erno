// we can use this instead of u32

use std::{default, fmt};

use colored::Colorize;

// do we want this??
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[cfg(feature = "sample")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    face: Face,
    dir: Direction,
}

impl Rotation {
    pub fn new(face: Face, dir: Direction) -> Self {
        Rotation { face, dir }
    }
}

#[cfg(feature = "sample")]
impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Rotation {
        let face = rand::random();
        let dir = rand::random();
        Rotation { face, dir }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Cw,
    Ccw,
}

#[cfg(feature = "sample")]
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..2) {
            0 => Direction::Cw,
            _ => Direction::Ccw,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
}

#[cfg(feature = "sample")]
impl Distribution<Face> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.gen_range(0..6) {
            0 => Face::U,
            1 => Face::L,
            2 => Face::F,
            3 => Face::R,
            4 => Face::B,
            _ => Face::D,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube {
    // index 0 to 5 (white, blue, red, green, orange, yellow)
    // could compress this to u24
    faces: [u32; 6],
}

impl Cube {
    pub fn is_solved(&self) -> bool {
        *self == Cube::default()
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        let mut copy = self.clone();
        copy.rotate_in_place(rotation);
        copy
    }

    pub fn rotate_in_place(&mut self, rotation: Rotation) {
        // this will be fun to implement
        let face_index = rotation.face as usize;
        if rotation.dir == Direction::Cw {
            self.faces[face_index] = self.faces[face_index].rotate_left(4 * 2);
        } else {
            self.faces[face_index] = self.faces[face_index].rotate_right(4 * 2);
        }

        // maybe make this more explainable
        // this is for ccw
        let emb = match rotation.face {
            Face::U => [(4, 0), (3, 0), (2, 0), (1, 0)],
            Face::L => [(0, 6), (2, 6), (5, 6), (4, 2)],
            Face::F => [(3, 6), (5, 0), (1, 2), (0, 4)],
            Face::R => [(0, 2), (4, 6), (5, 2), (2, 2)],
            Face::B => [(0, 0), (1, 6), (5, 4), (3, 2)],
            Face::D => [(2, 4), (3, 4), (4, 4), (1, 4)],
        };

        let emb_inter = emb.iter().zip(emb[1..].iter());

        let emb_inter: Vec<_> = if rotation.dir == Direction::Cw {
            emb_inter.rev().collect()
        } else {
            emb_inter.collect()
        };

        for (&(face_l, sticker_l), &(face_r, sticker_r)) in emb_inter.into_iter() {
            // need swap helper function that swaps two stickers
            self.swap_stickers((face_l, sticker_l), (face_r, sticker_r));
            self.swap_stickers((face_l, (sticker_l + 1) % 8), (face_r, (sticker_r + 1) % 8));
            self.swap_stickers((face_l, (sticker_l + 2) % 8), (face_r, (sticker_r + 2) % 8));
        }
    }

    pub fn rotate_many(&self, rotations: impl AsRef<[Rotation]>) -> Self {
        let mut copy = self.clone();
        copy.rotate_many_in_place(rotations);
        copy
    }

    pub fn rotate_many_in_place(&mut self, rotations: impl AsRef<[Rotation]>) {
        let rotations = rotations.as_ref();
        for &r in rotations.iter() {
            self.rotate_in_place(r);
        }
    }

    #[inline]
    fn swap_stickers(&mut self, left: (usize, usize), right: (usize, usize)) {
        let (face_left, sticker_left_i) = left;
        let (face_right, sticker_right_i) = right;

        let sticker_left = (self.faces[face_left] >> (sticker_left_i * 4)) & 0x7;
        let sticker_right = (self.faces[face_right] >> (sticker_right_i * 4)) & 0x7;

        let stickers_swap = sticker_left ^ sticker_right;

        self.faces[face_left] ^= stickers_swap << (sticker_left_i * 4);
        self.faces[face_right] ^= stickers_swap << (sticker_right_i * 4);
    }
}

// would be nice to have a convienience function for each sticker

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_top = |face: &[String]| format!(" {} {} {} ", face[0], face[1], face[2]);
        let format_mid =
            |face: &[String], c: Color| format!(" {} {} {} ", face[7], c.to_abbr(), face[3]);
        let format_bot = |face: &[String]| format!(" {} {} {} ", face[6], face[5], face[4]);

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
    fn is_solved() {
        let mut c = Cube::default();
        assert!(c.is_solved());

        c.rotate_in_place(Rotation::new(Face::B, Direction::Ccw));
        assert!(!c.is_solved());

        c.rotate_in_place(Rotation::new(Face::B, Direction::Cw));
        assert!(c.is_solved());
    }

    #[test]
    fn swap_stickers() {
        // reflexive
        let mut a = Cube::default();
        let mut b = Cube::default();
        a.swap_stickers((0, 1), (3, 4));
        b.swap_stickers((3, 4), (0, 1));
        assert_eq!(a, b);

        // inverse
        let mut a = Cube::default();
        a.swap_stickers((0, 1), (3, 4));
        a.swap_stickers((3, 4), (0, 1));
        assert_eq!(a, Cube::default());
    }

    #[test]
    fn rotate_order() {
        let mut c = Cube::default();

        // this would be a cool declarative macro
        // let p = permutation!(R U' R' U); // ?

        let p = [
            Rotation::new(Face::R, Direction::Cw),
            Rotation::new(Face::U, Direction::Ccw),
            Rotation::new(Face::R, Direction::Ccw),
            Rotation::new(Face::U, Direction::Cw),
        ];
        for _ in 0..6 {
            c.rotate_many_in_place(p);
        }
        assert_eq!(c, Cube::default());

        let t_perm = [
            Rotation::new(Face::R, Direction::Cw),
            Rotation::new(Face::U, Direction::Cw),
            Rotation::new(Face::R, Direction::Ccw),
            Rotation::new(Face::U, Direction::Ccw),
            Rotation::new(Face::R, Direction::Ccw),
            Rotation::new(Face::F, Direction::Cw),
            Rotation::new(Face::R, Direction::Cw),
            Rotation::new(Face::R, Direction::Cw),
            Rotation::new(Face::U, Direction::Ccw),
            Rotation::new(Face::R, Direction::Ccw),
            Rotation::new(Face::U, Direction::Ccw),
            Rotation::new(Face::R, Direction::Cw),
            Rotation::new(Face::U, Direction::Cw),
            Rotation::new(Face::R, Direction::Ccw),
            Rotation::new(Face::F, Direction::Ccw),
        ];
        for _ in 0..2 {
            c.rotate_many_in_place(t_perm);
            println!("{}", c);
        }
        assert_eq!(c, Cube::default());

        let back_t_perm = [
            Rotation::new(Face::L, Direction::Ccw),
            Rotation::new(Face::D, Direction::Ccw),
            Rotation::new(Face::L, Direction::Cw),
            Rotation::new(Face::D, Direction::Cw),
            Rotation::new(Face::L, Direction::Cw),
            Rotation::new(Face::B, Direction::Ccw),
            Rotation::new(Face::L, Direction::Ccw),
            Rotation::new(Face::L, Direction::Ccw),
            Rotation::new(Face::D, Direction::Cw),
            Rotation::new(Face::L, Direction::Cw),
            Rotation::new(Face::D, Direction::Cw),
            Rotation::new(Face::L, Direction::Ccw),
            Rotation::new(Face::D, Direction::Ccw),
            Rotation::new(Face::L, Direction::Cw),
            Rotation::new(Face::B, Direction::Cw),
        ];
        for _ in 0..2 {
            c.rotate_many_in_place(back_t_perm);
        }
        assert_eq!(c, Cube::default());
    }
}
