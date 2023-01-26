use database::DbConnection;
use domain::model::{RoleForGroup, RoleForGroupId};
use domain::repository::RoleForGroupRepository;

pub struct RoleForGroupRepositoryImpl;

impl RoleForGroupRepository for RoleForGroupRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, role: &RoleForGroup) -> Result<(), Self::Err> {
        database::adapters::role_for_group::create(tx, role.into())?;
        Ok(())
    }

    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &RoleForGroupId,
    ) -> Result<Option<RoleForGroup>, Self::Err> {
        let entity = database::adapters::role_for_group::get_by_id(tx, &id)?;
        Ok(entity.map(Into::into))
    }
}
