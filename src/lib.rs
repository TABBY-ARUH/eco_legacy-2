use serde::{Deserialize as SerdeDeserialize, Serialize};

// Define SustainabilityProject struct with #[derive] attributes
#[derive(SerdeDeserialize, Serialize, Clone)]
pub struct Sustainability Project {
    id: u32,
    name: String,
    description: String,
}

// Define EcoLegacy struct to hold the projects
pub struct EcoLegacy {
    projects: Vec<Sustainability Project>,
}

impl EcoLegacy {
    pub fn new() -> Self {
        // Initialize EcoLegacy with an empty list of projects
        EcoLegacy {
            projects: Vec::new(),
        }
    }

    pub fn get_projects(&self) -> Vec<Sustainability Project> {
        // Return a clone of the projects list
        self.projects.clone()
    }

    pub fn add_project(&mut self, project: Sustainability Project) {
        // Add a project to the list
        self.projects.push(project);
    }
}

// Function to get projects, marked as canister_query
#[export_name = "canister_query get_projects"]
fn get_projects() -> Vec<Sustainability Project> {
    // Create a new instance of EcoLegacy
    let eco_legacy = EcoLegacy::new();
    // Return the projects
    eco_legacy.get_projects()
}

// Function to add a project, marked as canister_update
#[export_name = "canister_update add_project"]
fn add_project(project: Sustainability Project) {
    // Create a mutable reference to the shared EcoLegacy instance
    let mut eco_legacy = ECO_LEGACY_INSTANCE.lock().expect("Failed to acquire lock");
    // Add the project
    eco_legacy.add_project(project);
}

// Create a shared instance of EcoLegacy using lazy_static
lazy_static::lazy_static! {
    static ref ECO_LEGACY_INSTANCE: std::sync::Mutex<EcoLegacy> = std::sync::Mutex::new(EcoLegacy::new());
}

// Export the Candid interface
#[export_name = "canister_init"]
fn canister_init() {
    ic_cdk::export_service!();
}