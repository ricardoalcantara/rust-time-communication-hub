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
  `payload` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL
);

CREATE TABLE `user_message` (
  `user_id` integer NOT NULL,
  `message_id` integer NOT NULL,
  `read` bool NOT NULL DEFAULT 0,
  PRIMARY KEY(`user_id`, `message_id`),
  FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  FOREIGN KEY (`message_id`) REFERENCES `message` (`id`)
);

CREATE TABLE `user_group` (
  `user_id` integer NOT NULL,
  `group_id` integer NOT NULL,
  PRIMARY KEY(`user_id`, `group_id`),
  FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  FOREIGN KEY (`group_id`) REFERENCES `group` (`id`)
);
