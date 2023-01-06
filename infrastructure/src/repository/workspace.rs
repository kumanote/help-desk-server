use database::DbConnection;
use domain::model::Workspace;
use domain::repository::WorkspaceRepository;

pub struct WorkspaceRepositoryImpl {}

impl WorkspaceRepository for WorkspaceRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, workspace: &Workspace) -> Result<(), Self::Err> {
        database::adapters::workspace::create(tx, workspace.into())?;
        Ok(())
    }

    fn update(&self, tx: &mut Self::Transaction, workspace: &Workspace) -> Result<(), Self::Err> {
        database::adapters::workspace::update(tx, &workspace.name, &workspace.id)?;
        Ok(())
    }

    fn get(&self, tx: &mut Self::Transaction) -> Result<Option<Workspace>, Self::Err> {
        let entity = database::adapters::workspace::get(tx)?;
        Ok(entity.map(Into::into))
    }
}
