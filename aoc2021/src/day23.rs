use std::{fs, str::FromStr, fmt, cmp, collections::HashSet};

use itertools::Itertools;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day23a.dat").expect("input file does not exist");
    let situation: Situation = input.parse().expect("invalid input");

    let input = fs::read_to_string("data/day23b.dat").expect("input file does not exist");
    let situation4: Situation = input.parse().expect("invalid input");

    (
        move_all(situation).unwrap(),
        move_all(situation4).unwrap(),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
            _ => unimplemented!(),
        }
    }

    fn pos(&self) -> isize {
        match self {
            Amphipod::Amber => 2,
            Amphipod::Bronze => 4,
            Amphipod::Copper => 6,
            Amphipod::Desert => 8,
            _ => unimplemented!()
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Situation {
    room_a: [Amphipod; 4],
    room_b: [Amphipod; 4],
    room_c: [Amphipod; 4],
    room_d: [Amphipod; 4],
    hallway: [Amphipod; 11],
    spent_energy: usize,
    four: bool
}

impl<'a> Situation {
    fn finished(&self) -> bool {
        let mut flag = true;
        for i in 0..(if self.four {4} else {2}) {
            flag &= self.room_a[i] == Amphipod::Amber;
            flag &= self.room_b[i] == Amphipod::Bronze;
            flag &= self.room_c[i] == Amphipod::Copper;
            flag &= self.room_d[i] == Amphipod::Desert;
        }

        flag
    }

    fn room_mut(&'a mut self, a: Amphipod) -> &'a mut [Amphipod; 4] {
        match a {
            Amphipod::Amber => &mut self.room_a,
            Amphipod::Bronze => &mut self.room_b,
            Amphipod::Copper => &mut self.room_c,
            Amphipod::Desert => &mut self.room_d,
            _ => unreachable!()
        }
    }

    fn room(&'a self, a: Amphipod) -> &'a [Amphipod; 4] {
        match a {
            Amphipod::Amber => &self.room_a,
            Amphipod::Bronze => &self.room_b,
            Amphipod::Copper => &self.room_c,
            Amphipod::Desert => &self.room_d,
            _ => unreachable!()
        }
    }
}

impl FromStr for Situation {
    type Err = AdventError;

    fn from_str(lines: &str) -> Result<Self, AdventError> {
        let mut members: Vec<Vec<Amphipod>> = lines.trim()
            .split('\n')
            .skip(2)
            .filter(|line| !line.trim().starts_with("#####"))
            .map(|line|
                line.trim().chars().filter(|&l| l != '#').map(|l| {
                    match l {
                        'A' => Ok(Amphipod::Amber),
                        'B' => Ok(Amphipod::Bronze),
                        'C' => Ok(Amphipod::Copper),
                        'D' => Ok(Amphipod::Desert),
                        c => Err(AdventError::UnexpectedElement { found: c.to_string(), expected: &["A", "B", "C", "D"] })
                    }
            })
            .collect()
        )
        .collect::<Result<_, _>>()?;

        let four = members.len() == 4;
        if !four {
            assert!(members.len() == 2);
            members.push(vec![Amphipod::None; 4]);
            members.push(vec![Amphipod::None; 4]);
        }

        Ok(
            Situation {
                room_a: [members[0][0], members[1][0], members[2][0], members[3][0]],
                room_b: [members[0][1], members[1][1], members[2][1], members[3][1]],
                room_c: [members[0][2], members[1][2], members[2][2], members[3][2]],
                room_d: [members[0][3], members[1][3], members[2][3], members[3][3]],
                hallway: [Amphipod::None; 11],
                spent_energy: 0,
                four
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
        if self.four {
            writeln!(f, "  #{}#{}#{}#{}#", self.room_a[2], self.room_b[2], self.room_c[2], self.room_d[2])?;
            writeln!(f, "  #{}#{}#{}#{}#", self.room_a[3], self.room_b[3], self.room_c[3], self.room_d[3])?;
        }
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
    let incr = if start > end { -1 } else { 1 };

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
        match s.hallway[i] {
            Amphipod::None => {},
            a => {
                e += (1+(i as isize - a.pos()).abs() as usize) * a.cost()
            }
        }
    }

    for species in [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert] {
        for (n, i) in s.room(species).iter().enumerate() {
            match *i {
                Amphipod::None => {},
                a if a != species => e += (n + (species.pos() - a.pos()).abs() as usize) * a.cost(),
                _ => {}
            }
        }
    }

    e
}

fn sort_room(s: &mut Situation, species: Amphipod) {
    let room_size = if s.four {4} else {2};
    let room = s.room_mut(species);
    let mut energy_delta = 0;

    if room.iter().any(|&a| a == Amphipod::None) {
        // now we sort the room:
        // going from bottom to top, we move down if we encounter `species`
        // until we encounter another species,
        // everything above we move up
        let pivot = room.iter()
            .take(room_size)
            .rev()
            .take_while(|&&x| x == species || x == Amphipod::None)
            .count();

        for i in 0..(room_size - pivot) {
            if room[i] == Amphipod::None {
                let mut j = i + 1;
                while j < (room_size - pivot) {
                    if room[j] != Amphipod::None {
                        energy_delta += (j-i) * room[j].cost();
                        room.swap(i, j);
                        break;
                    }
                    j += 1;
                }
            }
        }

        for i in (room_size - pivot)..room_size {
            if room[i] == species {
                let mut j = i + 1;
                while j < room_size {
                    if room[j] == Amphipod::None {
                        energy_delta += (j-i) * species.cost();
                        room.swap(i, j);
                        break;
                    }
                    j += 1;
                }
            }
        }

        s.spent_energy += energy_delta;
    }
}

fn move_in(s: &Situation, species: Amphipod, start: isize) -> Option<Situation> {
    let end = species.pos();

    if s.room(species)[0] == Amphipod::None
        && travel_check(s, start, end) {
        let mut n = s.clone();
        n.hallway[start as usize] = Amphipod::None;
        n.room_mut(species)[0] = species;
        n.spent_energy += (1+(end - start).abs() as usize) * species.cost();
        Some(n)
    } else {
        None
    }
}

fn possible_moves(s: &Situation) -> Vec<Situation> {
    let mut out = Vec::new();

    // there is no error, in moving all impure in the rooms to the top
    // and moving all pures to the bottom
    let mut s = s.clone();
    sort_room(&mut s, Amphipod::Amber);
    sort_room(&mut s, Amphipod::Bronze);
    sort_room(&mut s, Amphipod::Copper);
    sort_room(&mut s, Amphipod::Desert);

    // hallway
    for i in [0, 1, 3, 5, 7, 9, 10] {
        let start = i as isize;
        match s.hallway[i] {
            Amphipod::None => (),
            a => {
                move_in(&s, a, start).map(|n| {out.push(n); Some(())});
            }
        }
    }

    // rooms
    for species in [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert] {
        let room = s.room(species);
        match room[0] {
            Amphipod::None => {}
            a => {
                if a != species || (
                    room[1] != species
                    || (s.four && (room[2] != species || room[3] != species))
                )
                {
                    for mut i in travel_out(&s, species.pos(), a) {
                        i.room_mut(species)[0] = Amphipod::None;
                        out.push(i);
                    }
                }
            }
        }
    }

    out
}

// brute force branch and bound
fn move_all(situation: Situation) -> Option<usize> {
    let mut best = if situation.four {60000} else {20000};
    let mut to_check = vec![situation];
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

        to_check.extend(possible_moves(&s));

        encountered.insert(s);
    }

    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut s = Situation{
            room_a: [Amphipod::Amber, Amphipod::None, Amphipod::None, Amphipod::None],
            room_b: [Amphipod::None, Amphipod::Desert, Amphipod::None, Amphipod::None],
            room_c: [Amphipod::None; 4],
            room_d: [Amphipod::None; 4],
            hallway: [Amphipod::None; 11],
            spent_energy: 0,
            four: false,
        };
        sort_room(&mut s, Amphipod::Bronze);
        assert_eq!(s.room_b, [Amphipod::Desert, Amphipod::None, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 1000);
        sort_room(&mut s, Amphipod::Bronze);
        assert_eq!(s.room_b, [Amphipod::Desert, Amphipod::None, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 1000);

        sort_room(&mut s, Amphipod::Amber);
        assert_eq!(s.room_a, [Amphipod::None, Amphipod::Amber, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 1001);
        sort_room(&mut s, Amphipod::Amber);
        assert_eq!(s.room_a, [Amphipod::None, Amphipod::Amber, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 1001);
    }

    #[test]
    fn sort4() {
        let mut s = Situation{
            room_a: [Amphipod::Amber, Amphipod::None, Amphipod::None, Amphipod::None],
            room_b: [Amphipod::None, Amphipod::None, Amphipod::None, Amphipod::Desert],
            room_c: [Amphipod::None, Amphipod::None, Amphipod::Copper, Amphipod::Desert],
            room_d: [Amphipod::None, Amphipod::None, Amphipod::Copper, Amphipod::Desert],
            hallway: [Amphipod::None; 11],
            spent_energy: 0,
            four: true
        };
        sort_room(&mut s, Amphipod::Bronze);
        assert_eq!(s.room_b, [Amphipod::Desert, Amphipod::None, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 3000);
        sort_room(&mut s, Amphipod::Bronze);
        assert_eq!(s.room_b, [Amphipod::Desert, Amphipod::None, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 3000);

        sort_room(&mut s, Amphipod::Amber);
        assert_eq!(s.room_a, [Amphipod::None, Amphipod::None, Amphipod::None, Amphipod::Amber]);
        assert_eq!(s.spent_energy, 3003);
        sort_room(&mut s, Amphipod::Amber);
        assert_eq!(s.room_a, [Amphipod::None, Amphipod::None, Amphipod::None, Amphipod::Amber]);
        assert_eq!(s.spent_energy, 3003);

        sort_room(&mut s, Amphipod::Copper);
        assert_eq!(s.room_c, [Amphipod::Copper, Amphipod::Desert, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 5203);
        sort_room(&mut s, Amphipod::Desert);
        assert_eq!(s.room_d, [Amphipod::Copper, Amphipod::None, Amphipod::None, Amphipod::Desert]);
        assert_eq!(s.spent_energy, 5403);

        s.room_b = [Amphipod::Bronze, Amphipod::Desert, Amphipod::None, Amphipod::None];
        s.spent_energy = 0;
        sort_room(&mut s, Amphipod::Bronze);
        assert_eq!(s.room_b, [Amphipod::Bronze, Amphipod::Desert, Amphipod::None, Amphipod::None]);
        assert_eq!(s.spent_energy, 0);
    }

    #[test]
    // #[ignore]
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

        let input = r"
            #############
            #...........#
            ###B#C#B#D###
              #D#C#B#A#
              #D#B#A#C#
              #A#D#C#A#
              #########
        ";

        let situation: Situation = input.parse().expect("invalid input");
        assert_eq!(move_all(situation).unwrap(), 44169);
    }
}