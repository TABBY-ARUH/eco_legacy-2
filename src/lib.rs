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