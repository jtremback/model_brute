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

macro_rules! branch {
    ($state:expr, $prime:expr, $inner:block) => {
        $prime.push({
            let mut state = $state.clone();
            $inner
            state
        });
    };
}

fn next(state: State) -> Vec<State> {
    let mut prime = vec![state.clone()];

    if state.iterations > 5 {
        return prime
    }

    prime.push({
        let mut state = state.clone();

        {
            state.phrase = format!("{}{}", state.phrase, "A");
            state.iterations += 1;
        }

        state
    });

    // branch! (state, prime, {
    //     state.phrase = format!("{}{}", state.phrase, "A");
    //     state.iterations += 1;
    // });

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

        // if !invariant(prev_state.clone()) {
        //     println!("Invariant violated with state: {:#?}", prev_state);
        //     break;
        // }

        println!("{:#?}", prev_state);
        let mut next_states = next(prev_state.clone());

        state_queue.append(&mut next_states);
        seen.insert(prev_state);
    }
}
