use database::DbConnection;
use domain::model::{Group, GroupId, GroupMember};
use domain::repository::GroupRepository;

pub struct GroupRepositoryImpl {}

impl GroupRepository for GroupRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, group: &Group) -> Result<(), Self::Err> {
        database::adapters::group::create(tx, group.into())?;
        Ok(())
    }

    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &GroupId,
    ) -> Result<Option<Group>, Self::Err> {
        let entity = database::adapters::group::get_by_id(tx, &id)?;
        Ok(entity.map(Into::into))
    }

    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&GroupId>,
    ) -> Result<Vec<Group>, Self::Err> {
        let ids = ids.iter().map(AsRef::as_ref).collect();
        let entities = database::adapters::group::get_list_by_ids(tx, &ids)?;
        Ok(entities.into_iter().map(Into::into).collect())
    }

    fn create_group_member(
        &self,
        tx: &mut Self::Transaction,
        group_member: &GroupMember,
    ) -> Result<(), Self::Err> {
        database::adapters::group_member::create(tx, group_member.into())?;
        Ok(())
    }
}
