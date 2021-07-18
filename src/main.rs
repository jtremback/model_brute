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
    if state.iterations < 6 {
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

fn main() {
    
}
