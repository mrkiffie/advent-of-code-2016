const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(&INPUT[..8]);
    match String::from_utf8(result.to_vec()) {
        Ok(result) => println!("{result}"),
        Err(err) => eprintln!("{err:?}"),
    }
}

fn solve(input: &[u8]) -> [u8; 8] {
    let mut password = [0; 8];
    let mut j: usize = 0;
    let mut i: usize = 0;
    loop {
        if let Some(c) = get_hash(input, j) {
            password[i] = match c {
                0..=9 => c + b'0',
                _ => c + b'a' - 10,
            };
            i += 1;
            if i == password.len() {
                break;
            }
        }
        j += 1;
    }
    password
}

#[inline]
fn get_hash(input: &[u8], number: usize) -> Option<u8> {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update([input, format!("{number}").as_bytes()].concat());
    let hash = hasher.finalize();
    let hash = &hash.as_slice()[..3];
    match hash {
        [0, 0, x] if *x <= 0xf => Some(*x),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::get_hash;
    use super::solve;

    #[test]
    fn test_get_hash() {
        let result = get_hash(b"abc", 3231929);
        assert_eq!(result, Some(1));

        let result = get_hash(b"abc", 5017308);
        assert_eq!(result, Some(8));

        let result = get_hash(b"abc", 5278568);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_1() {
        let result = solve(b"abc");
        assert_eq!(&result, b"18f47a30");
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
