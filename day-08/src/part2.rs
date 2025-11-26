const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: u8,
    y: u8,
}

const ROWS: u8 = 6;
const COLS: u8 = 50;
fn solve(input: &str) -> String {
    let pixels = parse_pixels(input);
    format_output(pixels)
}

fn parse_pixels(input: &str) -> Vec<Vec2> {
    let mut pixels = vec![];
    input
        .lines()
        .fold(&mut pixels, |pixels: &mut Vec<Vec2>, line: &str| {
            if let Some(instruction) = line.strip_prefix("rotate column x=")
                && let Some((x, delta)) = instruction.split_once(" by ")
            {
                let x = x.parse::<u8>().unwrap();
                let delta = delta.parse::<u8>().unwrap();
                for pixel in pixels.iter_mut() {
                    if pixel.x == x {
                        pixel.y = (pixel.y + delta) % ROWS;
                    }
                }
            } else if let Some(instruction) = line.strip_prefix("rotate row y=")
                && let Some((y, delta)) = instruction.split_once(" by ")
            {
                let y = y.parse::<u8>().unwrap();
                let delta = delta.parse::<u8>().unwrap();
                for pixel in pixels.iter_mut() {
                    if pixel.y == y {
                        pixel.x = (pixel.x + delta) % COLS;
                    }
                }
            } else if let Some(instruction) = line.strip_prefix("rect ")
                && let Some((x, y)) = instruction.split_once('x')
            {
                let cols = x.parse::<u8>().unwrap();
                let rows = y.parse::<u8>().unwrap();
                for x in 0..cols {
                    for y in 0..rows {
                        pixels.push(Vec2 { x, y });
                    }
                }
            }
            pixels
        });
    pixels
}

fn format_output(pixels: Vec<Vec2>) -> String {
    let mut output = String::new();
    let mut grid = [[0; COLS as usize]; ROWS as usize];
    for pixel in pixels {
        grid[pixel.y as usize][pixel.x as usize] = 1;
    }
    for row in grid {
        for c in row {
            if c == 1 {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{INPUT, solve};

    #[test]
    fn test_1() {
        let result = solve(INPUT);
        assert_eq!(
            result,
            "####  ##   ##  ###   ##  ###  #  # #   # ##   ##  \n#    #  # #  # #  # #  # #  # #  # #   ##  # #  # \n###  #  # #  # #  # #    #  # ####  # # #  # #  # \n#    #  # #### ###  # ## ###  #  #   #  #### #  # \n#    #  # #  # # #  #  # #    #  #   #  #  # #  # \n####  ##  #  # #  #  ### #    #  #   #  #  #  ##  \n"
        );
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
