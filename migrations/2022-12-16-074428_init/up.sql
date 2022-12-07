CREATE TABLE IF NOT EXISTS `files`
(
    `id` varchar(26) NOT NULL,
    `stored_filename` varchar(255) NOT NULL,
    `original_filename` varchar(255) NOT NULL,
    `mime_type` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) CLUSTERED
);
