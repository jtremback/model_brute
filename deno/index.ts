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
