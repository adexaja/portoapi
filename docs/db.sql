CREATE TABLE `user` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `username` varchar(50),
  `password` varchar(100),
  `name` varchar(100),
  `refresh_token` varchar(100),
  `created_at` datetime,
  `updated_at` datetime
);

CREATE TABLE `work` (
  `work_id` int PRIMARY KEY AUTO_INCREMENT,
  `work_name` varchar(100),
  `work_logo` varchar(200),
  `work_desc` text,
  `work_from` date,
  `work_to` date,
  `work_until_now` tinyint
);

CREATE TABLE `expertise` (
  `exp_id` int PRIMARY KEY AUTO_INCREMENT,
  `exp_logo` varchar(200),
  `exp_name` varchar(100)
);

CREATE TABLE `services` (
  `service_id` int PRIMARY KEY AUTO_INCREMENT,
  `service_name` varchar(100),
  `service_logo` varchar(200)
);

CREATE TABLE `projects` (
  `project_id` int PRIMARY KEY AUTO_INCREMENT,
  `project_featured_image` varchar(200),
  `project_title` text,
  `project_client` varchar(100),
  `service_id` int,
  `project_description` text,
  `project_situation` text,
  `project_task` text,
  `project_action` text,
  `project_result` text,
  `project_conclusion` text,
  `project_start` date,
  `project_end` date,
  `project_link` varchar(200)
);

CREATE TABLE `clients` (
  `client_id` int PRIMARY KEY AUTO_INCREMENT,
  `client_name` varchar(100),
  `client_logo` varchar(200)
);

CREATE TABLE `testimonials` (
  `testimonial_id` int PRIMARY KEY AUTO_INCREMENT,
  `testimonial_name` varchar(100),
  `testimonial_photo` varchar(200),
  `testimonial_review` text,
  `testimonial_occuption` varchar(100),
  `testimonial_company` varchar(100)
);

CREATE TABLE `ceritificates` (
  `cert_id` int PRIMARY KEY AUTO_INCREMENT,
  `cert_title` text,
  `cert_year` int(4),
  `cert_link` varchar(200)
);

CREATE TABLE `faq` (
  `faq_id` int PRIMARY KEY AUTO_INCREMENT,
  `faq_question` text,
  `faq_answer` text
);

CREATE TABLE `post` (
  `post_id` bigint(20) PRIMARY KEY NOT NULL,
  `post_title` text DEFAULT null,
  `post_slug` text DEFAULT null,
  `post_content` longtext DEFAULT null,
  `post_date` datetime DEFAULT null,
  `meta_title` text DEFAULT null,
  `meta_description` text DEFAULT null,
  `meta_keyword` text DEFAULT null,
  `post_status` ENUM ('draft', 'submit', 'publish', 'schedule') DEFAULT null,
  `post_type` varchar(10) DEFAULT null,
  `post_view` bigint(20) DEFAULT null,
  `user_id` int(11) DEFAULT null,
  `post_video_url` text DEFAULT null,
  `post_excerpt` varchar(300) DEFAULT null,
  `first_image` text DEFAULT null,
  `thumbnail` text DEFAULT null,
  `medium_thumbnail` text DEFAULT null,
  `post_trending_topic` tinyint(1) DEFAULT null,
  `post_hottopic` tinyint(1) DEFAULT null,
  `post_slider` tinyint(1) DEFAULT null,
  `post_malayhomeland` tinyint(1) DEFAULT null,
  `post_advetorial` tinyint(1) DEFAULT null,
  `created_at` datetime DEFAULT (current_timestamp()),
  `updated_at` datetime DEFAULT null
);

CREATE TABLE `category` (
  `category_id` int(11) PRIMARY KEY NOT NULL,
  `category_name` varchar(200) DEFAULT null,
  `category_slug` varchar(200) DEFAULT null,
  `category_thumbnail` text DEFAULT null,
  `parent_id` int(11) DEFAULT null,
  `category_active` tinyint(1) DEFAULT null,
  `category_show` tinyint(1) DEFAULT null
);

CREATE TABLE `contact` (
  `contact_id` int(11) PRIMARY KEY NOT NULL,
  `contact_title` varchar(200) DEFAULT null,
  `contact_name` varchar(200) DEFAULT null,
  `contact_icon` varchar(200) DEFAULT null
);

CREATE TABLE `tag` (
  `tag_id` int(11) PRIMARY KEY NOT NULL,
  `tag_name` varchar(200) DEFAULT null,
  `tag_slug` varchar(200) DEFAULT null,
  `tag_popular` tinyint(1) DEFAULT 0,
  `tag_count` int(11) DEFAULT 0
);

CREATE TABLE `post_categories` (
  `cat_id` bigint(20) PRIMARY KEY NOT NULL,
  `category_id` int(11) DEFAULT null,
  `post_id` bigint(20) DEFAULT null
);

CREATE TABLE `post_tags` (
  `t_id` bigint(20) PRIMARY KEY NOT NULL,
  `tag_id` int(11) DEFAULT null,
  `post_id` bigint(20) DEFAULT null
);

CREATE TABLE `setting` (
  `setting_id` int(11) PRIMARY KEY NOT NULL,
  `key` varchar(100) DEFAULT null,
  `value` text DEFAULT null
);

ALTER TABLE `projects` ADD FOREIGN KEY (`service_id`) REFERENCES `services` (`service_id`);

ALTER TABLE `post_categories` ADD CONSTRAINT `fk_post_categories_category1` FOREIGN KEY (`category_id`) REFERENCES `category` (`category_id`) ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE `post_categories` ADD CONSTRAINT `fk_post_categories_post1` FOREIGN KEY (`post_id`) REFERENCES `post` (`post_id`) ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE `post_tags` ADD CONSTRAINT `fk_post_tags_post1` FOREIGN KEY (`post_id`) REFERENCES `post` (`post_id`) ON DELETE NO ACTION ON UPDATE NO ACTION;

ALTER TABLE `post_tags` ADD CONSTRAINT `fk_post_tags_tag1` FOREIGN KEY (`tag_id`) REFERENCES `tag` (`tag_id`) ON DELETE NO ACTION ON UPDATE NO ACTION;
