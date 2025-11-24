const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(&INPUT[..8]);
    match String::from_utf8(result.to_vec()) {
        Ok(result) => println!("{result}"),
        Err(err) => eprintln!("{err:?}"),
    }
}

fn solve(input: &[u8]) -> [u8; 8] {
    let mut password = [u8::MAX; 8];
    let mut j: usize = 0;
    let mut count: usize = 0;
    loop {
        if let Some((i, c)) = get_hash(input, j)
            && password[i as usize] == u8::MAX
        {
            password[i as usize] = match c {
                0..=9 => c + b'0',
                _ => c + b'a' - 10,
            };
            count += 1;
            if count == password.len() {
                break;
            }
        }
        j += 1;
    }
    password
}

#[inline]
fn get_hash(input: &[u8], number: usize) -> Option<(u8, u8)> {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update([input, format!("{number}").as_bytes()].concat());
    let hash = hasher.finalize();
    let hash = &hash.as_slice()[..4];
    match hash {
        [0, 0, i, x] if *i < 8 => Some((*i, *x >> 4)),
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
        assert_eq!(result, Some((1, 5)));

        let result = get_hash(b"abc", 5017308);
        assert_eq!(result, None);

        let result = get_hash(b"abc", 5357525);
        assert_eq!(result, Some((4, 14)));
    }

    #[test]
    fn test_1() {
        let result = solve(b"abc");
        assert_eq!(&result, b"05ace8e3");
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
