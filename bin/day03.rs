use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Problem reading from stdin!");

    let lines = input.lines()
        .map(|s| s.to_string())
        .collect();

    let part1 = part_1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part_2(&lines);
    println!("Part 2: {}", part2);
}

fn part_1(lines: &Vec<String>) -> u32 {
    let claims = Claim::claim_collect(lines);

    let mut gt2_count = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            let mut contain_count = 0;
            let point = &Point { x, y };

            for claim in &claims {
                if claim.rect.contains(point) {
                    contain_count += 1;
                    if contain_count == 2 { continue }
                }
            }

            if contain_count >= 2 {
                gt2_count += 1;
            }
        }

    }

    gt2_count
}

fn part_2(lines: &Vec<String>) -> usize {
    let claims = Claim::claim_collect(lines);

    let mut lonely_vec: Vec<Claim> = Vec::new();

    for claim in &claims {
        let mut lonely = true;

        for claim2 in &claims {
            if claim == claim2 { continue }
            if claim.rect.overlaps(&claim2.rect) {
                lonely = false;
                break;
            }
        }

        if lonely {
            lonely_vec.push(claim.clone());
        }
    }

    assert!(lonely_vec.len() == 1);

    lonely_vec[0].id
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point{
    #[allow(dead_code)]
    pub fn contained_by(&self, rect: &Rectangle) -> bool {
        let rect_br = rect.end_corner();
        self.x > rect.origin.x &&
        self.x <= rect_br.x    &&
        self.y > rect.origin.y &&
        self.y <= rect_br.y
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Point,
}

impl Rectangle {
    pub fn end_corner(&self) -> Point {
        let x = self.origin.x + self.size.x;
        let y = self.origin.y + self.size.y;
        Point { x, y}
    }

    pub fn bounds(&self) -> Bounds {
        let ec = self.end_corner();

        let top    = ec.y;
        let bottom = self.origin.y;
        let left   = self.origin.x;
        let right  = ec.x;

        Bounds { top, bottom, left, right, }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let a = self.bounds();
        let b = other.bounds();

        if a == b { return true; }

        if a.top  <= b.bottom || b.top  <= a.bottom { return false; }
        if a.left >= b.right  || b.left >= a.right  { return false; }

        true
    }

    pub fn contains(&self, point: &Point) -> bool {
        let self_br = self.end_corner();

        // Needs work for both points and square inches
        point.x >= self.origin.x &&
        point.x < self_br.x      &&
        point.y >= self.origin.y &&
        point.y < self_br.y
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Bounds {
    pub top:    usize,
    pub bottom: usize,
    pub left:   usize,
    pub right:  usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Claim {
    pub id: usize,
    pub rect: Rectangle,
}

impl Claim {
    pub fn build_from_line(s: &String) -> Claim {
        let split: Vec<&str> = s.split_whitespace().collect();

        let id = split[0].trim_start_matches('#').parse::<usize>().unwrap_or(0);
        
        let split_comma: Vec<&str> = split[2].split(',').collect();
        let origin_xy: Vec<usize> = split_comma.iter()
            .map(|s| s.trim_end_matches(':').parse().unwrap_or(0))
            .collect();
        let origin = Point {
            x: origin_xy[0],
            y: origin_xy[1],
        };

        let size_xy: Vec<usize> = split[3].split('x')
            .map(|i| i.parse::<usize>().unwrap_or(0))
            .collect();
        let size = Point {
            x: size_xy[0],
            y: size_xy[1],
        };

        let rect = Rectangle {
            origin,
            size,
        };

        Claim {
            id,
            rect,
        }

    }

    pub fn claim_collect(lines: &Vec<String>) -> Vec<Claim> {
        lines.iter()
            .map(|s| Claim::build_from_line(s))
            .collect()
    }
}
