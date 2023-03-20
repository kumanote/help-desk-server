use crate::{config, Error, Result};
use anyhow::anyhow;
use tokio::signal::unix::{signal, SignalKind};

pub struct SearchJobExecutor;

impl SearchJobExecutor {
    pub fn new() -> Self {
        SearchJobExecutor
    }

    pub(crate) async fn start(self) -> Result<()> {
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut sigterm = signal(SignalKind::terminate())?;
        let app_config = config::app_config();
        let queue_connection = queue::establish_connection(&app_config.queue.url)?;
        let consumer = queue::consumers::search::subscribe_search_engine_tasks(queue_connection)?;
        loop {
            let consumer = consumer.clone();
            let fetch_next_msg = tokio::task::spawn(async move {
                let mut messages = consumer.fetch(1).unwrap();
                messages.next()
            });
            tokio::select! {
                _ = sigint.recv() => {
                    println!("sigint detected. let's finish the process...");
                    break;
                }
                _ = sigterm.recv() => {
                    println!("sigterm detected. let's finish the process...");
                    break;
                }
                message_result = fetch_next_msg => {
                    match message_result {
                        Ok(message) => {
                            if let Some(msg) = message {
                                let params: queue::entities::Search = serde_json::from_slice(&msg.data).map_err(|cause| Error::SystemError {
                                    cause: anyhow!(cause)
                                })?;
                                println!("{:?}", params);
                                msg.ack()?;
                            }
                        },
                        Err(err) => {
                            eprintln!("{:?}", err);
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
