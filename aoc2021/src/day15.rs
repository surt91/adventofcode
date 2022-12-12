use crate::{utils::{Map, shortest_path::astar}, data_str};

pub fn run() -> (usize, usize) {
    let input = data_str!("day15a");
    let map: Map = input.parse().expect("invalid input");

    (
        map.lowest_risk(),
        map.expand().lowest_risk()
    )
}

impl Map {
    fn lowest_risk(&self) -> usize {
        let start = (0, 0);
        let end = (self.width-1, self.height-1);

        let shortest = astar(&self, start, end);

        shortest.iter().rev().skip(1).map(|p| self[*p] as usize).sum()
    }

    fn expand(&self) -> Map {
        let width = 5*self.width;
        let height = 5*self.height;

        fn wrap(v: u8) -> u8 {
            if v > 9 {
                v-9
            } else {
                v
            }
        }

        let mut values = vec![vec![0; width]; height];
        for j in 0..5usize {
            for i in 0..5usize {
                for y in 0..self.height {
                    for x in 0..self.width {
                        values[y+j*self.height][x+i*self.width] = wrap(self.values[y][x] + (i+j) as u8);
                    }
                }
            }
        }

        Map {
            width,
            height,
            values
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        ";

        let map: Map = input.parse().expect("invalid input");

        assert_eq!(map.lowest_risk(), 40);
        assert_eq!(map.expand().lowest_risk(), 315);
    }
}