#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Lector {
    pub id: AccountId,          // Wallet del lector
    pub nombre: String,
    pub email: String,
    pub foto_perfil: String,
    pub fecha_registro: u64,    // Timestamp
    pub ultima_actividad: u64,  // Timestamp
}