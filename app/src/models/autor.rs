#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Autor {
    pub id: AccountId,          // Wallet del autor
    pub nombre: String,
    pub email: String,
    pub foto_perfil: String,
    pub descripcion: String,
    pub idioma_origen: String,
    pub ingresos_totales: u128, // Ingresos acumulados en la blockchain
    pub donaciones_recibidas: u128,
    pub fecha_registro: u64,    // Timestamp
    pub ultima_actividad: u64, // Timestamp
    pub estado_cuenta: String, // "activo", "suspendido", etc.
}
