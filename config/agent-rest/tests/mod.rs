use agent_rest_config::*;

#[test]
fn test_build_app_config() {
    let config_file_path = Some("./tests/test_app.toml");
    let args = AppArgs::default();
    let app_config = AppConfig::build(config_file_path, args).unwrap();
    let expected = AppConfig {
        agent_rest: AgentRestConfig {
            secret_key: "bcbe26847a6e70f70e24625b49a17988c91201c5c43cdc76217283042a56376d"
                .to_owned(),
            bind_address: "0.0.0.0:8000".to_owned(),
            allowed_origins: vec![
                "http://localhost:3000".to_owned(),
                "http://192.168.10.201:3000".to_owned(),
            ],
        },
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
            meilisearch_host: "127.0.0.1".to_owned(),
            meilisearch_api_key: "01d5e2eaaaee7a36104ff786f5621b3f21a41ddd628ca12f6fc0b157cfc109ff"
                .to_owned(),
        },
    };
    assert_eq!(app_config, expected);
}
