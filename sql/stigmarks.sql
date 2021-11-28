CREATE DATABASE IF NOT EXISTS `stigmarks`;
USE `stigmarks`;

CREATE TABLE IF NOT EXISTS `users` (
    `id` int(11) NOT NULL,
    `name` varchar(256) DEFAULT NULL,
    `email` varchar(256) DEFAULT NULL,
    `hash` binary(255) DEFAULT NULL,
    `creation_date` datetime DEFAULT NULL,
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `keywords` (
    `id` int(11) NOT NULL,
    `keyword` varchar(256) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `keyword` (`keyword`)
);

CREATE TABLE IF NOT EXISTS `urls` (
    `id` int(11) NOT NULL,
    `url` varchar(2048) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `url` (`url`) USING HASH
);

CREATE TABLE IF NOT EXISTS `collections` (
    `id` int(11) NOT NULL,
    `user_id` int(11) NOT NULL,
    `creation_date` datetime DEFAULT NULL,
    `hidden` tinyint(1) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_collectionuser` (`user_id`),
    CONSTRAINT `fk_collections_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE IF NOT EXISTS `keyword_lists` (
    `collection_id` int(11) NOT NULL,
    `keyword_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `keyword_id` (`keyword_id`),
    CONSTRAINT `fk_keyword_lists_collections_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_keyword_lists_keywords_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`)
);

CREATE TABLE IF NOT EXISTS `url_lists` (
    `collection_id` int(11) NOT NULL,
    `url_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `url_id` (`url_id`),
    CONSTRAINT `fk_url_lists_collection_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_url_lists_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`)
);
