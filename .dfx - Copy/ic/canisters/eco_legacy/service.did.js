export const idlFactory = ({ IDL }) => {
  const SustainabilityProject = IDL.Record({
    'id' : IDL.Nat32,
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  return IDL.Service({
    'add_project' : IDL.Func([SustainabilityProject], [], []),
    'get_projects' : IDL.Func([], [IDL.Vec(SustainabilityProject)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
