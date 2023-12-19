use std::collections::HashMap;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day19/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day19/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let workflows = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (id, line) = line.split_once('{').unwrap();
            let line = line.strip_suffix('}').unwrap();
            let workflow = Workflow {
                conditions: line.split(',').map(Condition::from).collect(),
            };

            (id, workflow)
        })
        .collect::<HashMap<_, _>>();

    let datasets = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(Dataset::from)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for set in datasets {
        let mut wid = "in";
        loop {
            wid = match workflows[wid].handle(&set) {
                Command::Run(next_wid) => &next_wid,
                Command::Accept => {
                    sum += set.total();
                    break;
                }
                Command::Reject => break,
            };
        }
    }

    sum
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day19/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day19/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let workflows = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (id, line) = line.split_once('{').unwrap();
            let line = line.strip_suffix('}').unwrap();
            let workflow = Workflow {
                conditions: line.split(',').map(Condition::from).collect(),
            };

            (id, workflow)
        })
        .collect::<HashMap<_, _>>();

    let mut sum = 0;
    let mut queue = vec![("in", Datarange::whole())];
    while let Some((wid, mut datarange)) = queue.pop() {
        let workflow = &workflows[wid];
        for condition in &workflow.conditions {
            match &condition.when {
                Some((get, cmp)) => {
                    let mut exit_datarange = Datarange::whole();
                    let mut enter_datarange = Datarange::whole();
                    match cmp {
                        Comparison::LessThan(cmp) => match get {
                            Getter::A => {
                                enter_datarange.end.a = *cmp;
                                exit_datarange.start.a = *cmp;
                            }
                            Getter::M => {
                                enter_datarange.end.m = *cmp;
                                exit_datarange.start.m = *cmp;
                            }
                            Getter::S => {
                                enter_datarange.end.s = *cmp;
                                exit_datarange.start.s = *cmp;
                            }
                            Getter::X => {
                                enter_datarange.end.x = *cmp;
                                exit_datarange.start.x = *cmp;
                            }
                        },
                        Comparison::MoreThan(cmp) => match get {
                            Getter::A => {
                                enter_datarange.start.a = *cmp + 1;
                                exit_datarange.end.a = *cmp + 1;
                            }
                            Getter::M => {
                                enter_datarange.start.m = *cmp + 1;
                                exit_datarange.end.m = *cmp + 1;
                            }
                            Getter::S => {
                                enter_datarange.start.s = *cmp + 1;
                                exit_datarange.end.s = *cmp + 1;
                            }
                            Getter::X => {
                                enter_datarange.start.x = *cmp + 1;
                                exit_datarange.end.x = *cmp + 1;
                            }
                        },
                    }
                    if let Some(enter_datarange) = enter_datarange.intersection(datarange) {
                        match &condition.command {
                            Command::Run(next_wid) => {
                                queue.push((next_wid, enter_datarange));
                            }
                            Command::Accept => {
                                sum += (enter_datarange.end.a - enter_datarange.start.a)
                                    * (enter_datarange.end.m - enter_datarange.start.m)
                                    * (enter_datarange.end.s - enter_datarange.start.s)
                                    * (enter_datarange.end.x - enter_datarange.start.x);
                            }
                            Command::Reject => {}
                        }
                    }
                    if let Some(exit_datarange) = exit_datarange.intersection(datarange) {
                        datarange = exit_datarange;
                    } else {
                        break;
                    }
                }
                None => match &condition.command {
                    Command::Run(next_wid) => {
                        queue.push((next_wid, datarange));
                    }
                    Command::Accept => {
                        sum += (datarange.end.a - datarange.start.a)
                            * (datarange.end.m - datarange.start.m)
                            * (datarange.end.s - datarange.start.s)
                            * (datarange.end.x - datarange.start.x);
                    }
                    Command::Reject => {}
                },
            }
        }
    }

    sum
}

#[derive(Clone, Copy)]
struct Datarange {
    start: Dataset, // inclusive
    end: Dataset,   // exclusive
}

impl Datarange {
    fn whole() -> Self {
        Self {
            start: Dataset {
                a: 1,
                m: 1,
                s: 1,
                x: 1,
            },
            end: Dataset {
                a: 4001,
                m: 4001,
                s: 4001,
                x: 4001,
            },
        }
    }

    fn intersection(self, rhs: Self) -> Option<Self> {
        let start = Dataset {
            a: self.start.a.max(rhs.start.a),
            m: self.start.m.max(rhs.start.m),
            s: self.start.s.max(rhs.start.s),
            x: self.start.x.max(rhs.start.x),
        };
        let end = Dataset {
            a: self.end.a.min(rhs.end.a),
            m: self.end.m.min(rhs.end.m),
            s: self.end.s.min(rhs.end.s),
            x: self.end.x.min(rhs.end.x),
        };

        if end.a > start.a && end.m > start.m && end.s > start.s && end.x > start.x {
            Some(Datarange { start, end })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct Dataset {
    a: usize,
    m: usize,
    s: usize,
    x: usize,
}

impl Dataset {
    fn total(&self) -> usize {
        self.a + self.m + self.s + self.x
    }
}

impl From<&str> for Dataset {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix('{').unwrap();
        let value = value.strip_suffix('}').unwrap();
        let mut set = Dataset {
            a: 0,
            m: 0,
            s: 0,
            x: 0,
        };
        for field in value.split(',') {
            let (k, v) = field.split_once('=').unwrap();
            match k {
                "a" => {
                    set.a = v.parse().unwrap();
                }
                "m" => {
                    set.m = v.parse().unwrap();
                }
                "s" => {
                    set.s = v.parse().unwrap();
                }
                "x" => {
                    set.x = v.parse().unwrap();
                }
                _ => unreachable!(),
            }
        }

        set
    }
}

struct Workflow<'a> {
    conditions: Vec<Condition<'a>>,
}

impl<'a> Workflow<'a> {
    fn handle<'b: 'a>(&'b self, set: &'b Dataset) -> &'b Command<'a> {
        for condition in &self.conditions {
            if let Some(x) = condition.handle(set) {
                return x;
            }
        }
        unreachable!()
    }
}

struct Condition<'a> {
    when: Option<(Getter, Comparison)>,
    command: Command<'a>,
}

impl<'a> Condition<'a> {
    fn handle<'b: 'a>(&'b self, set: &'b Dataset) -> Option<&'b Command<'a>> {
        let Some((getter, comparison)) = &self.when else {
            return Some(&self.command);
        };

        let value = match getter {
            Getter::A => &set.a,
            Getter::M => &set.m,
            Getter::S => &set.s,
            Getter::X => &set.x,
        };
        match comparison {
            Comparison::LessThan(cmp) if value < cmp => Some(&self.command),
            Comparison::LessThan(_) => None,
            Comparison::MoreThan(cmp) if value > cmp => Some(&self.command),
            Comparison::MoreThan(_) => None,
        }
    }
}

impl<'a> From<&'a str> for Condition<'a> {
    fn from(value: &'a str) -> Self {
        if let Some((pre, post)) = value.split_once(':') {
            let getter = match &pre[..1] {
                "a" => Getter::A,
                "m" => Getter::M,
                "s" => Getter::S,
                "x" => Getter::X,
                _ => unreachable!(),
            };
            let compared = pre[2..].parse().unwrap();
            let comparison = match &pre[1..2] {
                "<" => Comparison::LessThan(compared),
                ">" => Comparison::MoreThan(compared),
                _ => unreachable!(),
            };

            Self {
                when: Some((getter, comparison)),
                command: match post {
                    "A" => Command::Accept,
                    "R" => Command::Reject,
                    run => Command::Run(run),
                },
            }
        } else {
            Self {
                when: None,
                command: match value {
                    "A" => Command::Accept,
                    "R" => Command::Reject,
                    run => Command::Run(run),
                },
            }
        }
    }
}

enum Getter {
    A,
    M,
    S,
    X,
}

enum Comparison {
    LessThan(usize),
    MoreThan(usize),
}

enum Command<'a> {
    Run(&'a str),
    Accept,
    Reject,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 19114);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 167409079868000);
    }
}
