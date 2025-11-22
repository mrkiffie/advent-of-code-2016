const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Aabb {
    min: Vec2,
    max: Vec2,
    direction: Direction,
}

impl Aabb {
    fn new(from: Vec2, to: Vec2) -> Self {
        match (from.x.cmp(&to.x), from.y.cmp(&to.y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Self {
                min: from,
                max: to,
                direction: Direction::E,
            },
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less | std::cmp::Ordering::Equal) => {
                Self {
                    min: from,
                    max: to,
                    direction: Direction::N,
                }
            }
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Self {
                min: to,
                max: from,
                direction: Direction::S,
            },
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Self {
                min: to,
                max: from,
                direction: Direction::W,
            },
            _ => {
                unimplemented!();
            }
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        let x_overlaps = self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y_overlaps = self.min.y <= other.max.y && self.max.y >= other.min.y;
        x_overlaps && y_overlaps
    }
}

#[derive(Debug)]
struct Visited {
    visited: Vec<Vec2>,
}

impl Visited {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            visited: Vec::with_capacity(capacity),
        }
    }

    fn check_intersection(&mut self, next: Vec2) -> Option<Vec2> {
        if self.visited.len() < 2 {
            self.visited.push(next);
            return None;
        }
        let previous = self.visited.last().copied().expect("length checked above");

        let delta = match previous.x.cmp(&next.x) {
            std::cmp::Ordering::Less => Vec2::X,
            std::cmp::Ordering::Equal => {
                if previous.y > next.y {
                    Vec2::NEG_Y
                } else {
                    Vec2::Y
                }
            }
            std::cmp::Ordering::Greater => Vec2::NEG_X,
        };

        let new_line = Aabb::new(previous + delta, next);
        let Some(intersecting_line) = self
            .visited
            .windows(2)
            .map(|window| Aabb::new(window[0], window[1]))
            .find(|line| line.intersects(&new_line))
        else {
            self.visited.push(next);
            return None;
        };

        // find exact intersection
        let delta = match new_line.direction {
            Direction::N | Direction::E => 1,
            Direction::S | Direction::W => -1,
        };

        match new_line.direction {
            Direction::N | Direction::S => {
                // x is locked
                let mut y = previous.y;
                let x = previous.x;

                while y != next.y {
                    let point = Vec2 { x, y };
                    if intersecting_line.intersects(&Aabb::new(point, point)) {
                        return Some(point);
                    }
                    y += delta;
                }

                if intersecting_line.intersects(&Aabb::new(next, next)) {
                    return Some(next);
                }
                None
            }
            Direction::E | Direction::W => {
                // y is locked
                let y = previous.y;
                let mut x = previous.x;

                while x != next.x {
                    let point = Vec2 { x, y };
                    if intersecting_line.intersects(&Aabb::new(point, point)) {
                        return Some(point);
                    }
                    x += delta;
                }

                if intersecting_line.intersects(&Aabb::new(next, next)) {
                    return Some(next);
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i16,
    y: i16,
}

impl Vec2 {
    const Y: Vec2 = Vec2 { x: 0, y: 1 };
    const X: Vec2 = Vec2 { x: 1, y: 0 };
    const NEG_Y: Vec2 = Vec2 { x: 0, y: -1 };
    const NEG_X: Vec2 = Vec2 { x: -1, y: 0 };
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Player {
    position: Vec2,
    direction: Direction,
}

impl Player {
    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn step(&mut self, steps: i16) {
        match self.direction {
            Direction::N => self.position.y += steps,
            Direction::E => self.position.x += steps,
            Direction::S => self.position.y -= steps,
            Direction::W => self.position.x -= steps,
        }
    }

    fn manhattan_distance(self) -> i16 {
        self.position.x.abs() + self.position.y.abs()
    }
}

fn solve(input: &[u8]) -> i16 {
    let mut iter = input.iter();

    let mut player = Player::default();

    let mut visited = Visited::with_capacity(128);
    visited.check_intersection(player.position);

    while let Some(c) = iter.next() {
        match c {
            b' ' => {
                // ignore spaces
            }
            b',' => {
                panic!("shouldn't encounter a comma, it should be consumed by the number parsing");
            }
            b'R' | b'L' => {
                if *c == b'R' {
                    player.turn_right();
                } else {
                    player.turn_left();
                }
                let mut steps: i16 = 0;
                while let Some(value) = iter.next()
                    && value.is_ascii_digit()
                {
                    steps = steps * 10 + i16::from(*value) - 48;
                }
                player.step(steps);

                if let Some(point) = visited.check_intersection(player.position) {
                    player.position = point;
                    return player.manhattan_distance();
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    player.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"R8, R4, R4, R8");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let result = solve(b"R2, R2, R2, R2");
        assert_eq!(result, 0);
        let result = solve(b"L2, L2, L2, L2");
        assert_eq!(result, 0);
        let result = solve(b"L10, L10, L10, L10");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let result = solve(b"R4, R3, R2, R1, R2");
        assert_eq!(result, 6);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {}

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT);
    }
}
