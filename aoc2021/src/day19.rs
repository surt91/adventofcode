use std::{collections::HashSet, fs, str::FromStr, ops::{Sub, Add}};

use itertools::{Itertools, iproduct};
use scan_fmt::scan_fmt;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day19a.dat").expect("input file does not exist");
    let scanners = parse(&input).expect("invalid input");
    let scanners = locate_scanners(&scanners);

    (
        count_beacons(&scanners),
        scanner_distance(&scanners),
    )
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

impl Point {
    fn all_orientations(&self) -> impl Iterator<Item=Point> + '_ {
        iproduct!(&[-1, 1], &[-1, 1], &[-1, 1]).flat_map(|(i, j, k)| {
            [i*self.x, j*self.y, k*self.z].iter()
                .cloned()
                .permutations(3)
                .map(|v| {
                    Point{
                        x: v[0],
                        y: v[1],
                        z: v[2],
                    }
                }).collect_vec()
        })
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    fn manhattan(&self, other: &Point) -> usize {
        (
            (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
        )
        as usize
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl FromStr for Point {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let (x, y, z) = scan_fmt!(
            line,
            "{},{},{}",
            isize, isize, isize
        )?;

        Ok(
            Point {
                x,
                y,
                z
            }
        )
    }
}

#[derive(Clone)]
struct Scanner {
    position: Point,
    beacons: Vec<Point>
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            position: Point {x: 0, y: 0, z: 0},
            beacons: Vec::new()
        }
    }

    fn all_orientations(&self) -> Vec<Vec<Point>> {
        let mut out: Vec<Vec<Point>> = vec![Vec::new(); 48];

        for p in &self.beacons {
            for (n, o) in p.all_orientations().enumerate() {
                out[n].push(o)
            }
        }

        out
    }

    // I am sure I could avoid some of the clones, but it works (but not very fast)
    fn overlap(&self, other: &Scanner) -> Option<Scanner> {
        for o in other.all_orientations() {
            for j in &o {
                for i in &self.beacons {
                    let delta = i - j;
                    // shift by the delta of all pairs
                    // and look if there are >= 12 overlapping
                    let matches = self.beacons.iter().cloned().flat_map(|b| {
                        let shifted = b - delta.clone();
                        o.iter().cloned().map(move |a| {
                            shifted.clone() - a
                        })
                    }).filter(|d| d.is_zero())
                    .count();

                    if matches >= 12 {
                        // then save the shift and maybe return a
                        // shifted and rotated replacement for other
                        let out = Scanner {
                            position: &other.position - &delta,
                            beacons: o.iter().map(|p| {
                                p + &delta
                            }).collect()
                        };
                        return Some(out)
                    }
                }
            }
        }

        None
    }
}

fn parse(input: &str) -> Result<Vec<Scanner>, AdventError> {
    let blocks = input.split("\n\n");
    let mut scanners = Vec::new();

    for block in blocks {
        let mut scanner = Scanner::new();
        for line in block.trim().split('\n').skip(1) {
            let p: Point = line.trim().parse()?;
            scanner.beacons.push(p);
        }
        scanners.push(scanner);
    }

    Ok(scanners)
}

fn locate_scanners(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut out: Vec<Scanner> = Vec::new();
    let mut unlocated: HashSet<usize> = (1..scanners.len()).collect();

    let mut idx = 0;
    let mut reference = scanners[0].clone();
    out.push(reference.clone());
    while !unlocated.is_empty() {
        let mut located: HashSet<usize> = HashSet::new();
        for &i in &unlocated {
            if let Some(x) = reference.overlap(&scanners[i]) {
                located.insert(i);
                out.push(x);
            }
        }
        unlocated.retain(|x| !located.contains(x));
        idx += 1;
        reference = out[idx].clone()
    }

    out
}

fn count_beacons(scanners: &[Scanner]) -> usize {
    let mut beacons: HashSet<Point> = HashSet::new();
    for scanner in scanners {
        for p in &scanner.beacons {
            beacons.insert(p.clone());
        }
    }

    beacons.len()
}

fn scanner_distance(scanners: &[Scanner]) -> usize {
    scanners.iter()
        .combinations(2)
        .map(|pair|
            pair[0].position.manhattan(&pair[1].position)
        )
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        ";

        let scanners = parse(input).expect("invalid input");
        let scanners = locate_scanners(&scanners);

        println!("{:?}", scanners.iter().map(|s| s.position.clone()).collect_vec());

        assert_eq!(count_beacons(&scanners), 79);
        assert_eq!(scanner_distance(&scanners), 3621);
    }
}
