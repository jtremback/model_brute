use std::collections::HashSet;

fn main() {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    phrase: String,
    iterations: u64,
}

struct StateTreeNode {
    state: State,
    parent: Option<usize>,
}

struct StateQueueNode {
    state: State,
    tree_index: usize,
}

fn get_stack_trace(state_tree: &Vec<StateTreeNode>, mut index: Option<usize>) {
    let mut trace = vec![];

    while let Some(i) = index {
        trace.push(state_tree[i].state.clone());
        index = state_tree[i].parent;
    }

    return trace.reverse();
}

fn check(init: State, next: fn(&State) -> Vec<State>, invariant: fn(&State) -> bool) {
    let mut state_tree = vec![StateTreeNode {
        state: init.clone(),
        parent: None,
    }];

    let mut state_queue = vec![StateQueueNode {
        state: init.clone(),
        tree_index: 0,
    }];

    let mut seen: HashSet<State> = HashSet::new();

    let mut i = 0;

    while state_queue.len() > 0 {
        i += 1;

        let StateQueueNode { state, tree_index } = state_queue.pop().unwrap();

        // If we have seen a state before, we skip it. next should be completely deterministic depending on previous state
        // so there is no point in running.
        if !seen.contains(&state) {
            if !invariant(&state) {
                let trace = get_stack_trace(&state_tree, Some(tree_index));

                panic!("{:#?}", trace)
            }

            for state in next(&state) {
                state_tree.push(StateTreeNode {
                    state: state.clone(),
                    parent: Some(tree_index),
                });
                // `stateTree.length - 1` is the index of the item that was just added
                state_queue.push(StateQueueNode {
                    state,
                    tree_index: state_tree.len() - 1,
                });
            }
            seen.insert(state);
        }
    }
}
