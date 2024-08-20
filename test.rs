// Example unit test for adding a project
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
}


    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_project() {
        // Setup: Mock data and prepare test environment
        let mut eco_legacy = EcoLegacy::new();
        let mock_project = json!({
            "id": 1,
            "name": "Test Project",
            "description": "This is a test project."
        });

        // Test: Add a project and assert the result
        eco_legacy.add_project(mock_project).await;
        let projects = eco_legacy.get_projects().await;
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");
    }

od lib; // Ensure module name matches your project structure
mod api;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = api::api_routes();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}