use database::DbConnection;
use domain::model::{Agent, AgentId};
use domain::repository::AgentRepository;

pub struct AgentRepositoryImpl {}

impl AgentRepository for AgentRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, agent: &Agent) -> Result<(), Self::Err> {
        database::adapters::agent::create(tx, agent.into())?;
        Ok(())
    }

    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &AgentId,
    ) -> Result<Option<Agent>, Self::Err> {
        let entity = database::adapters::agent::get_by_id(tx, &id)?;
        Ok(entity.map(Into::into))
    }

    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&AgentId>,
    ) -> Result<Vec<Agent>, Self::Err> {
        let ids = ids.iter().map(AsRef::as_ref).collect();
        let entities = database::adapters::agent::get_list_by_ids(tx, &ids)?;
        Ok(entities.into_iter().map(Into::into).collect())
    }
}
