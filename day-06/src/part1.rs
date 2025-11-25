use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> String {
    let size = input
        .find('\n')
        .expect("input should consist of multiple lines");
    let mut counts = vec![HashMap::new(); size];
    for line in input.lines() {
        for (col, char) in line.chars().enumerate() {
            match counts.get_mut(col) {
                Some(map) => {
                    map.entry(char).and_modify(|entry| *entry += 1).or_insert(1);
                }
                None => unreachable!(),
            }
        }
    }
    let mut message = String::with_capacity(size);

    for col_counts in counts {
        if let Some((c, _count)) = col_counts.iter().max_by(|a, b| a.1.cmp(b.1)) {
            message.push(*c);
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_1() {
        let result = solve(
            "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar",
        );
        assert_eq!(result, "easter");
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
