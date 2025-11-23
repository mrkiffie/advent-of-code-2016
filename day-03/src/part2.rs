const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    let mut iter = input.lines();

    let mut count = 0;
    while let (Some(row1), Some(row2), Some(row3)) = (iter.next(), iter.next(), iter.next()) {
        if let (
            Some((col1_a, col2_a, col3_a)),
            Some((col1_b, col2_b, col3_b)),
            Some((col1_c, col2_c, col3_c)),
        ) = (parse_line(row1), parse_line(row2), parse_line(row3))
        {
            if is_triangle(col1_a, col1_b, col1_c) {
                count += 1;
            }
            if is_triangle(col2_a, col2_b, col2_c) {
                count += 1;
            }
            if is_triangle(col3_a, col3_b, col3_c) {
                count += 1;
            }
        }
    }
    count
}

#[inline]
fn parse_line(line: &str) -> Option<(i32, i32, i32)> {
    let mut parts = line
        .split_whitespace()
        .filter_map(|value| value.parse::<i32>().ok());
    match (parts.next(), parts.next(), parts.next()) {
        (Some(a), Some(b), Some(c)) => Some((a, b, c)),
        _ => None,
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
        let result = solve("1 3 6\n2 4 8\n4 6 10");
        assert_eq!(result, 2);
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
