import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface SustainabilityProject {
  'id' : number,
  'name' : string,
  'description' : string,
}
export interface _SERVICE {
  'add_project' : ActorMethod<[SustainabilityProject], undefined>,
  'get_projects' : ActorMethod<[], Array<SustainabilityProject>>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
