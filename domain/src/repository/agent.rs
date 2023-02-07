use crate::model::{Agent, AgentId, Email};

pub trait AgentRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, agent: &Agent) -> Result<(), Self::Err>;
    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &AgentId,
    ) -> Result<Option<Agent>, Self::Err>;
    fn get_by_email(
        &self,
        tx: &mut Self::Transaction,
        email: &Email,
    ) -> Result<Option<Agent>, Self::Err>;
    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&AgentId>,
    ) -> Result<Vec<Agent>, Self::Err>;
}
