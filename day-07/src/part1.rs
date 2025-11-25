const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    input.lines().filter(|line| check(line)).count()
}

fn check(input: &str) -> bool {
    let mut valid = false;
    let mut outside = true;
    for window in input.as_bytes().windows(4) {
        if window.contains(&b'[') {
            outside = false;
            continue;
        }
        if window.contains(&b']') {
            outside = true;
            continue;
        }
        match window[0..4] {
            [a, b, c, d] => {
                if a == d && b == c && a != b {
                    if outside {
                        valid = true;
                    } else {
                        return false;
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn text_check() {
        assert!(check("abba[mnop]qrst")); // supports TLS (abba outside square brackets).
        assert!(!check("abcd[bddb]xyyx")); // does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
        assert!(!check("aaaa[qwer]tyui")); // does not support TLS (aaaa is invalid; the interior characters must be different).
        assert!(check("ioxxoj[asdfgh]zxcvbn")); // supports TLS (oxxo is outside square brackets, even though it's within a larger string).
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
