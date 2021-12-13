import { check, either, Premise } from "./mod.ts";

interface State {
  phrase: string;
  iterations: number;
}

// function next(state: State) {
//   if (state.iterations >= 5) {
//     stop;
//   }
//
//   either {
//     state.phrase += "A";
//   } or {
//     state.phrase += "B";
//
//     maybe {
//       state.phrase += "b";
//     }
//   }
//
//   state.iterations += 1;
//
//   either {
//     state.phrase += "C";
//   } or {
//     state.phrase += "D";
//   }
//
//   return state
// }

function next5(state: State) {
  return new Premise(state)
    .then((state) => {
      if (state.iterations >= 5) {
        return [null];
      }
      return [state];
    })
    .either([
      (state) => {
        state.phrase += "A";
        return [state];
      },
      (state) => {
        state.phrase += "B";
        return new Premise(state).maybe((state) => {
          state.phrase += "b";
          return [state];
        }).states;
      },
    ])
    .then((state) => {
      state.iterations += 1;
      return [state];
    })
    .either([
      (state) => {
        state.phrase += "C";
        return [state];
      },
      (state) => {
        state.phrase += "D";
        return [state];
      },
    ]).states;
}

function next4(state: State) {
  return either(
    either(
      either(
        either(
          [state],
          [
            (state) => {
              if (state.iterations >= 5) {
                return [null];
              }
              return [state];
            },
          ]
        ),
        [
          (state) => {
            state.phrase += "A";
            return [state];
          },
          (state) => {
            state.phrase += "B";
            return either(
              [state],
              [
                (state) => {
                  return [state];
                },
                (state) => {
                  state.phrase += "b";
                  return [state];
                },
              ]
            );
          },
        ]
      ),
      [
        (state) => {
          state.iterations += 1;
          return [state];
        },
      ]
    ),
    [
      (state) => {
        state.phrase += "C";
        return [state];
      },
      (state) => {
        state.phrase += "D";
        return [state];
      },
    ]
  );
}

function invariant(state: State) {
  return state.phrase !== "BbCACACAD";
}

check(
  {
    phrase: "",
    iterations: 0,
  },
  next5,
  invariant
);
