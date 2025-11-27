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
                total += consume * repeat;
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
        let input = b"ADVENT"; // contains no markers and decompresses to itself with no changes, resulting in a decompressed length of 6.
        assert_eq!(solve(input), 6);
        let input = b"A(1x5)BC"; // repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
        assert_eq!(solve(input), 7);
        let input = b"(3x3)XYZ"; // becomes XYZXYZXYZ for a decompressed length of 9.
        assert_eq!(solve(input), 9);
        let input = b"A(2x2)BCD(2x2)EFG"; // doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
        assert_eq!(solve(input), 11);
        let input = b"(6x1)(1x3)A"; // simply becomes (1x3)A - the (1x3) looks like a marker, but because it's within a data section of another marker, it is not treated any differently from the A that comes after it. It has a decompressed length of 6.
        assert_eq!(solve(input), 6);
        let input = b"X(8x2)(3x3)ABCY"; // becomes X(3x3)ABC(3x3)ABCY (for a decompressed length of 18), because the decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.
        assert_eq!(solve(input), 18);
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
