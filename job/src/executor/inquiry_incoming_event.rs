use crate::{config, Error, Result};
use anyhow::anyhow;
use database::DbConnectionPool;
use domain::use_case::{
    HandleLineFollowEventUseCase, HandleLineFollowEventUseCaseImpl, HandleLineJoinEventUseCase,
    HandleLineJoinEventUseCaseImpl, HandleLineLeaveEventUseCase, HandleLineLeaveEventUseCaseImpl,
    HandleLineMessageEventUseCase, HandleLineMessageEventUseCaseImpl,
    HandleLineUnFollowEventUseCase, HandleLineUnFollowEventUseCaseImpl,
    HandleLineUnsendEventUseCase, HandleLineUnsendEventUseCaseImpl,
};
use infrastructure::{InquiryRepositoryImpl, InquirySearchRepositoryImpl, LineRepositoryImpl};
use line::LineClient;
use queue::entities::InquiryIncomingEvent;
use search::SearchClient;
use tokio::signal::unix::{signal, SignalKind};

pub struct InquiryIncomingEventJobExecutor;

impl InquiryIncomingEventJobExecutor {
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn start(self) -> Result<()> {
        println!("start watching inquiry incoming event back ground tasks...");
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut sigterm = signal(SignalKind::terminate())?;
        let app_config = config::app_config();
        let queue_connection = queue::establish_connection(&app_config.queue.url)?;
        let consumer =
            queue::consumers::inquiry_incoming_event::subscribe_inquiry_incoming_event_tasks(
                queue_connection,
            )?;
        let db_connection_pool = database::new_pool(
            &app_config.database.url,
            app_config.database.max_connection_pool_size,
        )?;
        let search_client = SearchClient::new(
            &app_config.search.meilisearch_host,
            &app_config.search.meilisearch_api_key,
        );
        let line_client = app_config
            .line
            .channel_access_token
            .as_deref()
            .map(LineClient::new);
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
                                let params: InquiryIncomingEvent = serde_json::from_slice(&msg.data).map_err(|cause| Error::SystemError {
                                    cause: anyhow!(cause)
                                })?;
                                if let Err(err) = handle_incoming_message(db_connection_pool.clone(), search_client.clone(), line_client.clone(), params) {
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

fn handle_incoming_message(
    db_connection_pool: DbConnectionPool,
    search_client: SearchClient,
    line_client: Option<LineClient>,
    params: InquiryIncomingEvent,
) -> Result<()> {
    match params {
        InquiryIncomingEvent::Line(event) => {
            match event.r#type {
                line::events::EventType::UnsendEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let use_case = HandleLineUnsendEventUseCaseImpl::new(inquiry_repository);
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::FollowEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let inquiry_search_repository = InquirySearchRepositoryImpl::new(search_client);
                    if line_client.is_none() {
                        return Err(Error::ImproperConfigError {
                            cause: format!("this command requires line.channel_access_token configuration value."),
                        });
                    }
                    let line_repository = LineRepositoryImpl::new(line_client.unwrap());
                    let use_case = HandleLineFollowEventUseCaseImpl::new(
                        inquiry_repository,
                        inquiry_search_repository,
                        line_repository,
                    );
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::UnFollowEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let use_case = HandleLineUnFollowEventUseCaseImpl::new(inquiry_repository);
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::JoinEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let inquiry_search_repository = InquirySearchRepositoryImpl::new(search_client);
                    if line_client.is_none() {
                        return Err(Error::ImproperConfigError {
                            cause: format!("this command requires line.channel_access_token configuration value."),
                        });
                    }
                    let line_repository = LineRepositoryImpl::new(line_client.unwrap());
                    let use_case = HandleLineJoinEventUseCaseImpl::new(
                        inquiry_repository,
                        inquiry_search_repository,
                        line_repository,
                    );
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::LeaveEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let use_case = HandleLineLeaveEventUseCaseImpl::new(inquiry_repository);
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::MemberJoinEvent(_) => unimplemented!(),
                line::events::EventType::MemberLeaveEvent(_) => unimplemented!(),
                line::events::EventType::PostBackEvent(_) => unimplemented!(),
                line::events::EventType::VideoPlayCompleteEvent(_) => unimplemented!(),
                line::events::EventType::BeaconEvent(_) => unimplemented!(),
                line::events::EventType::AccountLinkEvent(_) => unimplemented!(),
                line::events::EventType::ThingsEvent(_) => unimplemented!(),
                line::events::EventType::MessageEvent(logic_input) => {
                    let mut tx = db_connection_pool.get()?;
                    let inquiry_repository = InquiryRepositoryImpl;
                    let inquiry_search_repository = InquirySearchRepositoryImpl::new(search_client);
                    if line_client.is_none() {
                        return Err(Error::ImproperConfigError {
                            cause: format!("this command requires line.channel_access_token configuration value."),
                        });
                    }
                    let line_repository = LineRepositoryImpl::new(line_client.unwrap());
                    let use_case = HandleLineMessageEventUseCaseImpl::new(
                        inquiry_repository,
                        inquiry_search_repository,
                        line_repository,
                    );
                    use_case.execute(&mut tx, logic_input).map_err(Into::into)
                },
                line::events::EventType::Other => unimplemented!(),
            }
        },
    }
}
