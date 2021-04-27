use std::{
    fmt::Display,
    ops::{Div, Mul},
};

use colored::Colorize;
use rand::distributions::{Distribution, Standard};

#[derive(PartialEq, Eq, Debug)]
enum Winner {
    Undecided,
    Fisch,
    Boot,
    Unentschieden,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Blau,
    Orange,
    Gelb,
    Rosa,
    Rot,
    Gruen,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Color::Blau => "BLAU".on_blue(),
            Color::Orange => "ORANGE".on_bright_red(),
            Color::Gelb => "GELB".on_yellow(),
            Color::Rosa => "ROSA".on_magenta(),
            Color::Rot => "ROT".on_red(),
            Color::Gruen => "GRUEN".on_green(),
        };

        write!(f, "{}", name)
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=5) {
            0 => Color::Blau,
            1 => Color::Orange,
            2 => Color::Gelb,
            3 => Color::Rosa,
            4 => Color::Rot,
            _ => Color::Gruen,
        }
    }
}

fn d6() -> Color {
    rand::random()
}

#[derive(Debug, Copy, Clone)]
struct Fisch(Color);

struct Fische([Option<Fisch>; 4]);

impl Default for Fische {
    fn default() -> Self {
        Fische([None, None, None, None])
    }
}

impl Fische {
    fn fill() -> Self {
        Fische([
            Some(Fisch(Color::Blau)),
            Some(Fisch(Color::Orange)),
            Some(Fisch(Color::Gelb)),
            Some(Fisch(Color::Rosa)),
        ])
    }

    fn get_blau(&self) -> &Option<Fisch> {
        &self.0[0]
    }

    fn get_mut_blau(&mut self) -> &mut Option<Fisch> {
        &mut self.0[0]
    }

    fn get_orange(&self) -> &Option<Fisch> {
        &self.0[1]
    }

    fn get_mut_orange(&mut self) -> &mut Option<Fisch> {
        &mut self.0[1]
    }

    fn get_gelb(&self) -> &Option<Fisch> {
        &self.0[2]
    }

    fn get_mut_gelb(&mut self) -> &mut Option<Fisch> {
        &mut self.0[2]
    }

    fn get_rosa(&self) -> &Option<Fisch> {
        &self.0[3]
    }

    fn get_mut_rosa(&mut self) -> &mut Option<Fisch> {
        &mut self.0[3]
    }

    fn get_mut_col(&mut self, col: &Color) -> &mut Option<Fisch> {
        match col {
            Color::Blau => self.get_mut_blau(),
            Color::Orange => self.get_mut_orange(),
            Color::Gelb => self.get_mut_gelb(),
            Color::Rosa => self.get_mut_rosa(),
            _ => panic!("Not a fishcolor"),
        }
    }

    pub fn has(&self, col: &Color) -> bool {
        match col {
            Color::Blau => self.get_blau(),
            Color::Orange => self.get_orange(),
            Color::Gelb => self.get_gelb(),
            Color::Rosa => self.get_rosa(),
            _ => &None,
        }
        .is_some()
    }

    pub fn get_first(&self) -> &Option<Fisch> {
        if self.has(&Color::Blau) {
            self.get_blau()
        } else if self.has(&Color::Orange) {
            self.get_orange()
        } else if self.has(&Color::Gelb) {
            self.get_gelb()
        } else if self.has(&Color::Rosa) {
            self.get_rosa()
        } else {
            &None
        }
    }

    pub fn extract_first(&mut self) -> Option<Fisch> {
        let col = self.get_first().map(|f| f.0);

        if let Some(col) = col {
            self.get_mut_col(&col).take()
        } else {
            None
        }
    }

    pub fn rm(&mut self, col: &Color) {
        self.get_mut_col(col).take();
    }

    pub fn add(&mut self, col: Color) {
        self.get_mut_col(&col).replace(Fisch(col));
    }
}

struct Game {
    fluss: [Fische; 13],
    fisch_cols: Fische,
    free_cols: Fische,
    caught_cols: Fische,
    boot_pos: u8,
    last_col: Color,
}

impl Game {
    fn new() -> Self {
        Game {
            fluss: [
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische([
                    Some(Fisch(Color::Blau)),
                    Some(Fisch(Color::Orange)),
                    None,
                    None,
                ]),
                Fische([
                    None,
                    None,
                    Some(Fisch(Color::Gelb)),
                    Some(Fisch(Color::Rosa)),
                ]),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
            ],
            fisch_cols: Fische::fill(),
            free_cols: Fische::default(),
            caught_cols: Fische::default(),
            boot_pos: 0,
            last_col: Color::Blau,
        }
    }

    pub fn tick(&mut self) -> Winner {
        let free = self.free_cols.0.iter().filter(|f| f.is_some()).count();
        let caught = self.caught_cols.0.iter().filter(|f| f.is_some()).count();

        if self.fluss.len() - 1 == self.boot_pos.into() {
            if free == caught {
                Winner::Unentschieden
            } else if free > caught {
                Winner::Fisch
            } else {
                Winner::Boot
            }
        } else if free >= 3 {
            Winner::Fisch
        } else if caught >= 3 {
            Winner::Boot
        } else if free == 2 && caught == 2 {
            Winner::Unentschieden
        } else {
            let col = d6();

            self.mv(col);
            self.last_col = col;

            Winner::Undecided
        }
    }

    fn mv(&mut self, col: Color) {
        if self.fisch_cols.has(&col) {
            self.mv_fisch(&col);
        } else if self.free_cols.has(&col) {
            // find endangered species
            let fisch = self
                .fluss
                .iter()
                // .rev()
                .find(|f| f.get_first().is_some())
                .map(|f| f.get_first())
                .unwrap()
                .unwrap();

            self.mv_fisch(&fisch.0);
        } else {
            self.mv_boot();
        }
    }

    fn mv_fisch(&mut self, col: &Color) {
        let start: usize = (self.boot_pos + 1).into();

        if start < self.fluss.len() {
            for i in start..self.fluss.len() - 1 {
                if self.fluss[i].has(col) {
                    self.fluss[i].rm(col);
                    self.fluss[i + 1].add(*col);
                    break;
                }
            }
        }

        if let Some(fisch) = self.fluss[12].extract_first() {
            self.fisch_cols.rm(&fisch.0);
            self.free_cols.add(fisch.0);
        }
    }

    fn mv_boot(&mut self) {
        self.boot_pos += 1;

        let index: usize = self.boot_pos.into();

        while let Some(fisch) = self.fluss[index].extract_first() {
            self.fisch_cols.rm(&fisch.0);
            self.caught_cols.add(fisch.0);
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blau = self
            .fluss
            .iter()
            .map(|f| if f.has(&Color::Blau) { "x" } else { "=" })
            .collect::<Vec<&str>>()
            .join("")
            .blue();

        let orange = self
            .fluss
            .iter()
            .map(|f| if f.has(&Color::Orange) { "x" } else { "=" })
            .collect::<Vec<&str>>()
            .join("")
            .bright_red();

        let gelb = self
            .fluss
            .iter()
            .map(|f| if f.has(&Color::Gelb) { "x" } else { "=" })
            .collect::<Vec<&str>>()
            .join("")
            .yellow();

        let rosa = self
            .fluss
            .iter()
            .map(|f| if f.has(&Color::Rosa) { "x" } else { "=" })
            .collect::<Vec<&str>>()
            .join("")
            .magenta();

        let boot = self
            .fluss
            .iter()
            .enumerate()
            .map(|(i, _)| if i == self.boot_pos.into() { "v" } else { "_" })
            .collect::<Vec<&str>>()
            .join("");

        write!(f, "{}\n", self.last_col)?;
        write!(f, "{}\n", boot)?;
        write!(f, "{}\n", blau)?;
        write!(f, "{}\n", orange)?;
        write!(f, "{}\n", gelb)?;
        write!(f, "{}\n", rosa)?;

        Ok(())
    }
}

fn main() {
    // manual_game();
    benchmark();
}

fn benchmark() {
    let runs = 29718;
    let winners = (0..=runs)
        .step_by(1)
        .map(|_| {
            let mut g = Game::new();
            while g.tick() == Winner::Undecided {}
            g.tick()
        })
        .collect::<Vec<Winner>>();

    let b_win = winners.iter().filter(|w| w == &&Winner::Boot).count();
    let f_win = winners.iter().filter(|w| w == &&Winner::Fisch).count();
    let tie = winners
        .iter()
        .filter(|w| w == &&Winner::Unentschieden)
        .count();

    println!(
        "Boot: {} ({:.2}%)",
        b_win,
        (b_win as f32).div(runs as f32).mul(100f32)
    );
    println!(
        "Fisch: {} ({:.2}%)",
        f_win,
        (f_win as f32).div(runs as f32).mul(100f32)
    );
    println!(
        "Unentschieden: {} ({:.2}%)",
        tie,
        (tie as f32).div(runs as f32).mul(100f32)
    );
}

// fn manual_game() {
//     let mut g = Game::new();
//     println!("{}", g);

//     while g.tick() == Winner::Undecided {
//         println!("{}", g);
//         std::thread::sleep(std::time::Duration::from_millis(250))
//     }

//     println!("Winner: {:?}", g.tick());
// }
