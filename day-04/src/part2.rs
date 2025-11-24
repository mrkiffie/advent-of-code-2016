const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    match result {
        Some(result) => println!("{result}"),
        None => eprintln!("No result"),
    }
}

fn solve(input: &str) -> Option<u32> {
    input
        .trim()
        .lines()
        .find_map(|line| match validate_room(line) {
            Some(sector_id) => north_prefix(line, sector_id),
            None => None,
        })
}

fn north_prefix(line: &str, sector_id: u32) -> Option<u32> {
    let reverse_offset: u8 = 26 - (sector_id % 26) as u8;
    let prefix = b"north"
        .iter()
        .map(|c| ((c - b'a' + reverse_offset) % 26) + b'a');
    let iter = line.as_bytes().iter().take(5).copied();
    if prefix.eq(iter) {
        Some(sector_id)
    } else {
        None
    }
}

fn validate_room(input: &str) -> Option<u32> {
    let mut bracket_index = 0;
    let mut digit_index = 0;
    let mut counts = [(0, 0); 26];
    for (i, c) in input.as_bytes().iter().enumerate() {
        match c {
            b'a'..=b'z' => {
                let c_as_index = char_to_index(*c);
                counts[c_as_index].0 = *c;
                counts[c_as_index].1 += 1;
            }
            b'-' => {}
            b'0'..=b'9' => {
                if digit_index == 0 {
                    digit_index = i;
                }
            }
            b'[' => {
                bracket_index = i;
                break;
            }
            _ => unimplemented!(),
        }
    }
    let sector_id = &input[digit_index..bracket_index];

    let checksum = &input[bracket_index + 1..bracket_index + 6];

    counts.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, c) in counts.iter().enumerate().take(5) {
        if c.0 != checksum.as_bytes()[i] {
            return None;
        }
    }

    sector_id.parse::<u32>().ok()
}

fn char_to_index(c: u8) -> usize {
    c as usize - 97
}

#[cfg(test)]
mod tests {
    use super::validate_room;

    #[test]
    fn test_validate_room() {
        let result = validate_room("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(result, Some(123));
        let result = validate_room("a-b-c-d-e-f-g-h-987[abcde]");
        assert_eq!(result, Some(987));
        let result = validate_room("not-a-real-room-404[oarel]");
        assert_eq!(result, Some(404));
        let result = validate_room("totally-real-room-200[decoy]");
        assert_eq!(result, None);
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
