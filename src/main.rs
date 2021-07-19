use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    return vec![
        State {
            phrase: format!("{}{}", state.phrase, "A"),
            iterations: state.iterations + 1,
            ..state
        },
        State {
            phrase: format!("{}{}", state.phrase, "B"),
            iterations: state.iterations + 1,
            ..state
        },
        State {
            phrase: state.phrase,
            ..state
        }
    ]
}

fn invariant(state: State) -> bool {
    state.phrase != "BAAB"
}

struct StateNode {
    state: State,
    parent: Option<usize>,
    children: Vec<usize>
}

struct StateTree {
    nodes: Vec<StateNode>,
}

impl StateTree {
    fn new() -> StateTree {
        StateTree {
            nodes: vec![]
        }
    }

    fn get_backtrace(&self, node_idx: usize) -> Vec<State> {
        let mut node = &self.nodes[node_idx];

        let mut trace = vec![node.state.clone()];

        while node.parent.is_some() {
            node = &self.nodes[node.parent.unwrap()];
            trace.push(node.state.clone())
        }

        trace
    }
}

fn main() {
    let initial_state = init();

    let mut state_queue = vec![initial_state];

    let mut seen: HashSet<State> = HashSet::new();

    while !state_queue.is_empty() {
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

        println!("{:#?}", prev_state);
        let mut next_states = next(prev_state.clone());

        state_queue.append(&mut next_states);
        seen.insert(prev_state);
    }
}
