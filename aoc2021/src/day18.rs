use std::{str::FromStr, fs, ops::{Add, AddAssign}, iter::Sum, fmt::Display};


use crate::utils::{AdventError};

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day18a.dat").expect("invalid input");
    let g: Vec<SnailfishNumber> = input.trim().split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("invalid input");

    (
        g.into_iter().sum::<SnailfishNumber>().magnitude(),
        0,
    )
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Open,
    Close,
    Number(u8)
}

#[derive(Debug, PartialEq, Eq)]
struct SnailfishNumber {
    line: Vec<Element>
}

impl SnailfishNumber {
    fn new() -> Self {
        SnailfishNumber {
            line: Vec::new()
        }
    }

    fn split(&mut self) -> bool {
        let mut idx = 0usize;

        // println!("before s: {:}", self);

        while idx < self.line.len() {
            match self.line[idx] {
                Element::Number(x) if x > 9 => {
                    let left = (x as f64 / 2.).floor() as u8;
                    let right = (x as f64 / 2.).ceil() as u8;
                    self.line[idx] = Element::Close;
                    self.line.insert(idx, Element::Number(right));
                    self.line.insert(idx, Element::Number(left));
                    self.line.insert(idx, Element::Open);

                    // println!("split at {}: {}", x, self);

                    return true
                }
                _ => {}
            }
            idx += 1;
        }
        false
    }

    fn explode(&mut self) -> bool {
        let mut idx = 0usize;
        let mut depth = 0;
        // println!("before: {:}", self);

        while idx < self.line.len() {
            match self.line[idx] {
                Element::Open => depth += 1,
                Element::Close => depth -= 1,
                Element::Number(x) => {
                    if depth >= 5 {
                        let left_idx = self.find_left(idx);
                        let right_idx = self.find_right(idx+1);

                        // println!("add to left: {:?}", left_idx);
                        if let Some(li) = left_idx {
                            if let Element::Number(lv) = self.line[li] {
                                self.line[li] = Element::Number(lv + x)
                            } else {
                                panic!("panic!")
                            }
                        }

                        // println!("add to right: {:?}", right_idx);
                        if let Some(ri) = right_idx {
                            if let Element::Number(rv) = self.line[idx+1] {
                                // println!("rv: {:?}", rv);
                                if let Element::Number(y) = self.line[ri] {
                                    // println!("y: {:?}", y);
                                    self.line[ri] = Element::Number(rv + y)
                                } else {
                                    panic!("panic!")
                                }
                            }
                        }

                        // remove [, x, y, ], insert 0
                        self.line[idx-1] = Element::Number(0);
                        self.line.remove(idx);
                        self.line.remove(idx);
                        self.line.remove(idx);

                        // println!("exploded: {:}", self);
                        return true
                    }
                }
            }
            idx += 1
        }

        false
    }

    fn find_left(&self, mut idx: usize) -> Option<usize> {
        while idx > 0 {
            idx -= 1;
            if let Element::Number(_) = self.line[idx] {
                return Some(idx)
            }

        }
        None
    }

    fn find_right(&self, mut idx: usize) -> Option<usize> {
        while idx < self.line.len() - 1 {
            idx += 1;
            if let Element::Number(_) = self.line[idx] {
                return Some(idx)
            }
        }
        None
    }

    fn reduce(&mut self) {
        // println!("start: {:}", self);
        loop {
            if self.explode() {
                // println!("exploded: {:}", self);
                continue;
            }
            if self.split() {
                // println!("split: {:}", self);
                continue;
            }
            break
        }
    }

    fn magnitude(&self) -> usize {

        println!("look at {}", self);

        if let Element::Number(x) = self.line[0] {
            println!("found {}", x);
            return x as usize
        }

        let left_start = 1;
        let left_end;
        let right_start;
        let right_end;

        let mut num_open = 0;
        let mut idx = left_start;


        loop {
            match self.line[idx] {
                Element::Open => {
                    num_open += 1;
                },
                Element::Close => {
                    num_open -= 1;
                },
                _ => {}
            }
            if num_open == 0 {
                left_end = idx;
                right_start = idx + 1;
                break
            }
            idx += 1
        }

        loop {
            // println!("rest {:?}", self.line[idx..].to_vec());
            idx += 1;
            match self.line[idx] {
                Element::Open => {
                    num_open += 1;
                },
                Element::Close => {
                    num_open -= 1;
                },
                _ => {}
            }
            if num_open == 0 {
                right_end = idx;
                break
            }
        }

        let left = SnailfishNumber {
            line: self.line[left_start..=left_end].to_vec()
        };

        let right = SnailfishNumber {
            line: self.line[right_start..=right_end].to_vec()
        };

        println!("l {} ;  r {}", left, right);

        3*left.magnitude() + 2*right.magnitude()
    }
}

impl AddAssign for SnailfishNumber {
    fn add_assign(&mut self, rhs: Self) {
        // println!("before add: {}", self);
        let not_new = !self.line.is_empty();
        self.line.extend(rhs.line);

        if not_new {
            self.line.insert(0, Element::Open);
            self.line.push(Element::Close);
        }
        // println!("after add: {}", self);
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut out = SnailfishNumber::new();
        for i in iter {
            out += i;
            out.reduce();
        }
        out
    }
}

impl FromStr for SnailfishNumber {
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let line = input.trim().chars().map(|c| {
            match c {
                '[' => Some(Element::Open),
                ']' => Some(Element::Close),
                ',' | ' ' => None,
                x => Some(Element::Number(x.to_string().parse().unwrap()))
            }
        }).flatten()
        .collect();

        Ok(SnailfishNumber {
            line
        })
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.line.iter() {
            match c {
                Element::Open => write!(f, "[")?,
                Element::Close => write!(f, "]")?,
                Element::Number(x) => write!(f, "{},", x)?,
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut sn: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        let expected: SnailfishNumber = "[[[[0,9],2],3],4]".parse().unwrap();
        sn.explode();
        assert_eq!(sn, expected);
        let mut sn: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[7,[6,[5,[7,0]]]]".parse().unwrap();
        sn.explode();
        assert_eq!(sn, expected);
        let mut sn: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        let expected: SnailfishNumber = "[[6,[5,[7,0]]],3]".parse().unwrap();
        sn.explode();
        assert_eq!(sn, expected);
        let mut sn: SnailfishNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        sn.explode();
        assert_eq!(sn, expected);
        let mut sn: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();
        sn.explode();
        assert_eq!(sn, expected);

        let mut sn: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let sn2: SnailfishNumber = "[1,1]".parse().unwrap();
        let expected: SnailfishNumber = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        sn += sn2;
        assert_eq!(sn, expected);
        sn.reduce();
        let expected: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(sn, expected);

        let sn: SnailfishNumber = [
            "[1,1]".parse::<SnailfishNumber>().unwrap(),
            "[2,2]".parse::<SnailfishNumber>().unwrap(),
            "[3,3]".parse::<SnailfishNumber>().unwrap(),
            "[4,4]".parse::<SnailfishNumber>().unwrap(),
        ].into_iter().sum();
        let expected: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        println!("have: {}", sn);
        println!("should: {}", expected);
        assert_eq!(sn, expected);
        let sn: SnailfishNumber = [
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
            "[5,5]".parse().unwrap(),
        ].into_iter().sum();
        let expected: SnailfishNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        assert_eq!(sn, expected);
        let sn: SnailfishNumber = [
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
            "[5,5]".parse().unwrap(),
            "[6,6]".parse().unwrap(),
        ].into_iter().sum();
        let expected: SnailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(sn, expected);

        let sn: SnailfishNumber = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 143);
        let sn: SnailfishNumber = " [[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 1384);
        let sn: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 445);
        let sn: SnailfishNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 791);
        let sn: SnailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 1137);
        let sn: SnailfishNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
        assert_eq!(sn.magnitude(), 3488);

        let input = r"
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ";
        let s: Vec<SnailfishNumber> = input.trim().split('\n')
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .expect("invalid input");
        let expected: SnailfishNumber = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
        let sum = s.into_iter().sum::<SnailfishNumber>();

        println!("s {:?}", sum);
        println!("sum {}", sum);
        println!("expected {}", expected);

        assert_eq!(sum, expected);
        assert_eq!(sum.magnitude(), 4140);
    }
}