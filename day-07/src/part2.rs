use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Default)]
struct Scanner {
    inner: HashSet<(u8, u8)>,
    outer: HashSet<(u8, u8)>,
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .scan(Scanner::default(), |scanner: &mut Scanner, line| {
            Some(usize::from(check(scanner, line)))
        })
        .sum()
}

fn check(scanner: &mut Scanner, input: &str) -> bool {
    scanner.outer.clear();
    scanner.inner.clear();
    let mut outside = true;
    for window in input.as_bytes().windows(3) {
        if window.contains(&b'[') {
            outside = false;
            continue;
        }
        if window.contains(&b']') {
            outside = true;
            continue;
        }
        match window[0..3] {
            [a, b, c] => {
                if a == c && a != b {
                    if outside {
                        scanner.outer.insert((a, b));
                    } else {
                        scanner.inner.insert((b, a));
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
    scanner.inner.intersection(&scanner.outer).count() > 0
}

#[cfg(test)]
mod tests {
    use super::{Scanner, check};

    #[test]
    fn text_check() {
        assert!(check(&mut Scanner::default(), "aba[bab]xyz")); // supports SSL (aba outside square brackets with corresponding bab within square brackets).
        assert!(!check(&mut Scanner::default(), "xyx[xyx]xyx")); // does not support SSL (xyx, but no corresponding yxy).
        assert!(check(&mut Scanner::default(), "aaa[kek]eke")); // supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
        assert!(check(&mut Scanner::default(), "zazbz[bzb]cdb")); // supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
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
