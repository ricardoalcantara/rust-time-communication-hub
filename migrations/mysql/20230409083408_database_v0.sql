CREATE TABLE `user` (
  `id` integer PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `external_id` varchar(255) UNIQUE NOT NULL,
  `created_at` datetime NOT NULL
);

CREATE TABLE `group` (
  `id` integer PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `name` varchar(255) UNIQUE NOT NULL
);

CREATE TABLE `message` (
  `id` integer PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `created_at` datetime NOT NULL,
  `payload` varchar(255) NOT NULL
);

CREATE TABLE `user_message` (
  `user_id` integer NOT NULL,
  `message_id` integer NOT NULL,
  `read` bool NOT NULL
);

CREATE TABLE `user_group` (
  `user_id` integer NOT NULL,
  `group_id` integer NOT NULL
);

ALTER TABLE `user_message` ADD FOREIGN KEY (`user_id`) REFERENCES `user` (`id`);

ALTER TABLE `user_message` ADD FOREIGN KEY (`message_id`) REFERENCES `message` (`id`);

ALTER TABLE `user_group` ADD FOREIGN KEY (`user_id`) REFERENCES `user` (`id`);

ALTER TABLE `user_group` ADD FOREIGN KEY (`group_id`) REFERENCES `group` (`id`);
