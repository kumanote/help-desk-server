use domain::model::InquiryIncomingEvent;
use domain::repository::InquiryJobRepository;
use queue::QueueConnectionPool;

pub struct InquiryJobRepositoryImpl {
    queue_connection_pool: QueueConnectionPool,
}

impl InquiryJobRepositoryImpl {
    pub fn new(queue_connection_pool: QueueConnectionPool) -> Self {
        Self {
            queue_connection_pool,
        }
    }
}

impl InquiryJobRepository for InquiryJobRepositoryImpl {
    type Err = domain::Error;

    fn register(&self, inquiry_incoming_event: &InquiryIncomingEvent) -> Result<(), Self::Err> {
        let msg: queue::entities::InquiryIncomingEvent = inquiry_incoming_event.clone().into();
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::inquiry_incoming_event::publish_inquiry_incoming_event_task(
            &mut queue_connection,
            msg,
        )?;
        Ok(())
    }
}
