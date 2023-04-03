use crate::{config, Error, Result};
use anyhow::anyhow;
use domain::use_case::{
    DeleteFaqItemForSearchUseCase, DeleteFaqItemForSearchUseCaseImpl,
    DeletePublicFaqItemForSearchUseCase, DeletePublicFaqItemForSearchUseCaseImpl,
    UpsertFaqItemForSearchUseCase, UpsertFaqItemForSearchUseCaseImpl,
    UpsertPublicFaqItemForSearchUseCase, UpsertPublicFaqItemForSearchUseCaseImpl,
};
use infrastructure::{FaqSearchRepositoryExecutor, PublicFaqSearchRepositoryExecutor};
use queue::entities::Search;
use search::SearchClient;
use tokio::signal::unix::{signal, SignalKind};

pub struct SearchJobExecutor;

impl SearchJobExecutor {
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn start(self) -> Result<()> {
        println!("start watching search engine back ground tasks...");
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut sigterm = signal(SignalKind::terminate())?;
        let app_config = config::app_config();
        let queue_connection = queue::establish_connection(&app_config.queue.url)?;
        let consumer = queue::consumers::search::subscribe_search_engine_tasks(queue_connection)?;
        let search_client = SearchClient::new(
            &app_config.search.meilisearch_host,
            &app_config.search.meilisearch_api_key,
        );
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
                                let params: Search = serde_json::from_slice(&msg.data).map_err(|cause| Error::SystemError {
                                    cause: anyhow!(cause)
                                })?;
                                if let Err(err) = handle_incoming_message(search_client.clone(), params) {
                                    eprintln!("{:?}", err);
                                }
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

fn handle_incoming_message(search_client: SearchClient, params: Search) -> Result<()> {
    match params {
        Search::UpsertFaqItem(logic_input) => {
            let faq_search_repository = FaqSearchRepositoryExecutor::new(search_client);
            let use_case = UpsertFaqItemForSearchUseCaseImpl::new(faq_search_repository);
            use_case.execute(logic_input).map_err(Into::into)
        },
        Search::DeleteFaqItem(logic_input) => {
            let faq_search_repository = FaqSearchRepositoryExecutor::new(search_client);
            let use_case = DeleteFaqItemForSearchUseCaseImpl::new(faq_search_repository);
            use_case.execute(logic_input).map_err(Into::into)
        },
        Search::UpsertPublicFaqItem(logic_input) => {
            let public_faq_search_repository =
                PublicFaqSearchRepositoryExecutor::new(search_client);
            let use_case =
                UpsertPublicFaqItemForSearchUseCaseImpl::new(public_faq_search_repository);
            use_case.execute(logic_input).map_err(Into::into)
        },
        Search::DeletePublicFaqItem(logic_input) => {
            let public_faq_search_repository =
                PublicFaqSearchRepositoryExecutor::new(search_client);
            let use_case =
                DeletePublicFaqItemForSearchUseCaseImpl::new(public_faq_search_repository);
            use_case.execute(logic_input).map_err(Into::into)
        },
    }
}
