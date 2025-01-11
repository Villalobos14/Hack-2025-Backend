#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Manga {
    pub id: String,             // Identificador único
    pub autor_id: AccountId,    // Wallet del autor
    pub titulo: String,
    pub estado: String,         // "en_progreso", "finalizado", etc.
    pub cantidad_capitulos: u32,
    pub lecturas: u64,          // Número de lecturas totales
}