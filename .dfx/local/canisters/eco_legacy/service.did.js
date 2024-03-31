export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'addProject' : IDL.Func(
        [IDL.Nat, IDL.Text, IDL.Text],
        [IDL.Bool, IDL.Text],
        [],
      ),
    'getProjects' : IDL.Func([], [IDL.Vec(IDL.Nat), IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
