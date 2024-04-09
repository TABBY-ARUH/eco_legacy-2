use serde::Serialize;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use std::sync::Mutex;
use candid::{CandidType, Deserialize as CandidDeserialize};

#[derive(Serialize, CandidDeserialize, Clone, CandidType)]
pub struct SustainabilityProject {
    id: u32,
    name: String,
    description: String,
}

pub struct EcoLegacy {
    projects: Vec<SustainabilityProject>,
}

impl EcoLegacy {
    pub fn new() -> Self {
        EcoLegacy {
            projects: Vec::new(),
        }
    }

    pub fn get_projects(&self) -> Vec<SustainabilityProject> {
        self.projects.clone()
    }

    pub fn add_project(&mut self, project: SustainabilityProject) {
        self.projects.push(project);
    }
}

lazy_static! {
    static ref ECO_LEGACY_INSTANCE: Mutex<EcoLegacy> = Mutex::new(EcoLegacy::new());
}

#[query]
fn get_projects() -> Vec<SustainabilityProject> {
    let eco_legacy = ECO_LEGACY_INSTANCE.lock().unwrap();
    eco_legacy.get_projects()
}

#[update]
fn add_project(project: SustainabilityProject) {
    let mut eco_legacy = ECO_LEGACY_INSTANCE.lock().unwrap();
    eco_legacy.add_project(project);
}

// #[init]
// fn canister_init() {
//     let mut eco_legacy = ECO_LEGACY_INSTANCE.lock().unwrap();
//     eco_legac
// }
impl EcoLegacy {
    // Existing functions

    pub fn update_project(&mut self, id: u32, updated_project: SustainabilityProject) -> Result<(), String> {
        // Find the project with the given ID
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            // Update the project's details
            *project = updated_project;
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }

    pub fn delete_project(&mut self, id: u32) -> Result<(), String> {
        // Find the index of the project with the given ID
        if let Some(index) = self.projects.iter().position(|p| p.id == id) {
            // Remove the project from the vector
            self.projects.remove(index);
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }
}

#[update]
fn update_project(id: u32, updated_project: SustainabilityProject) -> Result<(), String> {
    let mut eco_legacy = ECO_LEGACY_INSTANCE.lock().unwrap();
    eco_legacy.update_project(id, updated_project)
}

#[update]
fn delete_project(id: u32) -> Result<(), String> {
    let mut eco_legacy = ECO_LEGACY_INSTANCE.lock().unwrap();
    eco_legacy.delete_project(id)
}
