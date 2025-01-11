// Add your service code

// service.rs

// necessary crates
use sails_rs::{
    prelude::*,
    collections::HashMap,
    gstd::msg,
};

// Import the state
use crate::states::*;
use crate::services::service::state::*;

// Defining the service
#[derive(Default)]
pub struct Service;

impl Service {
    // Function to initialize the service state (call only once)
    pub fn seed() {
        State::init_state();
    }
}

// Macro to define service implementation
#[service]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Create a new author with initial parameters and store in state
    pub fn create_author(&mut self, nombre: String, email: String) -> Events {
        // Add validations
        let caller = msg::source();
        let state = State::state_mut();

        // Check if author already exists
        if state.autores.contains_key(&caller) {
            return Events::Error("El autor ya existe.".into());
        }

        // Create new author
        let new_author = Autor {
            id: caller,
            nombre,
            email,
            ..Default::default()
        };


        // Add to state
        state.autores.insert(caller,new_author);

        Events::AuthorCreated(caller.to_string())
    }


    pub fn add_manga(&mut self,id:String,titulo:String,estado:String,cantidad_capitulos:i32,capitulos_gratis:i32,capitulo:Vec<Capitulo>)->Events{
        let state=State::state_mut();
        let caller=msg::source();
        state.autores
        .entry(caller)
        .and_modify(|autor|autor.mangas.push({
            Manga{
                id:id,
                titulo:titulo,
                estado:estado,
                cantidad_capitulos:cantidad_capitulos,
                
                capitulos:capitulo,
                capitulos_gratis:capitulos_gratis,
                lecturas:0

            }
        }));
        Events::MangaCreated(caller.to_string())


    }
    

    // Allows a reader to unlock a manga chapter
    // pub fn unlock_chapter(&mut self, lector_id: ActorId, capitulo_id: i32, manga_id: String) -> Events {
    //     // Add validations
    //     let state = State::state_mut();

    //     // Verify if lector exists
    //     if !state.lectores.iter().any(|l| l.id == lector_id) {
    //         return Events::Error("El lector no existe.".into());
    //     }

    //     // Verify if manga and chapter exist
    //     let manga_option = state.autores.iter()
    //         .flat_map(|autor| &autor.mangas)
    //         .find(|m| m.id == manga_id);

    //     if let Some(manga) = manga_option {
    //         if manga.capitulos.iter().any(|c| c.id == capitulo_id) {
    //             // Unlock chapter for lector
    //             let lector = state.lectores.iter_mut().find(|l| l.id == lector_id).unwrap();
    //             lector.capitulos_desbloqueados.push(CapituloDesbloqueado {
    //                 capitulo_id,
    //                 manga_id: manga_id.clone(),
    //                 fecha_desbloqueo: "fecha_actual".into(), // Replace with actual current date
    //             });
    //             return Events::ChapterUnlocked(manga_id, capitulo_id);
    //         }
    //     }

    //     Events::Error("Manga o capítulo no encontrado.".into())
    // }

    // Reader requests to collect earnings in their wallet
    // pub fn collect_earnings(&mut self, lector_id: ActorId) -> Events {
    //     // Add validations
    //     let state = State::state_mut();

    //     // Verify reader existence
    //     if !state.lectores.iter().any(|l| l.id == lector_id) {
    //         return Events::Error("El lector no existe.".into());
    //     }

    //     // Logic for transferring tokens could be implemented here
    //     // For now, we just acknowledge the request
    //     Events::EarningsCollected(lector_id.to_string())
    // }

    // Query current state of an author by ID
    pub fn my_chapters(&self) -> Option<Autor> {
        let caller=msg::source();
        State::state_ref()
            .autores
            .get(&caller)
            .cloned()

    }

    // Get all authors
    pub fn get_all_autors(&self) -> Vec<(ActorId, Autor)> {
        let state = State::state_ref();

        state.autores   
            .iter()
            .map(|(autor_address, autor_data)| (*autor_address, autor_data.clone()))
            .collect()
    }

    pub fn get_by_actor_id(&self,actorid:ActorId)->Option<Autor>{
        State::state_ref().autores.get(&actorid).cloned()
    }

    


    

    // // Query current state of a reader by ID
    // pub fn query_reader(&self, lector_id: ActorId) -> Option<Lector> {
    //     State::state_ref()
    //         .lectores
    //         .iter()
    //         .find(|l| l.id == lector_id)
    //         .cloned()
    // }

    // // Query current state of a manga by its ID
    // pub fn query_manga(&self, manga_id: String) -> Option<Manga> {
    //     State::state_ref()
    //         .autores
    //         .iter()
    //         .flat_map(|autor| &autor.mangas)
    //         .find(|m| m.id == manga_id)
    //         .cloned()
    // }
}

// Struct for responses to the user
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    AuthorCreated(String),
    MangaCreated(String),
 
    
    Error(String),
}
