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
    faq_categories (id) {
        id -> Varchar,
        slug -> Varchar,
        display_order -> Unsigned<Integer>,
    }
}

table! {
    faq_category_contents (faq_category_id, locale) {
        faq_category_id -> Varchar,
        locale -> Varchar,
        title -> Varchar,
    }
}

table! {
    faq_category_items (faq_category_id, faq_item_id) {
        faq_category_id -> Varchar,
        faq_item_id -> Varchar,
        display_order -> Unsigned<Integer>,
    }
}

table! {
    faq_item_contents (faq_item_id, locale) {
        faq_item_id -> Varchar,
        locale -> Varchar,
        title -> Varchar,
        body -> Json,
    }
}

table! {
    faq_items (id) {
        id -> Varchar,
        slug -> Varchar,
        is_published -> Bool,
        published_at -> Nullable<Datetime>,
        last_updated_at -> Nullable<Datetime>,
    }
}

table! {
    faq_settings (id) {
        id -> Varchar,
        data -> Json,
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
    inquiry_channels (id) {
        id -> Varchar,
        inquiry_channel_type -> Varchar,
        inquiry_channel_type_id -> Varchar,
        details -> Json,
        is_active -> Bool,
        activated_at -> Datetime,
        deactivated_at -> Nullable<Datetime>,
    }
}

table! {
    inquiry_contact_channels (inquiry_contact_id, inquiry_channel_id) {
        inquiry_contact_id -> Varchar,
        inquiry_channel_id -> Varchar,
        display_order -> Unsigned<Integer>,
    }
}

table! {
    inquiry_contacts (id) {
        id -> Varchar,
        line_user_id -> Nullable<Varchar>,
        details -> Json,
        memo -> Nullable<Varchar>,
        created_at -> Datetime,
    }
}

table! {
    inquiry_messages (id) {
        id -> Varchar,
        inquiry_thread_id -> Varchar,
        reply_inquiry_message_id -> Nullable<Varchar>,
        inquiry_message_type -> Varchar,
        inquiry_message_type_id -> Varchar,
        details -> Json,
        speaker_type -> Varchar,
        inquiry_contact_id -> Nullable<Varchar>,
        agent_id -> Nullable<Varchar>,
        is_canceled -> Bool,
        created_at -> Datetime,
        canceled_at -> Nullable<Datetime>,
    }
}

table! {
    inquiry_settings (id) {
        id -> Varchar,
        data -> Json,
    }
}

table! {
    inquiry_threads (id) {
        id -> Varchar,
        inquiry_channel_id -> Varchar,
        subject -> Varchar,
        inquiry_thread_type -> Varchar,
        inquiry_thread_type_id -> Varchar,
        details -> Json,
        status -> Varchar,
        assigned_agent_id -> Nullable<Varchar>,
        opened_at -> Datetime,
        closed_at -> Nullable<Datetime>,
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
joinable!(faq_category_contents -> faq_categories (faq_category_id));
joinable!(faq_category_items -> faq_categories (faq_category_id));
joinable!(faq_category_items -> faq_items (faq_item_id));
joinable!(faq_item_contents -> faq_items (faq_item_id));
joinable!(group_members -> agents (agent_id));
joinable!(group_members -> groups (group_id));
joinable!(group_members -> roles_for_group (role_id));
joinable!(group_roles -> groups (group_id));
joinable!(group_roles -> roles (role_id));
joinable!(inquiry_contact_channels -> inquiry_channels (inquiry_channel_id));
joinable!(inquiry_contact_channels -> inquiry_contacts (inquiry_contact_id));
joinable!(inquiry_messages -> inquiry_threads (inquiry_thread_id));
joinable!(inquiry_threads -> agents (assigned_agent_id));
joinable!(inquiry_threads -> inquiry_channels (inquiry_channel_id));
joinable!(role_scopes -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    agent_roles,
    agents,
    faq_categories,
    faq_category_contents,
    faq_category_items,
    faq_item_contents,
    faq_items,
    faq_settings,
    files,
    group_members,
    group_roles,
    groups,
    inquiry_channels,
    inquiry_contact_channels,
    inquiry_contacts,
    inquiry_messages,
    inquiry_settings,
    inquiry_threads,
    role_scopes,
    roles,
    roles_for_group,
    workspaces,
);
