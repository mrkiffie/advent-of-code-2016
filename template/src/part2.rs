#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

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

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT);
    }
}
