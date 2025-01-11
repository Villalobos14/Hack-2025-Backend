#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Nft {
    pub id: String,             // Identificador único del NFT
    pub autor_id: AccountId,    // Wallet del autor
    pub descripcion: String,
    pub estado: String,         // "activo", "transferido", etc.
    pub duenio: AccountId,      // Wallet del lector actual
}