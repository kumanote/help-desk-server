use database::DbConnection;
use domain::model::{Agent, AgentId, AgentName, Email, HashedPassword};
use domain::repository::AgentRepository;

pub struct AgentRepositoryImpl;

impl AgentRepository for AgentRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, agent: &Agent) -> Result<(), Self::Err> {
        database::adapters::agent::create(tx, agent.into())?;
        Ok(())
    }

    fn update_profile(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        email: Email,
        name: AgentName,
    ) -> Result<(), Self::Err> {
        let updated_count =
            database::adapters::agent::update_profile_by_id(tx, &email, &name, &agent.id)?;
        assert_eq!(updated_count, 1);
        agent.email = email;
        agent.name = name;
        Ok(())
    }

    fn update_hashed_password(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        hashed_password: HashedPassword,
    ) -> Result<(), Self::Err> {
        let updated_count = database::adapters::agent::update_hashed_password_by_id(
            tx,
            &hashed_password,
            &agent.id,
        )?;
        assert_eq!(updated_count, 1);
        agent.hashed_password = hashed_password;
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

    fn get_by_email(
        &self,
        tx: &mut Self::Transaction,
        email: &Email,
    ) -> Result<Option<Agent>, Self::Err> {
        let entity = database::adapters::agent::get_by_email(tx, &email)?;
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
