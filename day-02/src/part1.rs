use glam::IVec2;
use std::ops::Add;

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> i32 {
    let mut pos = IVec2::ZERO;
    let mut code = Vec::with_capacity(5);
    for c in input {
        match c {
            b'\n' => code.push(pos),
            b'U' => pos = pos.add(IVec2::Y).clamp(IVec2::NEG_ONE, IVec2::ONE),
            b'D' => pos = pos.add(IVec2::NEG_Y).clamp(IVec2::NEG_ONE, IVec2::ONE),
            b'L' => pos = pos.add(IVec2::NEG_X).clamp(IVec2::NEG_ONE, IVec2::ONE),
            b'R' => {
                pos = pos.add(IVec2::X).clamp(IVec2::NEG_ONE, IVec2::ONE);
            }
            _ => {
                unimplemented!()
            }
        }
    }

    code.iter()
        .rev()
        .enumerate()
        .map(|(i, p)| {
            let digit = match (p.x, p.y) {
                (-1, 1) => 1,
                (0, 1) => 2,
                (1, 1) => 3,
                (-1, 0) => 4,
                (0, 0) => 5,
                (1, 0) => 6,
                (-1, -1) => 7,
                (0, -1) => 8,
                (1, -1) => 9,
                _ => unimplemented!(),
            };

            digit * 10_i32.pow(u32::try_from(i).unwrap_or_default())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(result, 1985);
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
