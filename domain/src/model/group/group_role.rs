use crate::model::{GroupId, RoleId};

#[derive(Debug, Clone)]
pub struct GroupRole {
    pub group_id: GroupId,
    pub role_id: RoleId,
}
