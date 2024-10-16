use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub specialization_id: i32,
    pub image: String,
}

#[derive(Queryable, Serialize)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Queryable, Serialize)]
pub struct Consultation {
    pub id: i32,
    pub patient_id: i32,
    pub problem: String,
    pub recommended_specialization_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct Chat {
    pub id: i32,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub message: String,
    pub timestamp: String, // Or use `chrono::NaiveDateTime` for better handling
}
