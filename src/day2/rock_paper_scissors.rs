use std::fs::File;
use std::io::{BufRead, BufReader};

enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl RPS {
    fn from_letter(letter: &str) -> RPS {
        return if letter == "A" {
            RPS::Rock
        } else if letter == "B" {
            RPS::Paper
        } else {
            RPS::Scissor
        }
    }

    fn beats_move(&self, opponent_move: RPS) {}
}

fn from_string(letter: &str) -> &dyn RPSShape {
    return if letter == "X" {
        &Rock {}
    } else if letter == "Y" {
        &Paper {}
    } else {
        &Scissor {}
    }
}

trait RPSShape {
    fn point_value(&self) -> u32;
    fn beats(&self, other_shape: &RPS) -> u32;
}

struct Rock {}

impl RPSShape for Rock {
    fn point_value(&self) -> u32 {
        1
    }

    fn beats(&self, other_shape: &RPS) -> u32 {
        match other_shape {
            RPS::Rock => 3 + self.point_value(),
            RPS::Paper => 0 + self.point_value(),
            RPS::Scissor => 6 + self.point_value(),
        }
    }
}

struct Paper {}

impl RPSShape for Paper {
    fn point_value(&self) -> u32 {
        2
    }

    fn beats(&self, other_shape: &RPS) -> u32 {
        match other_shape {
            RPS::Rock => 6 + self.point_value(),
            RPS::Paper => 3 + self.point_value(),
            RPS::Scissor => 0 + self.point_value(),
        }
    }
}

struct Scissor {}

impl RPSShape for Scissor {
    fn point_value(&self) -> u32 {
        3
    }

    fn beats(&self, other_shape: &RPS) -> u32 {
        match other_shape {
            RPS::Rock => 0 + self.point_value(),
            RPS::Paper => 6 + self.point_value(),
            RPS::Scissor => 3 + self.point_value(),
        }
    }
}

fn main() {
    let mut buf = BufReader::new(File::open("src/day2/test1.txt").expect("file not found"));
    let mut total = 0;
    loop {
        let mut line = String::new();
        let result = buf.read_line(&mut line);
        if result.expect("can't read") > 0 {
            let moves = line.trim().split(" ").collect::<Vec<&str>>();
            let opponent = RPS::from_letter(moves[0]);
            let me: &dyn RPSShape = from_string(moves[1]);

            let current = me.beats(&opponent);
            println!("current {current}");
            total += current
        } else {
            println!("total: {total}");
            return;
        }
    }
}