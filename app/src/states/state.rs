

// Add your state code
use sails_rs::{
    prelude::*,
    collections::HashMap,
};

// Estado global de contrato
pub static mut STATE: Option<State> = None;

// Estructura para representar un autor
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Autor {
    pub id: ActorId,
    pub nombre: String,
    pub email: String,
    pub foto_perfil: String,
    pub descripcion: String,
    pub idioma_origen: String,
    pub ingresos_totales: u64,
    pub donaciones_recibidas: u64,
    pub fecha_registro: String,
    pub ultima_actividad: String,
    pub estado_cuenta: String,
    pub redes_sociales: Vec<RedSocial>,
    pub preferencias_notificaciones: PreferenciasNotificaciones,
    pub mangas: Vec<Manga>,

    pub nfts_creados: Vec<NFT>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct RedSocial {
    pub plataforma: String,
    pub url: String,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PreferenciasNotificaciones {
    pub donaciones: bool,
    pub estadisticas: bool,
    pub mensajes: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Manga {
    pub id: String,
    pub titulo: String,
    pub estado: String,
    pub cantidad_capitulos: i32,
    pub lecturas: i32,
    pub capitulos_gratis:i32,
    pub capitulos: Vec<Capitulo>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Capitulo {
    pub id: i32,
    pub nombre: String,
    pub url: String,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct NFT {
    pub id: String,
    pub descripcion: String,
    pub estado: String,
}

// Estructura para representar un lector
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Lector {
    pub id: ActorId,
    pub nombre: String,
    pub email: String,
    pub foto_perfil: String,
    pub fecha_registro: String,
    pub ultima_actividad: String,
    pub capitulos_desbloqueados: Vec<CapituloDesbloqueado>,
    pub nfts_adquiridos: Vec<NFTAdquirido>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct CapituloDesbloqueado {
    pub capitulo_id: i32,
    pub manga_id: String,
    pub fecha_desbloqueo: String,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct NFTAdquirido {
    pub nft_id: String,
    pub fecha_adquisicion: String,
}

// Estructura para el estado 
#[derive(Clone, Default)]
pub struct State {
    pub autores: HashMap<ActorId, Autor>,
    pub lectores: Vec<Lector>,
    
}

// Implementación de métodos para el estado
impl State {
    
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        };
    }

    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}