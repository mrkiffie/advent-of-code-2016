const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Consume,
    Repeat,
    Read,
}

fn solve(input: &[u8]) -> usize {
    let mut total: usize = 0;
    let mut consume: usize = 0;
    let mut repeat: usize = 0;
    let mut state = State::Read;

    let mut tail = input;

    while !tail.is_empty() {
        match tail[0] {
            b'0'..=b'9' => {
                let digit: usize = (tail[0] - b'0').into();
                match state {
                    State::Consume => {
                        consume = consume * 10 + digit;
                    }
                    State::Repeat => {
                        repeat = repeat * 10 + digit;
                    }
                    State::Read => unimplemented!("unexpected digit"),
                }
                tail = &tail[1..];
            }
            b'(' => {
                debug_assert_eq!(state, State::Read);
                debug_assert_eq!(consume, 0);
                state = State::Consume;
                tail = &tail[1..];
            }
            b'x' => {
                debug_assert_eq!(state, State::Consume);
                debug_assert_ne!(consume, 0);
                debug_assert_eq!(repeat, 0);
                state = State::Repeat;
                tail = &tail[1..];
            }
            b')' => {
                debug_assert_eq!(state, State::Repeat);
                debug_assert_ne!(repeat, 0);
                state = State::Read;
                // parse sub clauses
                let inner = solve(&tail[1..=consume]);
                total += inner * repeat;
                tail = &tail[consume + 1..];
                repeat = 0;
                consume = 0;
            }
            x => {
                if !x.is_ascii_whitespace() {
                    total += 1;
                }
                // read char by char
                tail = &tail[1..];
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let input = b"(3x3)XYZ"; // still becomes XYZXYZXYZ, as the decompressed section contains no markers.
        assert_eq!(solve(input), 9);
        let input = b"X(8x2)(3x3)ABCY"; // becomes XABCABCABCABCABCABCY, because the decompressed data from the (8x2) marker is then further decompressed, thus triggering the (3x3) marker twice for a total of six ABC sequences.
        assert_eq!(solve(input), 20);
        let input = b"(27x12)(20x12)(13x14)(7x10)(1x12)A"; //decompresses into a string of A repeated 241920 times.
        assert_eq!(solve(input), 241_920);
        let input = b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"; // becomes 445 characters long.
        assert_eq!(solve(input), 445);
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
