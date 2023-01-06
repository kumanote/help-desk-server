use crate::model::{Group, GroupId, GroupMember};

pub trait GroupRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, group: &Group) -> Result<(), Self::Err>;
    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &GroupId,
    ) -> Result<Option<Group>, Self::Err>;
    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&GroupId>,
    ) -> Result<Vec<Group>, Self::Err>;
    fn create_group_member(
        &self,
        tx: &mut Self::Transaction,
        group_member: &GroupMember,
    ) -> Result<(), Self::Err>;
}
