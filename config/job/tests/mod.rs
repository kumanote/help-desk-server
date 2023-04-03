use job_config::*;

#[test]
fn test_build_app_config() {
    let config_file_path = Some("./tests/test_app.toml");
    let args = AppArgs::default();
    let app_config = AppConfig::build(config_file_path, args).unwrap();
    let expected = AppConfig {
        database: DatabaseConfig {
            url: "mysql://help_desk:password@127.0.0.1:4000/help_desk?charset=utf8mb4".to_owned(),
            max_connection_pool_size: 2,
        },
        cache: CacheConfig {
            url: "redis://127.0.0.1:6379/0".to_owned(),
            max_connection_pool_size: 2,
        },
        queue: QueueConfig {
            url: "127.0.0.1".to_owned(),
            max_connection_pool_size: 2,
        },
        search: SearchConfig {
            meilisearch_host: "http://localhost:7700".to_owned(),
            meilisearch_api_key: "01d5e2eaaaee7a36104ff786f5621b3f21a41ddd628ca12f6fc0b157cfc109ff"
                .to_owned(),
        },
        line: LineConfig {
            channel_access_token: Some(
                "<channel_access_token_issued_by_line_develoers_console>".to_owned(),
            ),
        },
    };
    assert_eq!(app_config, expected);
}
