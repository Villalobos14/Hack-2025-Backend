#![no_std]

// necesary crates
use sails_rs::prelude::*;

// import our modules 
pub mod states;
pub mod services;
pub mod app_types;

// Import service to be used for the program
use keyring_service::services::keyring_service::KeyringService;
use services::service::Service;

pub struct Program;

#[program]
impl Program {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        // Init the state
        Service::seed();

        Self
    }

    
    #[route("Service")]
    pub fn service_svc(&self) -> Service {
        Service::new()
    }

    #[route("Keyring")]
    pub fn keyring_svc(&self) -> KeyringService {
        KeyringService::new()
    }
}