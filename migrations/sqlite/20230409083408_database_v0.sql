CREATE TABLE `user` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `external_id` TEXT UNIQUE NOT NULL,
  `created_at` datetime NOT NULL
);

CREATE TABLE `group` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` TEXT UNIQUE NOT NULL
);

CREATE TABLE `message` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `payload` TEXT NOT NULL,
  `created_at` datetime NOT NULL
);

CREATE TABLE `user_message` (
  `user_id` INTEGER NOT NULL,
  `message_id` INTEGER NOT NULL,
  `read` bool NOT NULL DEFAULT 0,
  PRIMARY KEY(`user_id`, `message_id`),
  FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  FOREIGN KEY (`message_id`) REFERENCES `message` (`id`)
);

CREATE TABLE `user_group` (
  `user_id` INTEGER NOT NULL,
  `group_id` INTEGER NOT NULL,
  PRIMARY KEY(`user_id`, `group_id`),
  FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  FOREIGN KEY (`group_id`) REFERENCES `group` (`id`)
);
