use std::io::{self, Read};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!("Could not read from input {}", e);
    };
    let first_wire = buffer
        .trim_end()
        .split('\n')
        .nth(0)
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    let second_wire = buffer
        .trim_end()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    let first_wire_path = get_all_points(&first_wire);
    let second_wire_path = get_all_points(&second_wire);
    let intersections = find_intersections_steps(&first_wire_path, &second_wire_path);
    println!(
        "answer is: {}",
        find_minimum_steps_from_origin(&intersections)
    );
}

fn find_minimum_steps_from_origin(vec: &[i64]) -> i64 {
    *vec.iter().min_by(|&val1, &val2| val1.cmp(val2)).unwrap()
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn find_intersections_steps(v1: &[Point], v2: &[Point]) -> Vec<i64> {
    let origin = Point::new();
    let mut vec: Vec<i64> = Vec::new();
    let mut steps_1 = 0;
    for (p0, p1) in v1.iter().zip(v1[1..].iter()) {
        steps_1 += manhattan(p1, p0);
        let mut steps_2 = 0;
        for (p2, p3) in v2.iter().zip(v2[1..].iter()) {
            steps_2 += manhattan(p3, p2);
            if let Some(intersection) = intersection(p0, p1, p2, p3) {
                if intersection != origin {
                    let steps = steps_1 + steps_2
                        - manhattan(&intersection, &p1)
                        - manhattan(&intersection, &p3);
                    vec.push(steps);
                }
            }
        }
    }
    vec
}

fn intersection(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> Option<Point> {
    // Formula from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
    let x1 = p0.x as f64;
    let y1 = p0.y as f64;

    let x2 = p1.x as f64;
    let y2 = p1.y as f64;

    let x3 = p2.x as f64;
    let y3 = p2.y as f64;

    let x4 = p3.x as f64;
    let y4 = p3.y as f64;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -1.0 * (((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom);

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        Some(Point {
            x: x as i64,
            y: y as i64,
        })
    } else {
        None
    }
}

fn get_all_points(v: &[&str]) -> Vec<Point> {
    let mut x = 0;
    let mut y = 0;
    let mut vec: Vec<Point> = Vec::new();
    vec.push(Point::new());
    for val in v {
        let first_char = val.chars().nth(0).unwrap();
        let steps = val.trim_matches(first_char).parse::<i64>().unwrap();
        match first_char {
            'R' => {
                vec.push(Point { x: x + steps, y });
                x += steps;
            }
            'L' => {
                vec.push(Point { x: x - steps, y });
                x -= steps;
            }
            'U' => {
                vec.push(Point { x, y: y + steps });
                y += steps;
            }
            'D' => {
                vec.push(Point { x, y: y - steps });
                y -= steps;
            }
            _ => panic!("This is not a valid direction"),
        }
    }
    vec
}
