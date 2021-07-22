use std::collections::HashSet;

struct Next {
    states: Vec<State>
}

impl Next {
    fn next(state: State) -> Next {
        Next {
            states: vec![state]
        }
    }

    fn code(&mut self, closure: fn (&mut State)) -> Next {
        for state in self.states {
            closure(&mut state);
        }
        return *self;
    }

    fn branch(&mut self, closures: Vec<fn (&mut State)>) -> Next {
        let states = vec![];
        for closure in closures {
            let states_copy = self.states.clone();
            for state in states_copy {
                closure(&mut state);
            }
            states.append(&mut states_copy)
        }
        self.states = states;
        return *self;
    }
}



#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    phrase: String,
    iterations: u64
}

struct Context {
    state: State,
    prime: Vec<State>
}

impl Context {
    fn branch(&mut self, clo: fn (&mut State)) {
        let mut state = self.state.clone();
        clo(&mut state);
        self.prime.push(state);
    }
}

fn init() -> State {
    State {
        phrase: "".into(),
        iterations: 0 
    }
}

fn next(ctx: Context) -> Context {
    if ctx.state.iterations > 5 {
        return prime
    }

    ctx.branch(|&mut state| {
        state.phrase = format!("{}{}", state.phrase, "A");
        ctx.branch(|&mut state| {
            state.iterations += 1;
        })
    })

    prime.push(State {
        phrase: format!("{}{}", state.phrase, "A"),
        iterations: state.iterations + 1,
    });

    prime.push(State {
        phrase: format!("{}{}", state.phrase, "B"),
        iterations: state.iterations + 1,
    });

    prime.push(State {
        phrase: state.phrase,
        ..state
    });

    prime
}

fn invariant(state: State) -> bool {
    state.phrase != "BAAB"
}

fn main() {
    Next::next(State {
        phrase: "".into(),
        iterations: 0 
    })
    .code(|&mut state| {
        if state.iterations > 5 {
            // Figure out how to do assertion to end evaluation
            // maybe just put None into the state
        }
    })
    .branch(vec![
        |&mut state| {
            state.phrase = format!("{}{}", state.phrase, "A");
            state.iterations += 1;
        }, 
        |&mut state| {
            state.phrase = format!("{}{}", state.phrase, "B");
            state.iterations += 1;
        }
    ]);
}

// fn main() {
//     let initial_state = init();

//     let mut state_queue = vec![initial_state];

//     let mut seen: HashSet<State> = HashSet::new();

//     while !state_queue.is_empty() {
//         let prev_state = state_queue.pop().unwrap();
//         // If we have seen a state before, we skip it. fn next is completely deterministic depending on previous state
//         // so there is no point in running.
//         if seen.contains(&prev_state) {
//             continue;
//         }

//         if !invariant(prev_state.clone()) {
//             println!("Invariant violated with state: {:#?}", prev_state);
//             break;
//         }

//         println!("{:#?}", prev_state);
//         let mut next_states = next(prev_state.clone());

//         state_queue.append(&mut next_states);
//         seen.insert(prev_state);
//     }
// }
