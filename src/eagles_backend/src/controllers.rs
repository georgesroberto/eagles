use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Department, Doctor, Patient, Consultation, Chat}; // Import all models
use crate::db::get_connection; // Ensure this function returns a valid connection
use crate::db::DbPool; // Import your DbPool type

// Function to get all departments
pub async fn get_departments(pool: web::Data<DbPool>) -> impl Responder {
    let conn = get_connection(&pool); // Get a DB connection

    // Using web::block to handle blocking database operations
    let departments: Vec<Department> = web::block(move || {
        use crate::schema::departments::dsl::*; // Adjust based on your schema
        departments.load::<Department>(&conn) // Load departments
    })
    .await
    .unwrap_or_else(|_| vec![]); // Handle potential errors

    HttpResponse::Ok().json(departments) // Return departments as JSON
}

// Function to get all doctors
pub async fn get_doctors(pool: web::Data<DbPool>) -> impl Responder {
    let conn = get_connection(&pool);

    let doctors: Vec<Doctor> = web::block(move || {
        use crate::schema::doctors::dsl::*; // Adjust based on your schema
        doctors.load::<Doctor>(&conn) // Load doctors
    })
    .await
    .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(doctors)
}

// Function to get all patients
pub async fn get_patients(pool: web::Data<DbPool>) -> impl Responder {
    let conn = get_connection(&pool);

    let patients: Vec<Patient> = web::block(move || {
        use crate::schema::patients::dsl::*; // Adjust based on your schema
        patients.load::<Patient>(&conn) // Load patients
    })
    .await
    .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(patients)
}

// Function to get all consultations
pub async fn get_consultations(pool: web::Data<DbPool>) -> impl Responder {
    let conn = get_connection(&pool);

    let consultations: Vec<Consultation> = web::block(move || {
        use crate::schema::consultations::dsl::*; // Adjust based on your schema
        consultations.load::<Consultation>(&conn) // Load consultations
    })
    .await
    .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(consultations)
}

// Function to get all chats
pub async fn get_chats(pool: web::Data<DbPool>) -> impl Responder {
    let conn = get_connection(&pool);

    let chats: Vec<Chat> = web::block(move || {
        use crate::schema::chats::dsl::*; // Adjust based on your schema
        chats.load::<Chat>(&conn) // Load chats
    })
    .await
    .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(chats)
}
