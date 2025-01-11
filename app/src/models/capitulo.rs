#[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Capitulo {
    pub id: String,             // Identificador único
    pub manga_id: String,       // FK -> Manga.id
    pub nombre: String,
    pub url: String,            // URL para IPFS o almacenamiento descentralizado
}