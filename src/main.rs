use std::{
    fmt::Display,
    ops::{Div, Mul},
    usize,
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
    None,
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
            Color::None => "Noch nich gewürfelt".black().on_white(),
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

#[derive(Debug, Copy, Clone)]
struct Fische {
    all: [Option<Fisch>; 4],
}

impl Default for Fische {
    fn default() -> Self {
        Fische {
            all: [None, None, None, None],
        }
    }
}

impl Fische {
    fn fill(cols: Vec<Color>) -> Self {
        let mut fisches = Fische::default();

        cols.iter().for_each(|col| fisches.add(*col));

        fisches
    }

    fn get_blau(&self) -> &Option<Fisch> {
        &self.all[0]
    }

    fn get_mut_blau(&mut self) -> &mut Option<Fisch> {
        &mut self.all[0]
    }

    fn get_orange(&self) -> &Option<Fisch> {
        &self.all[1]
    }

    fn get_mut_orange(&mut self) -> &mut Option<Fisch> {
        &mut self.all[1]
    }

    fn get_gelb(&self) -> &Option<Fisch> {
        &self.all[2]
    }

    fn get_mut_gelb(&mut self) -> &mut Option<Fisch> {
        &mut self.all[2]
    }

    fn get_rosa(&self) -> &Option<Fisch> {
        &self.all[3]
    }

    fn get_mut_rosa(&mut self) -> &mut Option<Fisch> {
        &mut self.all[3]
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

    pub fn iter(&self) -> core::slice::Iter<Option<Fisch>> {
        self.all.iter()
    }

    pub fn get_first(&self) -> &Option<Fisch> {
        self.all.iter().find(|f| f.is_some()).unwrap_or(&None)
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
    pub round: u8,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            fluss: [
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::fill(vec![Color::Blau, Color::Gelb, Color::Orange]),
                Fische::fill(vec![Color::Rosa]),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
                Fische::default(),
            ],
            fisch_cols: Fische::fill(vec![Color::Blau, Color::Orange, Color::Gelb, Color::Rosa]),
            free_cols: Fische::default(),
            caught_cols: Fische::default(),
            boot_pos: 0,
            last_col: Color::None,
            round: 0,
        }
    }
}

impl Game {
    fn new(fluss: [Fische; 13]) -> Self {
        Game {
            fluss,
            fisch_cols: Fische::fill(vec![Color::Blau, Color::Orange, Color::Gelb, Color::Rosa]),
            free_cols: Fische::default(),
            caught_cols: Fische::default(),
            boot_pos: 0,
            last_col: Color::None,
            round: 0,
        }
    }

    pub fn tick(&mut self) -> Winner {
        if let Some(winner) = self.check_for_winner() {
            winner
        } else {
            let col = d6();

            self.mv(col);
            self.last_col = col;
            self.round += 1;

            Winner::Undecided
        }
    }

    fn check_for_winner(&self) -> Option<Winner> {
        let free = self.free_cols.iter().filter(|f| f.is_some()).count();
        let caught = self.caught_cols.iter().filter(|f| f.is_some()).count();

        if self.fluss.len() - 1 == self.boot_pos.into() {
            if free == caught {
                Some(Winner::Unentschieden)
            } else if free > caught {
                Some(Winner::Fisch)
            } else {
                Some(Winner::Boot)
            }
        } else if free >= 3 {
            Some(Winner::Fisch)
        } else if caught >= 3 {
            Some(Winner::Boot)
        } else if free == 2 && caught == 2 {
            Some(Winner::Unentschieden)
        } else {
            None
        }
    }

    fn mv(&mut self, col: Color) {
        // If fisch is still in the game, move it
        if self.fisch_cols.has(&col) {
            self.mv_fisch(&col);
            // If fisch is free, move another one
        } else if self.free_cols.has(&col) {
            // find endangered species
            let fisch = self
                .fluss
                .iter()
                .find(|f| f.get_first().is_some())
                .map(|f| f.get_first())
                .unwrap()
                .unwrap();

            self.mv_fisch(&fisch.0);

            // Any other case, fisch has to be caught already so move the boot
        } else {
            self.mv_boot();
        }
    }

    fn mv_fisch(&mut self, col: &Color) {
        let start: usize = (self.boot_pos + 1).into();

        if start < self.fluss.len() {
            // Iterate only those parts of the fluss that are not covered
            // by the boat
            let index = self.fluss[start..]
                .iter()
                .enumerate()
                .find(|(_, f)| f.has(col))
                .map(|f| f.0 + start);

            if let Some(i) = index {
                self.fluss[i].rm(col);
                self.fluss[i + 1].add(*col);
            }
        }

        // If a fisch reached the ocean, send it to freedom
        if let Some(fisch) = self.fluss[12].extract_first() {
            self.fisch_cols.rm(&fisch.0);
            self.free_cols.add(fisch.0);
        }
    }

    fn mv_boot(&mut self) {
        self.boot_pos += 1;

        let index: usize = self.boot_pos.into();

        // If we caught some fisch, move them to the boot
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

        write!(f, "Round {}: {}\n", self.round, self.last_col)?;
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

    let mut runs: Vec<[Fische; 13]> = vec![];

    for blau in 1..12 {
        for gelb in 1..12 {
            for rosa in 1..12 {
                for orange in 1..12 {
                    let mut fisches = [Fische::default(); 13];

                    fisches[blau].add(Color::Blau);
                    fisches[gelb].add(Color::Gelb);
                    fisches[rosa].add(Color::Rosa);
                    fisches[orange].add(Color::Orange);

                    runs.push(fisches);
                }
            }
        }
    }

    runs.sort_by(|a, b| {
        let a_str = fisch_pattern(a);
        let b_str = fisch_pattern(b);

        a_str.cmp(&b_str)
    });

    runs.dedup_by(|a, b| {
        let a_str = fisch_pattern(a);
        let b_str = fisch_pattern(b);

        a_str.eq_ignore_ascii_case(&b_str)
    });

    let mut results = runs
        .iter()
        .map(|fluss| benchmark(*fluss))
        .collect::<Vec<BenchResult>>();

    results.sort_by(|a, b| {
        let a = a.b_f_delta();
        let b = b.b_f_delta();

        a.partial_cmp(&b).unwrap()
    });

    let results = results
        .iter()
        .map(|b| format!("{}\n", b))
        .collect::<String>();

    println!("{}", results);
}

fn fisch_pattern(fische: &[Fische; 13]) -> String {
    fische
        .iter()
        .map(|fl| fl.iter().filter(|fi| fi.is_some()).count())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

struct BenchResult {
    fluss: [Fische; 13],

    b_win: usize,
    b_winp: f32,
    b_rounds: usize,

    f_win: usize,
    f_winp: f32,
    f_rounds: usize,

    tie: usize,
    tiep: f32,
    t_rounds: usize,
}

impl BenchResult {
    pub fn b_f_delta(&self) -> f32 {
        (&self.b_winp - &self.f_winp).abs()
    }
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | ", fisch_pattern(&self.fluss))?;
        write!(
            f,
            "B: {:5} ({:6.2}%, R: {:2}) | ",
            self.b_win, self.b_winp, self.b_rounds
        )?;
        write!(
            f,
            "F: {:5} ({:6.2}%, R: {:2}) | ",
            self.f_win, self.f_winp, self.f_rounds
        )?;
        write!(
            f,
            "T: {:5} ({:6.2}%, R: {:2})",
            self.tie, self.tiep, self.t_rounds
        )
    }
}

fn benchmark(fluss: [Fische; 13]) -> BenchResult {
    let runs = 100_000;
    let winners = (0..runs)
        .step_by(1)
        .map(|_| {
            let mut fl = [Fische::default(); 13];
            fl.copy_from_slice(&fluss);

            let mut g = Game::new(fl);
            while g.tick() == Winner::Undecided {}
            (g.tick(), g.round)
        })
        .collect::<Vec<(Winner, u8)>>();

    let b_wins = winners.iter().filter(|w| w.0 == Winner::Boot);
    let f_wins = winners.iter().filter(|w| w.0 == Winner::Fisch);
    let ties = winners.iter().filter(|w| w.0 == Winner::Unentschieden);

    let b_win = b_wins.clone().count();
    let b_rounds: usize = b_wins
        .map(|w| w.1 as usize)
        .sum::<usize>()
        .div(b_win.max(1));
    let b_winp = (b_win as f32).div(runs as f32).mul(100f32);

    let f_win = f_wins.clone().count();
    let f_rounds: usize = f_wins
        .map(|w| w.1 as usize)
        .sum::<usize>()
        .div(f_win.max(1));
    let f_winp = (f_win as f32).div(runs as f32).mul(100f32);

    let tie = ties.clone().count();
    let t_rounds: usize = ties.map(|w| w.1 as usize).sum::<usize>().div(tie.max(1));
    let tiep = (tie as f32).div(runs as f32).mul(100f32);

    BenchResult {
        fluss,
        b_win,
        b_rounds,
        b_winp,
        f_win,
        f_rounds,
        f_winp,
        tie,
        t_rounds,
        tiep,
    }
}

fn manual_game() {
    let mut g = Game::default();
    println!("{}", g);

    while g.tick() == Winner::Undecided {
        println!("{}", g);
        std::thread::sleep(std::time::Duration::from_millis(250))
    }

    println!("Winner: {:?}", g.tick());
}
