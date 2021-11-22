interface State {
  phrase: string;
  iterations: number;
}

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

// either[
//   {
//     state.phrase += "A";
//   },
//   {
//     state.phrase += "B";
//   }
// ]

// function next(state: State) {
//   next {
//     if (state.iterations >= 5) {
//       stop;
//     }
//     state.iterations += 1;

//     either {
//       state.phrase += "A";
//     } or {
//       state.phrase += "B";
//     }
//   }
// }

function next(state: State) {
  return branch(
    [state],
    [
      (state) => {
        if (state.iterations >= 5) {
          return [null];
        }
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

type StateTree = Array<{ state: State; parent: number }>;
type StateQueue = Array<{ state: State; index: number }>;

function getStackTrace(stateTree: StateTree, index: number) {
  const trace = [];

  while (index != -1) {
    trace.unshift(stateTree[index].state);
    index = stateTree[index].parent;
  }

  return trace;
}

function check(
  init: State,
  next: (state: State) => Array<State>,
  invariant: (state: State) => boolean
) {
  let stateTree: StateTree = [{ state: init, parent: -1 }];
  // we initialize the state queue with the init state at an index of 0
  // TODO: It might be possible to use the stateTree array as the stateQueue
  // and save on some memory
  let stateQueue: StateQueue = [{ state: init, index: 0 }];
  const seen: Set<State> = new Set();

  let i = 0;

  while (stateQueue.length > 0) {
    i += 1;
    const { state, index } = stateQueue.pop()!;

    // If we have seen a state before, we skip it. next should be completely deterministic depending on previous state
    // so there is no point in running.
    if (state && !seen.has(state)) {
      if (!invariant(state)) {
        const trace = getStackTrace(stateTree, index);
        throw new Error(
          "invariant broken by state: " + JSON.stringify(trace, null, "  ")
        );
      }
      const nextStates = next(state);

      nextStates.forEach((state) => {
        stateTree.push({ state, parent: index });
        // `stateTree.length - 1` is the index of the item that was just added
        stateQueue.push({ state, index: stateTree.length - 1 });
      });
      seen.add(state);
    }
  }
}

function invariant(state: State) {
  return state.phrase !== "ABABA";
}

check(
  {
    phrase: "",
    iterations: 0,
  },
  next,
  invariant
);
