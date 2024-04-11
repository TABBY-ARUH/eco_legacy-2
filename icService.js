// icService.js
import { createActor } from '@dfinity/agent';
import { idlFactory as ecoLegacyIdl, canisterId as ecoLegacyCanisterId } from './src/eco_legacy.did';

// Create an actor to interact with the Rust contract
const actor = createActor(ecoLegacyIdl, { canisterId: ecoLegacyCanisterId });

// Define functions to interact with the Rust contract
export const addProject = async (projectData) => {
  try {
    await actor.addProject(projectData);
    // Optionally handle success
  } catch (error) {
    // Handle error
  }
};

export const getProjects = async () => {
  try {
    const projects = await actor.getProjects();
    return projects;
  } catch (error) {
    // Handle error
  }
};

