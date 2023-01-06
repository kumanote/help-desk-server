use crate::model::{RoleForGroup, RoleForGroupId};

pub trait RoleForGroupRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, role: &RoleForGroup) -> Result<(), Self::Err>;
    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &RoleForGroupId,
    ) -> Result<Option<RoleForGroup>, Self::Err>;
}
