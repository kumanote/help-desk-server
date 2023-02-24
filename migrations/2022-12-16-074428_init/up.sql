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
    UNIQUE `uk_agent_email` (email),
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
