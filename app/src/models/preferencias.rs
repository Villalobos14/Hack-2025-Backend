#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PreferenciasNotificaciones {
    pub autor_id: AccountId,    // Wallet del autor
    pub donaciones: bool,
    pub estadisticas: bool,
    pub mensajes: bool,
}