/* CREATE DATABASE IF NOT EXISTS `stigmarks`; */

/* USE `stigmarks`; */

CREATE TABLE IF NOT EXISTS `users` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(256) NOT NULL,
    `email` varchar(256) NOT NULL UNIQUE,
    `role` int(11) NOT NULL,
    `hash` varchar(128) NOT NULL,
    `creation_date` datetime NOT NULL DEFAULT NOW(),
    `disabled_at` datetime DEFAULT NULL,
    `disabled_by` int(11) DEFAULT NULL,
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `keywords` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword` varchar(256) NOT NULL,
    `ref_count` int(11) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    UNIQUE KEY `keyword` (`keyword`)
);

CREATE TABLE IF NOT EXISTS `urls` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `url` varchar(2048) NOT NULL,
    `ref_count` int(11) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    UNIQUE KEY `url` (`url`) USING HASH
);

CREATE TABLE IF NOT EXISTS `collections` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `user_id` int(11) NOT NULL,
    `creation_date` datetime NOT NULL DEFAULT NOW(),
    `hidden_at` datetime DEFAULT NULL,
    `hidden_by` int(11) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_collectionuser` (`user_id`),
    CONSTRAINT `fk_collections_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE IF NOT EXISTS `keyword_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `keyword_id` (`keyword_id`),
    CONSTRAINT `fk_keyword_lists_collections_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_keyword_lists_keywords_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`),
    CONSTRAINT `fk_keyword_lists_primary_key` PRIMARY KEY (`collection_id`, `keyword_id`)
);

CREATE TABLE IF NOT EXISTS `url_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `url_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `url_id` (`url_id`),
    CONSTRAINT `fk_url_lists_collection_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_url_lists_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`),
    CONSTRAINT `fk_url_lists_primary_key` PRIMARY KEY (`collection_id`, `url_id`)
);

CREATE TABLE IF NOT EXISTS `url_scoring` (
    `url_id` int(11) NOT NULL,
    `keyword_id` int(11) NOT NULL,
    `pscore` double NOT NULL,
    `vscore` double NOT NULL,
    PRIMARY KEY (`url_id`, `keyword_id`),
    CONSTRAINT `fk_url_scoring_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`),
    CONSTRAINT `fk_url_scoring_keyword_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`)
);

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
);
