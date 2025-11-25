use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    input.lines().filter(|line| check(line)).count()
}

fn check(input: &str) -> bool {
    let mut inner = HashSet::new();
    let mut outer = HashSet::new();
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
                        outer.insert((a, b));
                    } else {
                        inner.insert((b, a));
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }

    inner.intersection(&outer).count() > 0
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn text_check() {
        assert!(check("aba[bab]xyz")); // supports SSL (aba outside square brackets with corresponding bab within square brackets).
        assert!(!check("xyx[xyx]xyx")); // does not support SSL (xyx, but no corresponding yxy).
        assert!(check("aaa[kek]eke")); // supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
        assert!(check("zazbz[bzb]cdb")); // supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
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
