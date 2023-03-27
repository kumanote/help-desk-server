use line_webhook_config::*;

#[test]
fn test_build_app_config() {
    let config_file_path = Some("./tests/test_app.toml");
    let args = AppArgs::default();
    let app_config = AppConfig::build(config_file_path, args).unwrap();
    let expected = AppConfig {
        line_webhook: LineWebhookConfig {
            channel_secret: "<channel_secret_issued_by_line_develoers_console>".to_owned(),
            channel_access_token: "<channel_access_token_issued_by_line_develoers_console>"
                .to_owned(),
            bind_address: "0.0.0.0:8001".to_owned(),
        },
        queue: QueueConfig {
            url: "127.0.0.1".to_owned(),
            max_connection_pool_size: 2,
        },
    };
    assert_eq!(app_config, expected);
}
