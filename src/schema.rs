// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Nullable<Integer>,
        patient_id -> Nullable<Integer>,
        doctor_id -> Nullable<Integer>,
        message -> Text,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    consultations (id) {
        id -> Nullable<Integer>,
        patient_id -> Nullable<Integer>,
        problem -> Text,
        recommended_specialization_id -> Nullable<Integer>,
    }
}

diesel::table! {
    departments (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    doctors (id) {
        id -> Nullable<Integer>,
        name -> Text,
        specialization_id -> Nullable<Integer>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    patients (id) {
        id -> Nullable<Integer>,
        name -> Text,
        age -> Integer,
    }
}

diesel::joinable!(chats -> doctors (doctor_id));
diesel::joinable!(chats -> patients (patient_id));
diesel::joinable!(consultations -> departments (recommended_specialization_id));
diesel::joinable!(consultations -> patients (patient_id));
diesel::joinable!(doctors -> departments (specialization_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    consultations,
    departments,
    doctors,
    patients,
);
