const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"");
        assert_eq!(result, 0);
    }
}
