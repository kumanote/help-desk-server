CREATE TABLE IF NOT EXISTS `workspaces`
(
    `id` varchar(26) NOT NULL,
    `name` varchar(100) NOT NULL,
    `created_at` datetime NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `files`
(
    `id` varchar(26) NOT NULL,
    `stored_filename` varchar(255) NOT NULL,
    `original_filename` varchar(255) NOT NULL,
    `mime_type` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `roles`
(
    `id` varchar(26) NOT NULL,
    `name` varchar(100) NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `role_scopes`
(
    `role_id` varchar(26) NOT NULL,
    `scope` varchar(255) NOT NULL,
    PRIMARY KEY (`role_id`, `scope`) CLUSTERED,
    FOREIGN KEY `fk_role_scope_role_id` (`role_id`) references `roles`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `roles_for_group`
(
    `id` varchar(26) NOT NULL,
    `name` varchar(100) NOT NULL,
    `scope` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `agents`
(
    `id` varchar(26) NOT NULL,
    `email` varchar(255) NOT NULL,
    `hashed_password` varchar(128) NOT NULL,
    `name` varchar(100) NOT NULL,
    `locale` varchar(31) NOT NULL,
    `is_active` boolean NOT NULL,
    UNIQUE `uk_agent_email` (`email`),
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `agent_roles`
(
    `agent_id` varchar(26) NOT NULL,
    `role_id` varchar(26) NOT NULL,
    PRIMARY KEY (`agent_id`, `role_id`) CLUSTERED,
    FOREIGN KEY `fk_agent_role_agent_id` (`agent_id`) references `agents`(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_agent_role_role_id` (`role_id`) references `roles`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `groups`
(
    `id` varchar(26) NOT NULL,
    `name` varchar(100) NOT NULL,
    `description` text NULL,
    `created_at` datetime NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `group_members`
(
    `group_id` varchar(26) NOT NULL,
    `agent_id` varchar(26) NOT NULL,
    `role_id` varchar(26) NOT NULL,
    PRIMARY KEY (`group_id`, `agent_id`) CLUSTERED,
    FOREIGN KEY `fk_group_member_group_id` (`group_id`) references `groups`(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_group_member_agent_id` (`agent_id`) references `agents`(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_group_member_role_id` (`role_id`) references `roles_for_group`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `group_roles`
(
    `group_id` varchar(26) NOT NULL,
    `role_id` varchar(26) NOT NULL,
    PRIMARY KEY (`group_id`, `role_id`) CLUSTERED,
    FOREIGN KEY `fk_group_role_group_id` (`group_id`) references `groups`(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_group_role_role_id` (`role_id`) references `roles`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `faq_settings`
(
    `id` varchar(26) NOT NULL,
    `data` json NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `faq_categories`
(
    `id` varchar(26) NOT NULL,
    `slug` varchar(50) NOT NULL,
    `display_order` int(32) UNSIGNED NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED,
    UNIQUE `uk_faq_category_slug` (`slug`),
    INDEX `idx_faq_category_by_display_order` (`display_order`)
);

CREATE TABLE IF NOT EXISTS `faq_category_contents`
(
    `faq_category_id` varchar(26) NOT NULL,
    `locale` varchar(31) NOT NULL,
    `title` varchar(100) NOT NULL,
    PRIMARY KEY (`faq_category_id`, `locale`) CLUSTERED,
    FOREIGN KEY `fk_faq_category_content_faq_category_id` (`faq_category_id`) references faq_categories(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `faq_items`
(
    `id` varchar(26) NOT NULL,
    `slug` varchar(50) NOT NULL,
    `is_published` boolean NOT NULL,
    `published_at` datetime NULL,
    `last_updated_at` datetime NULL,
    PRIMARY KEY (`id`) CLUSTERED,
    UNIQUE `uk_faq_item_slug` (`slug`),
    INDEX `idx_faq_item_by_last_updated_at` (`last_updated_at`, `is_published`)
);

CREATE TABLE IF NOT EXISTS `faq_item_contents`
(
    `faq_item_id` varchar(26) NOT NULL,
    `locale` varchar(31) NOT NULL,
    `title` varchar(100) NOT NULL,
    `body` json NOT NULL,
    PRIMARY KEY (`faq_item_id`, `locale`) CLUSTERED,
    FOREIGN KEY `fk_faq_item_content_faq_item_id` (`faq_item_id`) references faq_items(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `faq_category_items`
(
    `faq_category_id` varchar(26) NOT NULL,
    `faq_item_id` varchar(26) NOT NULL,
    `display_order` int(32) UNSIGNED NOT NULL,
    PRIMARY KEY (`faq_category_id`, `faq_item_id`) CLUSTERED,
    FOREIGN KEY `fk_faq_category_item_faq_category_id` (`faq_category_id`) references faq_categories(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_faq_category_item_faq_item_id` (`faq_item_id`) references faq_items(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `inquiry_settings`
(
    `id` varchar(26) NOT NULL,
    `data` json NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `inquiry_contacts`
(
    `id` varchar(26) NOT NULL,
    `details` json NOT NULL,
    `memo` varchar(4095) NULL,
    `created_at` datetime NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `inquiry_channels`
(
    `id` varchar(26) NOT NULL,
    `inquiry_channel_type` varchar(31) NOT NULL,
    `details` json NOT NULL,
    `is_active` boolean NOT NULL,
    `activated_at` datetime NOT NULL,
    `deactivated_at` datetime NULL,
    PRIMARY KEY (`id`) CLUSTERED
);

CREATE TABLE IF NOT EXISTS `inquiry_contact_channels`
(
    `inquiry_contact_id` varchar(26) NOT NULL,
    `inquiry_channel_id` varchar(26) NOT NULL,
    `display_order` int(32) UNSIGNED NOT NULL,
    PRIMARY KEY (`inquiry_contact_id`, `inquiry_channel_id`) CLUSTERED,
    FOREIGN KEY `fk_inquiry_contact_channel_inquiry_contact_id` (`inquiry_contact_id`) references inquiry_contacts(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_inquiry_contact_channel_inquiry_channel_id` (`inquiry_channel_id`) references inquiry_channels(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `inquiry_threads`
(
    `id` varchar(26) NOT NULL,
    `inquiry_channel_id` varchar(26) NOT NULL,
    `subject` varchar(400) NOT NULL,
    `inquiry_thread_type` varchar(31) NOT NULL,
    `details` json NOT NULL,
    `status` varchar(31) NOT NULL,
    `assigned_agent_id` varchar(26) NULL,
    `opened_at` datetime NOT NULL,
    `closed_at` datetime NULL,
    PRIMARY KEY (`id`) CLUSTERED,
    FOREIGN KEY `fk_inquiry_thread_inquiry_channel_id` (`inquiry_channel_id`) references inquiry_channels(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_inquiry_thread_assigned_agent_id` (`assigned_agent_id`) references agents(`id`) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS `inquiry_messages`
(
    `id` varchar(26) NOT NULL,
    `inquiry_thread_id` varchar(26) NOT NULL,
    `reply_inquiry_message_id` varchar(26) NULL,
    `inquiry_message_type` varchar(31) NOT NULL,
    `details` json NOT NULL,
    `speaker_type` varchar(31) NOT NULL,
    `inquiry_contact_id` varchar(26) NULL,
    `agent_id` varchar(26) NULL,
    `created_at` datetime NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED,
    FOREIGN KEY `fk_inquiry_message_inquiry_thread_id` (`inquiry_thread_id`) references inquiry_threads(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_inquiry_message_reply_inquiry_message_id` (`reply_inquiry_message_id`) references inquiry_messages(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_inquiry_message_inquiry_contact_id` (`inquiry_contact_id`) references agents(`id`) ON DELETE CASCADE,
    FOREIGN KEY `fk_inquiry_message_agent_id` (`agent_id`) references agents(`id`) ON DELETE CASCADE
);
