use std::collections::HashSet;

#[derive(Debug)]
struct State {
    phrase: String,
    iterations: u64
}

fn init() -> State {
    State {
        phrase: "".into(),
        iterations: 0 
    }
}

fn next(state: State) -> Vec<State> {
    if state.iterations > 5 {
        return vec![]
    }
    
    let state = State {
        iterations: state.iterations + 1,
        ..state
    };

    return vec![
        State {
            phrase: format!("{}{}", state.phrase, "A"),
            ..state
        },
        State {
            phrase: format!("{}{}", state.phrase, "B"),
            ..state
        }
    ]
}

struct StateNode {
    state: State,
    parent: usize,
    children: Vec<usize>
}

struct StateTree {
    nodes: Vec<StateNode>,
    seen: HashSet<State>
}

fn main() {
    let initial_state = init();

    let mut state_queue = vec![initial_state];

    while !state_queue.is_empty() {
        let prev_state = state_queue.pop().unwrap();
        println!("{:#?}", prev_state);
        state_queue.append(&mut next(prev_state))
    }
}
