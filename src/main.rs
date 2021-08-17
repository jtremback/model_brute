use std::collections::HashSet;

struct Next {
    states: Vec<Option<State>>,
}

impl Next {
    // fn code(&mut self, closure: fn (State) -> Option<State>) -> Next {
    //     let new_states: Vec<State> = vec![];
    //     for state in self.states {
    //         if let Some(new_state) = closure(state) {
    //             new_states.push(new_state);
    //         }
    //     }
    //     self.states = new_states;
    //     return *self;
    // }

    fn code(mut self, closure: fn(&mut State) -> bool) -> Next {
        for (i, state) in self.states.clone().iter().enumerate() {
            // for state in self.states {
            if let Some(some_state) = state {
                let mut some_state_copy = some_state.clone();
                if !closure(&mut some_state_copy) {
                    self.states[i] = None
                }
                self.states[i] = Some(some_state_copy)
            }
        }
        dbg!(self.states.clone());
        self
    }

    fn branch(mut self, closures: Vec<fn(&mut State) -> bool>) -> Next {
        dbg!(self.states.clone());
        let mut states = vec![];
        for closure in closures {
            let mut states_copy = self.states.clone();
            for (i, state) in states_copy.clone().iter().enumerate() {
                if let Some(some_state) = state {
                    let mut some_state_copy = some_state.clone();
                    if !closure(&mut some_state_copy) {
                        dbg!("returned false");
                        states_copy[i] = None
                    } else {
                        states_copy[i] = Some(some_state_copy)
                    }
                }
            }
            states.append(&mut states_copy)
        }
        dbg!(states.clone());
        self.states = states;
        self
    }

    fn output(self) -> Vec<State> {
        self.states
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    phrase: String,
    iterations: u64,
}

fn invariant(state: State) -> bool {
    state.phrase != "BAAB"
}

fn next(input: State) -> Vec<State> {
    Next {
        states: vec![Some(input)],
    }
    .branch(vec![|mut state| {
        dbg!("first");
        if state.iterations > 5 {
            return false;
        }

        state.iterations += 1;
        true
    }])
    .branch(vec![
        |mut state| {
            dbg!("second");
            state.phrase = format!("{}{}", state.phrase, "A");

            true
        },
        |mut state| {
            dbg!("third");
            state.phrase = format!("{}{}", state.phrase, "B");

            true
        },
    ])
    .output()
}

fn main() {
    let initial_state = State {
        iterations: 0,
        phrase: "".into(),
    };

    let mut state_queue = vec![initial_state];

    let mut seen: HashSet<State> = HashSet::new();

    let mut i = 0;

    while !state_queue.is_empty() {
        i += 1;
        let prev_state = state_queue.pop().unwrap();
        // If we have seen a state before, we skip it. fn next is completely deterministic depending on previous state
        // so there is no point in running.
        if seen.contains(&prev_state) {
            continue;
        }

        if !invariant(prev_state.clone()) {
            println!("Invariant violated with state: {:#?}", prev_state);
            break;
        }

        let mut next_states = next(prev_state.clone());

        state_queue.append(&mut next_states);
        seen.insert(prev_state);
    }
}
