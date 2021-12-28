/* CREATE DATABASE IF NOT EXISTS `stigmarks`; */

/* USE `stigmarks`; */

CREATE TABLE IF NOT EXISTS `users` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(256) NOT NULL,
    `email` varchar(256) NOT NULL UNIQUE,
    `role` int(11) NOT NULL,
    `hash` varchar(128) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT NOW(),
    `validated_at` datetime DEFAULT NULL,
    `disabled_at` datetime DEFAULT NULL,
    `disabled_by` int(11) DEFAULT NULL,
    `is_private` int(1) NOT NULL DEFAULT 0,
    `is_anonymous` int(1) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    CONSTRAINT `fk_users_disabled_by` FOREIGN KEY (`disabled_by`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `followers` (
    `stigmer_id` int(11) NOT NULL,
    `follower_id` int(11) NOT NULL,
    `authorized_at` datetime DEFAULT NULL,
    `forbidden_at` datetime DEFAULT NULL,
    UNIQUE KEY `stigmer_follower` (`stigmer_id`, `follower_id`),
    CONSTRAINT `fk_followers_stigmer_id` FOREIGN KEY (`stigmer_id`) REFERENCES `users` (`id`),
    CONSTRAINT `fk_followers_follower_id` FOREIGN KEY (`follower_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `keywords` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword` varchar(256) NOT NULL,
    `ref_count` int(11) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    UNIQUE KEY `keyword` (`keyword`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `urls` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `url` varchar(2048) NOT NULL,
    `ref_count` int(11) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    KEY `url` (`url`) USING HASH
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `collections` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `created_by` int(11) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT NOW(),
    `hidden_at` datetime DEFAULT NULL,
    `hidden_by` int(11) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_collectionuser` (`created_by`),
    CONSTRAINT `fk_collections_created_by` FOREIGN KEY (`created_by`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `keyword_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `keyword_id` (`keyword_id`),
    CONSTRAINT `fk_keyword_lists_collections_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_keyword_lists_keywords_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`),
    CONSTRAINT `fk_keyword_lists_primary_key` PRIMARY KEY (`collection_id`, `keyword_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `url_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `url_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `url_id` (`url_id`),
    CONSTRAINT `fk_url_lists_collection_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_url_lists_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`),
    CONSTRAINT `fk_url_lists_primary_key` PRIMARY KEY (`collection_id`, `url_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `url_scoring` (
    `url_id` int(11) NOT NULL,
    `keyword_id` int(11) NOT NULL,
    `pscore` double NOT NULL,
    `vscore` double NOT NULL,
    PRIMARY KEY (`url_id`, `keyword_id`),
    CONSTRAINT `fk_url_scoring_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`),
    CONSTRAINT `fk_url_scoring_keyword_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `stigmee_events` (
    `event_id` int(11) NOT NULL AUTO_INCREMENT,
    `event_date` timestamp NOT NULL DEFAULT NOW(),
    `event_type` int(11),
    `event_desc` varchar(256) NOT NULL,
    `event_arg1` int(11),
    `event_arg2` int(11),
    `event_arg3` int(11),
    `event_arg4` varchar(4000),
    PRIMARY KEY (`event_id`, `event_date`)
)
PARTITION BY RANGE (UNIX_TIMESTAMP(`event_date`))
(
	PARTITION p0 VALUES LESS THAN (UNIX_TIMESTAMP('2022-01-01 00:00:00')),
	PARTITION p1 VALUES LESS THAN (UNIX_TIMESTAMP('2023-01-01 00:00:00')),
	PARTITION p2 VALUES LESS THAN (UNIX_TIMESTAMP('2024-01-01 00:00:00')),
	PARTITION p3 VALUES LESS THAN (UNIX_TIMESTAMP('2025-01-01 00:00:00')),
	PARTITION pmax VALUES LESS THAN (MAXVALUE)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
