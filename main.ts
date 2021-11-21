interface State {
  phrase: string;
  iterations: number;
}

type MaybeState = State | null;

type Branch = (state: State) => Array<State | null>;

function branch(states: Array<State>, branches: Array<Branch>): Array<State> {
  const newStates: Array<State | null> = [];
  for (const state of states) {
    for (const branch of branches) {
      // Append states produced by the branch to
      // newStates
      newStates.push(
        ...branch(
          // Copy state so that we don't mess it up
          // for future runs
          JSON.parse(JSON.stringify(state))
        )
      );
    }
  }

  // Filter out null states
  const outStates: Array<State> = [];
  for (const maybeState of newStates) {
    if (maybeState) {
      outStates.push(maybeState);
    }
  }

  return outStates;
}

function next(state: State) {
  if (state.iterations >= 5) {
    return [null];
  }

  return branch(
    [state],
    [
      (state) => {
        state.iterations += 1;

        return branch(
          [state],
          [
            (state) => {
              state.phrase += "A";
              return [state];
            },
            (state) => {
              state.phrase += "B";
              return [state];
            },
          ]
        );
      },
    ]
  );
}

function check(
  init: State,
  next: (state: State) => Array<State | null>,
  invariant: (state: State) => boolean
) {
  let stateQueue: Array<State | null> = [init];
  const seen: Set<State> = new Set();

  let i = 0;

  while (stateQueue.length > 0) {
    i += 1;
    // this should be guaranteed to yield a State, given the test
    const prevState = stateQueue.pop();

    console.log(prevState);

    // If we have seen a state before, we skip it. fn next is completely deterministic depending on previous state
    // so there is no point in running.
    if (prevState && !seen.has(prevState)) {
      if (!invariant(prevState)) {
        throw new Error(
          "invariant broken by state: " + JSON.stringify(prevState)
        );
      }
      const nextStates = next(prevState);
      console.log(nextStates);
      stateQueue = stateQueue.concat(nextStates);
      seen.add(prevState);
      console.log(stateQueue);
    }
  }
}

function invariant(state: State) {
  if (state.phrase === "AAABB") {
    return false;
  }

  return true;
}

check(
  {
    phrase: "",
    iterations: 0,
  },
  next,
  invariant
);

// fn main() {
//   let initial_state = State {
//       iterations: 0,
//       phrase: "".into(),
//   };

//   let mut state_queue = vec![initial_state];

//   let mut seen: HashSet<State> = HashSet::new();

//   let mut i = 0;

//   while !state_queue.is_empty() {
//       i += 1;
//       let prev_state = state_queue.pop().unwrap();
//       // If we have seen a state before, we skip it. fn next is completely deterministic depending on previous state
//       // so there is no point in running.
//       if seen.contains(&prev_state) {
//           continue;
//       }

//       if !invariant(prev_state.clone()) {
//           println!("Invariant violated with state: {:#?}", prev_state);
//           break;
//       }

//       let mut next_states = next(prev_state.clone());

//       state_queue.append(&mut next_states);
//       seen.insert(prev_state);
//   }
// }
