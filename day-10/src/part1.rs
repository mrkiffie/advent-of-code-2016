use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./input.txt");

const FIND_LOW: u8 = 17;
const FIND_HIGH: u8 = 61;

pub fn main() {
    let result = solve(INPUT, FIND_LOW, FIND_HIGH);
    println!("{result}");
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct BotId(u8);
impl BotId {
    fn new(id: &str) -> Self {
        id.parse::<u8>().ok().map(Self).expect("Unable to parse id")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ChipId(u8);
impl ChipId {
    fn new(id: &str) -> Self {
        id.parse::<u8>().ok().map(Self).expect("Unable to parse id")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Bot(BotId),
    Output(OutputId),
}
impl Target {
    fn new(target_type: &str, target_id: &str) -> Target {
        match target_type {
            "bot" => Target::Bot(BotId::new(target_id)),
            "output" => Target::Output(OutputId::new(target_id)),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OutputId(u8);
impl OutputId {
    fn new(id: &str) -> Self {
        id.parse::<u8>().ok().map(Self).expect("id should be valid")
    }
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    id: BotId,
    low_value: Option<ChipId>,
    high_value: Option<ChipId>,
    low_target: Option<Target>,
    high_target: Option<Target>,
}
impl Bot {
    fn with_value(bot: BotId, value: ChipId) -> Self {
        Self {
            id: bot,
            low_value: Some(value),
            high_value: None,
            low_target: None,
            high_target: None,
        }
    }

    fn set_value(&mut self, value: ChipId) {
        match (self.low_value, self.high_value) {
            (None, None) => {
                self.low_value = Some(value);
            }
            (None, Some(_)) => {
                panic!("Shouldn't have high value without having a low value");
            }
            (Some(low_value), None) => {
                if value < low_value {
                    self.high_value = self.low_value;
                    self.low_value = Some(value);
                } else {
                    self.high_value = Some(value);
                }
            }
            (Some(_), Some(_)) => {
                panic!("Shouldn't be setting values when both low and high values are set");
            }
        }
    }
}

fn parse(input: &str) -> HashMap<BotId, Bot> {
    let mut bots: HashMap<BotId, Bot> = HashMap::with_capacity(210);

    for line in input.lines() {
        let mut iter = line.split(' ');

        match iter.next() {
            Some("bot") => {
                let bot_id = iter.next().map(BotId::new).expect("valid puzzle input");
                // skip "gives low to"
                let low_target = iter.nth(3).expect("valid puzzle input");
                let low_target_id = iter.next().expect("valid puzzle input");
                // skip "and high to"
                let high_target = iter.nth(3).expect("valid puzzle input");
                let high_target_id = iter.next().expect("valid puzzle input");
                let low_target = Some(Target::new(low_target, low_target_id));
                let high_target = Some(Target::new(high_target, high_target_id));

                bots.entry(bot_id)
                    .and_modify(|bot| {
                        bot.low_target = low_target;
                        bot.high_target = high_target;
                    })
                    .or_insert(Bot {
                        id: bot_id,
                        low_value: None,
                        high_value: None,
                        low_target,
                        high_target,
                    });
            }
            Some("value") => {
                let chip_id = iter.next().map(ChipId::new).expect("valid puzzle input");
                // skip "goes to bot"
                let bot_id = iter.nth(3).map(BotId::new).expect("valid puzzle input");

                bots.entry(bot_id)
                    .and_modify(|bot| {
                        bot.set_value(chip_id);
                    })
                    .or_insert(Bot::with_value(bot_id, chip_id));
            }
            Some(prefix) => unimplemented!("Unexpected prefix {}", prefix),
            None => unreachable!("Line should have some content"),
        }
    }
    bots
}

fn solve(input: &str, find_low: u8, find_high: u8) -> u8 {
    let mut queue: VecDeque<Bot> = parse(input).into_values().collect();
    while let Some(mut bot) = queue.pop_front() {
        if let (Some(low), Some(high), Some(low_target), Some(high_target)) = (
            bot.low_value,
            bot.high_value,
            bot.low_target,
            bot.high_target,
        ) {
            // Termination clause
            if low == ChipId(find_low) && high == ChipId(find_high) {
                return bot.id.0;
            }

            let mut low_given = false;
            let mut high_given = false;
            while let Some(mut bot_to_check) = queue.pop_front() {
                match low_target {
                    Target::Bot(bot_id) => {
                        if bot_id == bot_to_check.id {
                            bot_to_check.set_value(low);
                            bot.low_value = None;
                            low_given = true;
                        }
                    }
                    Target::Output(_) => {
                        bot.low_value = None;
                        low_given = true;
                    }
                }
                match high_target {
                    Target::Bot(bot_id) => {
                        if bot_id == bot_to_check.id {
                            bot_to_check.set_value(high);
                            bot.high_value = None;
                            high_given = true;
                        }
                    }
                    Target::Output(_) => {
                        bot.high_value = None;
                        high_given = true;
                    }
                }
                queue.push_back(bot_to_check);
                if low_given && high_given {
                    break;
                }
            }
        } else {
            // bot doesn't have both required values, pushing it to the back of the queue
            queue.push_back(bot);
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            "value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2",
            2,
            5,
        );
        assert_eq!(result, 2);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::FIND_HIGH;
    use super::FIND_LOW;
    use super::INPUT;

    pub fn main() {}

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT, FIND_LOW, FIND_HIGH);
    }
}
