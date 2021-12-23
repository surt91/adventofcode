use std::{fs, str::FromStr, fmt, cmp, collections::HashSet};

use itertools::Itertools;

use crate::utils::AdventError;

pub fn run() -> (usize, isize) {
    let input = fs::read_to_string("data/day23a.dat").expect("input file does not exist");
    let situation: Situation = input.parse().expect("invalid input");

    (
        move_all(situation).unwrap(),
        0,
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    None,
}

impl Amphipod {
    fn cost(&self) -> usize {
        match *self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
            Amphipod::None => 0,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Situation {
    room_a: [Amphipod; 2],
    room_b: [Amphipod; 2],
    room_c: [Amphipod; 2],
    room_d: [Amphipod; 2],
    hallway: [Amphipod; 11],
    spent_energy: usize,
}

impl Situation {
    fn finished(&self) -> bool {
        self.room_a[0] == Amphipod::Amber && self.room_a[1] == Amphipod::Amber
            && self.room_b[0] == Amphipod::Bronze && self.room_b[1] == Amphipod::Bronze
            && self.room_c[0] == Amphipod::Copper && self.room_c[1] == Amphipod::Copper
            && self.room_d[0] == Amphipod::Desert && self.room_d[1] == Amphipod::Desert
    }
}

impl FromStr for Situation {
    type Err = AdventError;

    fn from_str(lines: &str) -> Result<Self, AdventError> {
        let members: Vec<Vec<Amphipod>> = lines.trim().split('\n').skip(2).take(2).map(|line|
            line.trim().chars().filter(|&l| l != '#').map(|l| {
                match l {
                    'A' => Ok(Amphipod::Amber),
                    'B' => Ok(Amphipod::Bronze),
                    'C' => Ok(Amphipod::Copper),
                    'D' => Ok(Amphipod::Desert),
                    c => Err(AdventError::UnexpectedElement { found: c.to_string(), expected: &["A", "B", "C", "D"] })
                }
            }).collect()

        )
        .collect::<Result<_, _>>()?;

        Ok(
            Situation {
                room_a: [members[0][0], members[1][0]],
                room_b: [members[0][1], members[1][1]],
                room_c: [members[0][2], members[1][2]],
                room_d: [members[0][3], members[1][3]],
                hallway: [Amphipod::None; 11],
                spent_energy: 0,
            }
        )
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match *self {
            Amphipod::Amber => "A",
            Amphipod::Bronze => "B",
            Amphipod::Copper => "C",
            Amphipod::Desert => "D",
            Amphipod::None => ".",
        };
        write!(f, "{}", letter)
    }
}

impl fmt::Display for Situation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}#", self.hallway.iter().join(""))?;
        writeln!(f, "###{}#{}#{}#{}###", self.room_a[0], self.room_b[0], self.room_c[0], self.room_d[0])?;
        writeln!(f, "  #{}#{}#{}#{}#", self.room_a[1], self.room_b[1], self.room_c[1], self.room_d[1])?;
        writeln!(f, "  #########")?;
        writeln!(f, "E = {}", self.spent_energy)
    }
}

fn travel_out(s: &Situation, start: isize, a: Amphipod) -> Vec<Situation> {
    assert!([2, 4, 6, 8].contains(&start));
    let allowed = [0, 1, 3, 5, 7, 9, 10];
    let mut out = Vec::new();

    for end in allowed {
        if travel_check(s, start, end) {
            let mut n = s.clone();
            n.hallway[end as usize] = a;
            n.spent_energy += (1+(end-start).abs() as usize) * a.cost();
            out.push(n);
        }
    }

    out
}

fn travel_check(s: &Situation, start: isize, end: isize) -> bool {
    let incr = if start > end { -1 } else {1};

    let mut i = start;
    while i != end {
        i += incr;
        if s.hallway[i as usize] != Amphipod::None {
            return false
        }
    }

    true
}

fn lower_energy_bound(s: &Situation) -> usize {
    let mut e = 0;
    for i in [0, 1, 3, 5, 7, 9, 10] {
        if let a @ (Amphipod::Amber|Amphipod::Bronze|Amphipod::Copper|Amphipod::Desert) = s.hallway[i] {
            e += (1+(i as isize - 8).abs() as usize) * a.cost()
        }
    }
    for (n, i) in s.room_a.iter().enumerate() {
        match i {
            a @ Amphipod::Desert => e += (n+8) * a.cost(),
            a @ Amphipod::Copper => e += (n+6) * a.cost(),
            a @ Amphipod::Bronze => e += (n+4) * a.cost(),
            _ => {}
        }
    }
    for (n, i) in s.room_b.iter().enumerate() {
        match i {
            a @ Amphipod::Desert => e += (n+6) * a.cost(),
            a @ Amphipod::Copper => e += (n+4) * a.cost(),
            a @ Amphipod::Amber => e += (n+4) * a.cost(),
            _ => {}
        }
    }
    for (n, i) in s.room_c.iter().enumerate() {
        match i {
            a @ Amphipod::Desert => e += (n+4) * a.cost(),
            a @ Amphipod::Bronze => e += (n+4) * a.cost(),
            a @ Amphipod::Amber => e += (n+6) * a.cost(),
            _ => {}
        }
    }
    for (n, i) in s.room_d.iter().enumerate() {
        match i {
            a @ Amphipod::Copper => e += (n+4) * a.cost(),
            a @ Amphipod::Bronze => e += (n+6) * a.cost(),
            a @ Amphipod::Amber => e += (n+8) * a.cost(),
            _ => {}
        }
    }

    e
}

// I guess I could dry this tremendously, but there needs to be at least one really ugly solution for this years AoC
fn possible_moves(s: &Situation) -> Vec<Situation> {
    let mut out = Vec::new();

    // hallway
    for i in [0, 1, 3, 5, 7, 9, 10] {
        let start = i as isize;
        match s.hallway[i] {
            Amphipod::None => {}
            a @ Amphipod::Desert => {
                let end = 8;
                if (s.room_d[1] == Amphipod::None || s.room_d[1] == Amphipod::Desert)
                    && s.room_d[0] == Amphipod::None
                    && travel_check(s, start, end) {
                    let mut n = s.clone();
                    n.hallway[i] = Amphipod::None;
                    n.room_d[0] = Amphipod::Desert;
                    n.spent_energy += (1+(end - start).abs() as usize) * a.cost();
                    out.push(n);
                }
            }
            a @ Amphipod::Copper => {
                let end = 6;
                if (s.room_c[1] == Amphipod::None || s.room_c[1] == Amphipod::Copper)
                    && s.room_c[0] == Amphipod::None
                    && travel_check(s, start, end) {
                    let mut n = s.clone();
                    n.hallway[i] = Amphipod::None;
                    n.room_c[0] = Amphipod::Copper;
                    n.spent_energy += (1+(end - start).abs() as usize) * a.cost();
                    out.push(n);
                }
            }
            a @ Amphipod::Bronze => {
                let end = 4;
                if (s.room_b[1] == Amphipod::None || s.room_b[1] == Amphipod::Bronze)
                    && s.room_b[0] == Amphipod::None
                    && travel_check(s, start, end) {
                    let mut n = s.clone();
                    n.hallway[i] = Amphipod::None;
                    n.room_b[0] = Amphipod::Bronze;
                    n.spent_energy += (1+(end - start).abs() as usize) * a.cost();
                    out.push(n);
                }
            }
            a @ Amphipod::Amber => {
                let end = 2;
                if (s.room_a[1] == Amphipod::None || s.room_a[1] == Amphipod::Amber)
                    && s.room_a[0] == Amphipod::None
                    && travel_check(s, start, end)
                {
                    let mut n = s.clone();
                    n.hallway[i] = Amphipod::None;
                    n.room_a[0] = Amphipod::Amber;
                    n.spent_energy += (1+(end - start).abs() as usize) * a.cost();
                    out.push(n);
                }
            }
        }
    }

    // rooms
    match s.room_d[0] {
        a @ Amphipod::Desert => {
            if s.room_d[1] == Amphipod::None {
                let mut n = s.clone();
                n.room_d[1] = a;
                n.room_d[0] = Amphipod::None;
                n.spent_energy += a.cost();
                out.push(n)
            }
            if s.room_d[1] != Amphipod::Desert {
                for mut i in travel_out(s, 8, a) {
                    i.room_d[0] = Amphipod::None;
                    out.push(i);
                }
            }
        }
        Amphipod::None => {
            if s.room_d[1] != Amphipod::None && s.room_d[1] != Amphipod::Desert {
                let mut n = s.clone();
                n.room_d[0] = n.room_d[1];
                n.room_d[1] = Amphipod::None;
                n.spent_energy += n.room_d[0].cost();
                out.push(n)
            }
        }
        a => {
            for mut i in travel_out(s, 8, a) {
                i.room_d[0] = Amphipod::None;
                out.push(i);
            }
        }
    }
    match s.room_c[0] {
        a @ Amphipod::Copper => {
            if s.room_c[1] == Amphipod::None {
                let mut n = s.clone();
                n.room_c[1] = a;
                n.room_c[0] = Amphipod::None;
                n.spent_energy += a.cost();
                out.push(n)
            }
            if s.room_c[1] != Amphipod::Copper {
                for mut i in travel_out(s, 6, a) {
                    i.room_c[0] = Amphipod::None;
                    out.push(i);
                }
            }
        }
        Amphipod::None => {
            if s.room_b[1] != Amphipod::None && s.room_c[1] != Amphipod::Copper {
                let mut n = s.clone();
                n.room_c[0] = n.room_c[1];
                n.room_c[1] = Amphipod::None;
                n.spent_energy += n.room_c[0].cost();
                out.push(n)
            }
        }
        a => {
            for mut i in travel_out(s, 6, a) {
                i.room_c[0] = Amphipod::None;
                out.push(i);
            }
        }
    }
    match s.room_b[0] {
        a @ Amphipod::Bronze => {
            if s.room_b[1] == Amphipod::None {
                let mut n = s.clone();
                n.room_b[1] = a;
                n.room_b[0] = Amphipod::None;
                n.spent_energy += a.cost();
                out.push(n)
            }
            if s.room_b[1] != Amphipod::Bronze {
                for mut i in travel_out(s, 4, a) {
                    i.room_b[0] = Amphipod::None;
                    out.push(i);
                }
            }
        }
        Amphipod::None => {
            if s.room_b[1] != Amphipod::None && s.room_b[1] != Amphipod::Bronze {
                let mut n = s.clone();
                n.room_b[0] = n.room_b[1];
                n.room_b[1] = Amphipod::None;
                n.spent_energy += n.room_b[0].cost();
                out.push(n)
            }
        }
        a => {
            for mut i in travel_out(s, 4, a) {
                i.room_b[0] = Amphipod::None;
                out.push(i);
            }
        }
    }
    match s.room_a[0] {
        a @ Amphipod::Amber => {
            if s.room_a[1] == Amphipod::None {
                let mut n = s.clone();
                n.room_a[1] = a;
                n.room_a[0] = Amphipod::None;
                n.spent_energy += a.cost();
                out.push(n)
            }
            if s.room_a[1] != Amphipod::Amber {
                for mut i in travel_out(s, 2, a) {
                    i.room_a[0] = Amphipod::None;
                    out.push(i);
                }
            }
        }
        Amphipod::None => {
            if s.room_a[1] != Amphipod::None && s.room_a[1] != Amphipod::Amber {
                let mut n = s.clone();
                n.room_a[0] = n.room_a[1];
                n.room_a[1] = Amphipod::None;
                n.spent_energy += n.room_a[0].cost();
                out.push(n)
            }
        }
        a => {
            for mut i in travel_out(s, 2, a) {
                i.room_a[0] = Amphipod::None;
                out.push(i);
            }
        }
    }

    out
}

// brute force branch and bound
fn move_all(situation: Situation) -> Option<usize> {
    let mut to_check = vec![situation];
    let mut best = 20000;
    let mut encountered = HashSet::new();

    while let Some(s) = to_check.pop() {
        if s.finished() {
            best = cmp::min(best, s.spent_energy);
            continue;
        }

        if encountered.contains(&s) {
            continue;
        }

        if s.spent_energy + lower_energy_bound(&s) > best {
            continue;
        }
        if s.spent_energy > best {
            continue;
        }

        for p in possible_moves(&s) {
            to_check.push(p)
        }

        encountered.insert(s);
    }

    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            #############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########
        ";

        let situation: Situation = input.parse().expect("invalid input");
        assert_eq!(move_all(situation).unwrap(), 12521);
    }
}