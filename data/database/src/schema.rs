table! {
    agent_roles (agent_id, role_id) {
        agent_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    agents (id) {
        id -> Varchar,
        email -> Varchar,
        hashed_password -> Varchar,
        name -> Varchar,
        locale -> Varchar,
        is_active -> Bool,
    }
}

table! {
    files (id) {
        id -> Varchar,
        stored_filename -> Varchar,
        original_filename -> Varchar,
        mime_type -> Varchar,
    }
}

table! {
    group_members (group_id, agent_id) {
        group_id -> Varchar,
        agent_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    group_roles (group_id, role_id) {
        group_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    groups (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Datetime,
    }
}

table! {
    role_scopes (role_id, scope) {
        role_id -> Varchar,
        scope -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    roles_for_group (id) {
        id -> Varchar,
        name -> Varchar,
        scope -> Varchar,
    }
}

table! {
    workspaces (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Datetime,
    }
}

joinable!(agent_roles -> agents (agent_id));
joinable!(agent_roles -> roles (role_id));
joinable!(group_members -> agents (agent_id));
joinable!(group_members -> groups (group_id));
joinable!(group_members -> roles_for_group (role_id));
joinable!(group_roles -> groups (group_id));
joinable!(group_roles -> roles (role_id));
joinable!(role_scopes -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    agent_roles,
    agents,
    files,
    group_members,
    group_roles,
    groups,
    role_scopes,
    roles,
    roles_for_group,
    workspaces,
);
