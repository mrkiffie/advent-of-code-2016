use std::collections::HashSet;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

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

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
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

    fn step(&mut self, steps: i32) {
        match self.direction {
            Direction::N => self.position.y += steps,
            Direction::E => self.position.x += steps,
            Direction::S => self.position.y -= steps,
            Direction::W => self.position.x -= steps,
        }
    }

    fn manhattan_distance(self) -> i32 {
        self.position.x.abs() + self.position.y.abs()
    }
}

fn solve(input: &[u8]) -> i32 {
    let mut iter = input.iter();

    let mut player = Player::default();

    let mut visited: HashSet<Vec2> = HashSet::new();
    visited.insert(player.position);

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
                let mut steps: i32 = 0;
                while let Some(value) = iter.next()
                    && value.is_ascii_digit()
                {
                    steps = steps * 10 + i32::from(*value) - 48;
                }

                for _ in 0..steps {
                    player.step(1);
                    if visited.contains(&player.position) {
                        return player.manhattan_distance();
                    }
                    visited.insert(player.position);
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

    pub fn main() {
        divan::main();
    }

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT);
    }
}
