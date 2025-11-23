const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    input.lines().filter(parse_line).count()
}

#[inline]
fn parse_line(line: &&str) -> bool {
    let mut parts = line
        .split_whitespace()
        .filter_map(|value| value.parse::<i32>().ok());
    match (parts.next(), parts.next(), parts.next()) {
        (Some(a), Some(b), Some(c)) => is_triangle(a, b, c),
        _ => false,
    }
}

#[inline]
fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && a + c > b && b + c > a
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve("5 10 25");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let result = solve("5 10 6\n10 5 6\n6 5 10");
        assert_eq!(result, 3);
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
