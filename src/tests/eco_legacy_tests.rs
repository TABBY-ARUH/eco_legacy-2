// Import necessary modules
use super::*;

// Define a test module
#[cfg(test)]
mod tests {
    // Import necessary items from the parent module
    use super::*;

    // Define a test function for adding a project
    #[test]
    fn test_add_project() {
        // Create a new EcoLegacy instance
        let mut eco_legacy = EcoLegacy::new();

        // Define a sample project
        let project = SustainabilityProject {
            id: 1,
            name: "Sample Project".to_string(),
            description: "This is a sample project for testing.".to_string(),
        };

        // Add the project to the EcoLegacy instance
        eco_legacy.add_project(project.clone());

        // Retrieve the projects from the EcoLegacy instance
        let projects = eco_legacy.get_projects();

        // Check if the added project exists in the retrieved projects
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0], project);
    }

    // Define a test function for getting a project
    #[test]
    fn test_get_project() {
        // Create a new EcoLegacy instance
        let mut eco_legacy = EcoLegacy::new();

        // Define a sample project
        let project = SustainabilityProject {
            id: 1,
            name: "Sample Project".to_string(),
            description: "This is a sample project for testing.".to_string(),
        };

        // Add the project to the EcoLegacy instance
        eco_legacy.add_project(project.clone());

        // Get the project with the specified ID
        let retrieved_project = eco_legacy.get_project(1).unwrap();

        // Check if the retrieved project matches the original project
        assert_eq!(retrieved_project, project);
    }

    // Define a test function for updating a project
    #[test]
    fn test_update_project() {
        // Create a new EcoLegacy instance
        let mut eco_legacy = EcoLegacy::new();

        // Define a sample project
        let project = SustainabilityProject {
            id: 1,
            name: "Sample Project".to_string(),
            description: "This is a sample project for testing.".to_string(),
        };

        // Add the project to the EcoLegacy instance
        eco_legacy.add_project(project.clone());

        // Define an updated version of the sample project
        let updated_project = SustainabilityProject {
            id: 1,
            name: "Updated Project".to_string(),
            description: "This is an updated version of the sample project.".to_string(),
        };

        // Update the project in the EcoLegacy instance
        eco_legacy.update_project(1, updated_project.clone()).unwrap();

        // Retrieve the projects from the EcoLegacy instance
        let projects = eco_legacy.get_projects();

        // Check if the updated project exists in the retrieved projects
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0], updated_project);
    }

    // Define a test function for deleting a project
    #[test]
    fn test_delete_project() {
        // Create a new EcoLegacy instance
        let mut eco_legacy = EcoLegacy::new();

        // Define a sample project
        let project = SustainabilityProject {
            id: 1,
            name: "Sample Project".to_string(),
            description: "This is a sample project for testing.".to_string(),
        };

        // Add the project to the EcoLegacy instance
        eco_legacy.add_project(project.clone());

        // Delete the project from the EcoLegacy instance
        eco_legacy.delete_project(1).unwrap();

        // Retrieve the projects from the EcoLegacy instance
        let projects = eco_legacy.get_projects();

        // Check if the project has been deleted
        assert_eq!(projects.len(), 0);
    }
}

