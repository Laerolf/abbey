pub mod get_all_skills {
    use axum::http::StatusCode;
    use domain::features::skill::dto::SkillDto;

    use serial_test::serial;

    use crate::shared::{TestApp, utils::read_body_as_value};

    #[tokio::test]
    #[serial]
    pub async fn test_get_all_skills_returns_200() {
        // Given
        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context.game_service.create_game(db_transaction).await
            })
            .await
            .expect("Failed to create the test game.");

        // When
        let response = app.get("/api/catalog/skills").send().await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);

        let actual_dto: Vec<SkillDto> = read_body_as_value(response).await;

        assert!(!actual_dto.is_empty());
    }
}

pub mod get_all_resources {
    use axum::http::StatusCode;
    use domain::features::output::dto::ResourceDto;

    use serial_test::serial;

    use crate::shared::{TestApp, utils::read_body_as_value};

    #[tokio::test]
    #[serial]
    pub async fn test_get_all_resources_returns_200() {
        // Given
        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context.game_service.create_game(db_transaction).await
            })
            .await
            .expect("Failed to create the test game.");

        // When
        let response = app.get("/api/catalog/resources").send().await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);

        let actual_dto: Vec<ResourceDto> = read_body_as_value(response).await;

        assert!(!actual_dto.is_empty());
    }
}
