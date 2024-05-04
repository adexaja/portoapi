// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PostPostStatusEnum;
}

diesel::table! {
    category (category_id) {
        category_id -> Integer,
        #[max_length = 200]
        category_name -> Nullable<Varchar>,
        #[max_length = 200]
        category_slug -> Nullable<Varchar>,
        category_thumbnail -> Nullable<Text>,
        parent_id -> Nullable<Integer>,
        category_active -> Nullable<Bool>,
        category_show -> Nullable<Bool>,
    }
}

diesel::table! {
    ceritificates (cert_id) {
        cert_id -> Integer,
        cert_title -> Nullable<Text>,
        cert_year -> Nullable<Integer>,
        #[max_length = 200]
        cert_link -> Nullable<Varchar>,
    }
}

diesel::table! {
    clients (client_id) {
        client_id -> Integer,
        #[max_length = 100]
        client_name -> Nullable<Varchar>,
        #[max_length = 200]
        client_logo -> Nullable<Varchar>,
    }
}

diesel::table! {
    contact (contact_id) {
        contact_id -> Integer,
        #[max_length = 200]
        contact_title -> Nullable<Varchar>,
        #[max_length = 200]
        contact_name -> Nullable<Varchar>,
        #[max_length = 200]
        contact_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    expertise (exp_id) {
        exp_id -> Integer,
        #[max_length = 200]
        exp_logo -> Nullable<Varchar>,
        #[max_length = 100]
        exp_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    faq (faq_id) {
        faq_id -> Integer,
        faq_question -> Nullable<Text>,
        faq_answer -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostPostStatusEnum;

    post (post_id) {
        post_id -> Bigint,
        post_title -> Nullable<Text>,
        post_slug -> Nullable<Text>,
        post_content -> Nullable<Longtext>,
        post_date -> Nullable<Datetime>,
        meta_title -> Nullable<Text>,
        meta_description -> Nullable<Text>,
        meta_keyword -> Nullable<Text>,
        #[max_length = 8]
        post_status -> Nullable<PostPostStatusEnum>,
        #[max_length = 10]
        post_type -> Nullable<Varchar>,
        post_view -> Nullable<Bigint>,
        user_id -> Nullable<Integer>,
        post_video_url -> Nullable<Text>,
        #[max_length = 300]
        post_excerpt -> Nullable<Varchar>,
        first_image -> Nullable<Text>,
        thumbnail -> Nullable<Text>,
        medium_thumbnail -> Nullable<Text>,
        post_trending_topic -> Nullable<Bool>,
        post_hottopic -> Nullable<Bool>,
        post_slider -> Nullable<Bool>,
        post_malayhomeland -> Nullable<Bool>,
        post_advetorial -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    post_categories (cat_id) {
        cat_id -> Bigint,
        category_id -> Nullable<Integer>,
        post_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    post_tags (t_id) {
        t_id -> Bigint,
        tag_id -> Nullable<Integer>,
        post_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Integer,
        #[max_length = 200]
        project_featured_image -> Nullable<Varchar>,
        project_title -> Nullable<Text>,
        #[max_length = 100]
        project_client -> Nullable<Varchar>,
        service_id -> Nullable<Integer>,
        project_description -> Nullable<Text>,
        project_situation -> Nullable<Text>,
        project_task -> Nullable<Text>,
        project_action -> Nullable<Text>,
        project_result -> Nullable<Text>,
        project_conclusion -> Nullable<Text>,
        project_start -> Nullable<Date>,
        project_end -> Nullable<Date>,
        #[max_length = 200]
        project_link -> Nullable<Varchar>,
    }
}

diesel::table! {
    services (service_id) {
        service_id -> Integer,
        #[max_length = 100]
        service_name -> Nullable<Varchar>,
        #[max_length = 200]
        service_logo -> Nullable<Varchar>,
    }
}

diesel::table! {
    setting (setting_id) {
        setting_id -> Integer,
        #[max_length = 100]
        key -> Nullable<Varchar>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    tag (tag_id) {
        tag_id -> Integer,
        #[max_length = 200]
        tag_name -> Nullable<Varchar>,
        #[max_length = 200]
        tag_slug -> Nullable<Varchar>,
        tag_popular -> Nullable<Bool>,
        tag_count -> Nullable<Integer>,
    }
}

diesel::table! {
    testimonials (testimonial_id) {
        testimonial_id -> Integer,
        #[max_length = 100]
        testimonial_name -> Nullable<Varchar>,
        #[max_length = 200]
        testimonial_photo -> Nullable<Varchar>,
        testimonial_review -> Nullable<Text>,
        #[max_length = 100]
        testimonial_occuption -> Nullable<Varchar>,
        #[max_length = 100]
        testimonial_company -> Nullable<Varchar>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Nullable<Varchar>,
        #[max_length = 100]
        password -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        refresh_token -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    work (work_id) {
        work_id -> Integer,
        #[max_length = 100]
        work_name -> Nullable<Varchar>,
        #[max_length = 200]
        work_logo -> Nullable<Varchar>,
        work_desc -> Nullable<Text>,
        work_from -> Nullable<Date>,
        work_to -> Nullable<Date>,
        work_until_now -> Nullable<Tinyint>,
    }
}

diesel::joinable!(post_categories -> category (category_id));
diesel::joinable!(post_categories -> post (post_id));
diesel::joinable!(post_tags -> post (post_id));
diesel::joinable!(post_tags -> tag (tag_id));
diesel::joinable!(projects -> services (service_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    ceritificates,
    clients,
    contact,
    expertise,
    faq,
    post,
    post_categories,
    post_tags,
    projects,
    services,
    setting,
    tag,
    testimonials,
    user,
    work,
);
