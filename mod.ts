type Branch<S> = (state: S) => Array<S | null>;

export function either<S>(
  states: Array<S>,
  branches: Array<Branch<S>>
): Array<S> {
  const newStates: Array<S | null> = [];
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
  const outStates: Array<S> = [];
  for (const maybeState of newStates) {
    if (maybeState) {
      outStates.push(maybeState);
    }
  }

  return outStates;
}

export class Premise<S> {
  states: Array<S>;

  constructor(state: S) {
    this.states = [state];
  }

  either(branches: Array<Branch<S>>) {
    this.states = either(this.states, branches);
    return this;
  }

  then(branch: Branch<S>) {
    this.states = either(this.states, [branch]);
    return this;
  }

  maybe(branch: Branch<S>) {
    this.states = either(this.states, [branch, (state) => [state]]);
    return this;
  }
}

type StateTree<S> = Array<{ state: S; parent: number }>;
type StateQueue<S> = Array<{ state: S; index: number }>;

function getStackTrace<S>(stateTree: StateTree<S>, index: number) {
  const trace = [];

  while (index != -1) {
    trace.unshift(stateTree[index].state);
    index = stateTree[index].parent;
  }

  return trace;
}

export function check<S>(
  init: S,
  next: (state: S) => Array<S>,
  invariant: (state: S) => boolean
) {
  const stateTree: StateTree<S> = [{ state: init, parent: -1 }];
  // we initialize the state queue with the init state at an index of 0
  // TODO: It might be possible to use the stateTree array as the stateQueue or seen
  // and save on some memory
  const stateQueue: StateQueue<S> = [{ state: init, index: 0 }];
  const seen: Set<S> = new Set();

  let i = 0;

  while (stateQueue.length > 0) {
    i += 1;
    const { state, index } = stateQueue.pop()!;

    // If we have seen a state before, we skip it. next should be completely deterministic depending on previous state
    // so there is no point in running.
    if (state && !seen.has(state)) {
      console.log(state);
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

  console.log("number of states: ", i);
}
