use warp::test::request;
use warp::Filter;

#[tokio::test]
async fn test_get_all_projects() {
    let api = crate::api::api_routes(); // Ensure you import the correct module path

    let response = request()
        .method("GET")
        .path("/api/projects")
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    // Further assertions depending on your expected response
}

// Write similar tests for other handlers
