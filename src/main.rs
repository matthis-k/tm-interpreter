use serde::{Deserialize, Serialize};
use serde_scan;
use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
struct TuringMachine<'a> {
    tape: HashMap<i32, char>,
    head: i32,
    min_head: i32,
    max_head: i32,
    state: String,
    accepting_states: Vec<String>,
    transitions: Vec<Transition<'a>>,
}

impl<'a> From<&'a str> for TuringMachine<'a> {
    fn from(s: &'a str) -> Self {
        let mut alphabet = "";
        let mut tape_str = "";
        let mut tape_offset = 0;
        let mut start_state = "".into();
        let mut accepted_states = Vec::new();
        let mut rules = Vec::new();
        for line in s.lines() {
            if let Some((field, val)) = line.split_once(":") {
                let val = val.trim();
                match field {
                    "tape" => tape_str = val.clone(),
                    "tape_offset" => tape_offset = val.parse::<i32>().expect("number format"),
                    "start_state" => start_state = val.into(),
                    "accepted_states" => accepted_states.push(val.into()),
                    "rule" => {
                        let (state, cin, cout, mov, next_state): (&str, &str, &str, char, &str) =
                            serde_scan::from_str(val).unwrap();
                        let transition = Transition(
                            state.into(),
                            ReadPattern::from(cin),
                            WriteAction::from(cout),
                            Movement::from(mov),
                            next_state.into(),
                        );
                        rules.push(transition);
                    }
                    "alphabet" => alphabet = val,
                    _ => {}
                }
            }
        }
        let mut tape = HashMap::new();
        for (i, c) in tape_str.chars().enumerate() {
            if alphabet.contains(c) {
                tape.insert(tape_offset + i as i32, c);
            }
        }
        let min_head = tape.keys().min().unwrap_or(&0).clone();
        let max_head = tape.keys().max().unwrap_or(&0).clone();
        TuringMachine {
            tape,
            head: 0,
            min_head,
            max_head,
            state: start_state,
            accepting_states: accepted_states,
            transitions: rules,
        }
    }
}

impl<'a> TuringMachine<'a> {
    fn run(&mut self) -> bool {
        loop {
            println!("{}", self);

            let transition = self.get_transition();
            match transition {
                Some(transition) => self.step(transition),
                None => break,
            }
        }
        self.accepting_states.contains(&self.state)
    }

    fn step(&mut self, transition: Transition) {
        match transition.2 {
            WriteAction::Write(c) => {
                self.tape.insert(self.head.clone(), c);
            }
            WriteAction::Delete => {
                self.tape.remove(&self.head);
            }
            WriteAction::None => {}
        }
        self.move_head(&transition.3);
        self.state = transition.4.to_owned();
    }

    fn move_head(&mut self, movement: &Movement) {
        match movement {
            Movement::Left => {
                self.head -= 1;
                if self.head < self.min_head {
                    self.min_head = self.head;
                }
            }
            Movement::Right => {
                self.head += 1;
                if self.head > self.max_head {
                    self.max_head = self.head;
                }
            }
            Movement::Stay => (),
        }
    }

    fn get_transition(&self) -> Option<Transition<'a>> {
        self.transitions
            .iter()
            .filter(|&x| x.filter_state(&self.state))
            .filter(|x| x.filter_tape(self.tape.get(&self.head)))
            .map(Clone::clone)
            .next()
    }
}

impl<'a> Display for TuringMachine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in self.min_head..=self.max_head {
            f.write_fmt(format_args!("{}", self.tape.get(&k).unwrap_or(&' ')))?;
        }
        f.write_fmt(format_args!(":{}\n", self.state))?;
        f.write_fmt(format_args!(
            "{}^head",
            " ".repeat(self.head.abs_diff(self.min_head) as usize)
        ))?;
        Ok(())
    }
}

/// A state transition: (currentState, read, write, movement, nextState)
#[derive(Clone)]
struct Transition<'a>(&'a str, ReadPattern<'a>, WriteAction, Movement, &'a str);
impl<'a> Transition<'a> {
    fn filter_state(&self, state: &String) -> bool {
        self.0 == *state
    }

    fn filter_tape(&self, c: Option<&char>) -> bool {
        self.1.matches(c)
    }
}
#[derive(Clone, Serialize, Deserialize)]
enum Movement {
    Left,
    Right,
    Stay,
}

impl From<char> for Movement {
    fn from(c: char) -> Self {
        match c {
            'l' => Self::Left,
            'r' => Self::Right,
            _ => Self::Stay,
        }
    }
}

#[derive(Clone)]
enum ReadPattern<'a> {
    Any,
    Empty,
    Some(&'a str),
}

impl<'a> ReadPattern<'a> {
    fn matches(&self, co: Option<&char>) -> bool {
        match self {
            ReadPattern::Any => co.is_some(),
            ReadPattern::Empty => co.is_none(),
            ReadPattern::Some(cs) => {
                if let Some(c) = co {
                    cs.contains(*c)
                } else {
                    false
                }
            }
        }
    }
}

impl<'a> From<&'a str> for ReadPattern<'a> {
    fn from(c: &'a str) -> Self {
        match c {
            "any" => Self::Any,
            "empty" => Self::Empty,
            s => Self::Some(s),
        }
    }
}

#[derive(Clone)]
enum WriteAction {
    None,
    Delete,
    Write(char),
}

impl From<&str> for WriteAction {
    fn from(s: &str) -> Self {
        match s {
            "none" => Self::None,
            "Delete" => Self::Delete,
            c => Self::Write(c.chars().next().unwrap()),
        }
    }
}

fn main() {
    let src = std::fs::read_to_string("goal.tm").expect("read file without errors");
    let mut tm: TuringMachine = TuringMachine::from(src.as_str());
    println!("{}", tm.run());
}
