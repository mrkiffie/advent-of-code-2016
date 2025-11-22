const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    match String::from_utf8(result) {
        Ok(result) => println!("{result}"),
        Err(err) => eprintln!("{err}"),
    }
}

enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
}

impl Key {
    fn to_ascii(&self) -> u8 {
        match self {
            Key::Key1 => b'1',
            Key::Key2 => b'2',
            Key::Key3 => b'3',
            Key::Key4 => b'4',
            Key::Key5 => b'5',
            Key::Key6 => b'6',
            Key::Key7 => b'7',
            Key::Key8 => b'8',
            Key::Key9 => b'9',
            Key::KeyA => b'A',
            Key::KeyB => b'B',
            Key::KeyC => b'C',
            Key::KeyD => b'D',
        }
    }

    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    fn go(self, c: u8) -> Self {
        use Key::{Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, KeyA, KeyB, KeyC, KeyD};
        match c {
            b'U' => match self {
                Key3 => Key1,
                Key6 => Key2,
                Key7 => Key3,
                Key8 => Key4,
                Key1 | Key2 | Key4 | Key5 | Key9 => self,
                KeyA => Key6,
                KeyB => Key7,
                KeyC => Key8,
                KeyD => KeyB,
            },
            b'D' => match self {
                Key1 => Key3,
                Key2 => Key6,
                Key3 => Key7,
                Key4 => Key8,
                Key5 | Key9 | KeyA | KeyC | KeyD => self,
                Key6 => KeyA,
                Key7 => KeyB,
                Key8 => KeyC,
                KeyB => KeyD,
            },
            b'L' => match self {
                Key3 => Key2,
                Key4 => Key3,
                Key6 => Key5,
                Key7 => Key6,
                Key8 => Key7,
                Key9 => Key8,
                KeyB => KeyA,
                KeyC => KeyB,
                Key1 | Key2 | Key5 | KeyA | KeyD => self,
            },
            b'R' => match self {
                Key2 => Key3,
                Key3 => Key4,
                Key5 => Key6,
                Key6 => Key7,
                Key7 => Key8,
                Key8 => Key9,
                KeyA => KeyB,
                KeyB => KeyC,
                Key1 | Key4 | Key9 | KeyC | KeyD => self,
            },

            _ => unimplemented!(),
        }
    }
}

fn solve(input: &[u8]) -> Vec<u8> {
    let mut key = Key::Key5;

    let mut code = Vec::with_capacity(5);

    for c in input {
        match c {
            b'\n' => {
                code.push(key.to_ascii());
            }
            _ => {
                key = key.go(*c);
            }
        }
    }

    code
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(result, b"5DB3");
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
