CREATE TABLE IF NOT EXISTS `users` (
    `id` BLOB PRIMARY KEY NOT NULL,
    -- this is bad i know i'll deal with this later (if i even need to)
    -- at this stage `roles` is string with format "role1|role2"
    `roles` TEXT NOT NULL,
    `username` TEXT NOT NULL UNIQUE,
    `password` TEXT NOT NULL,
    `email` TEXT NOT NULL UNIQUE,
    `full_name` TEXT NOT NULL,
    `active` BOOLEAN NOT NULL DEFAULT 0,
    `force_password_reset` BOOLEAN NOT NULL DEFAULT 0,
    `onboarding_done` BOOLEAN NOT NULL DEFAULT 0,
    `request_date` DATE NOT NULL DEFAULT (datetime('now','localtime')),
    `join_date` DATE NULL,
    `update_date` DATE NULL,
    `totp_secret` BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS `totp_request`(
    `user_id` BLOB PRIMARY KEY NOT NULL,
    `totp_secret` BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS `user_rejections`(
    `username` TEXT NOT NULL,
    `email` TEXT NOT NULL,
    `request_date` DATE NOT NULL,
    `rejection_date` TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS `users_signup_requests` (
    `user_id` BLOB PRIMARY KEY NOT NULL,
    `request_text` TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS `types`(
    `id` BLOB PRIMARY KEY NOT NULL,
    `name` TEXT NOT NULL,
    `description` TEXT
);

CREATE TABLE IF NOT EXISTS `devices`(
    `id` BLOB PRIMARY KEY NOT NULL,
    `mac_address` TEXT UNIQUE NOT NULL,
    `name` TEXT NOT NULL,
    `description` TEXT,
    `on` BOOLEAN NOT NULL DEFAULT 0
);

-- if you have a lot of devices just create and index on `device_id`
CREATE TABLE IF NOT EXISTS `device_types`(
    `device_id` BLOB NOT NULL,
    `type_id` BLOB NOT NULL,
    FOREIGN KEY(`device_id`) REFERENCES device(`id`),
    FOREIGN KEY(`type_id`) REFERENCES types(`id`),
    UNIQUE(`device_id`,`type_id`)
);

CREATE TABLE IF NOT EXISTS `user_devices`(
    `user_id` BLOB NOT NULL,
    `device_id` BLOB NOT NULL,
    `visible` BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY(`user_id`) REFERENCES users(`id`),
    FOREIGN KEY(`device_id`) REFERENCES device(`id`),
    UNIQUE(`user_id`, `device_id`)
);

INSERT INTO users(id, roles, username, password, email, full_name, active, force_password_reset, request_date, join_date, update_date, totp_secret)
VALUES (X'550e8400e29b41d4a716446655440000', 'admin|user','admin','$argon2id$v=19$m=15000,t=2,p=1$OXh6dTJTOHJSVnJWZEZSUw$Gbm36MmV+3adnEAo+j8OMQ','admin@admin.com','my admin',1,1,datetime('now','start of day','localtime'), datetime('now','start of day','localtime'), datetime('now','start of day','localtime'), X'155555');
