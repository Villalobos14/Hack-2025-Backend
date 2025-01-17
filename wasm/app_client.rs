// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct AppFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> AppFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::AppFactory for AppFactory<R> {
    type Args = R::Args;
    fn new(&self) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(self.remoting.clone(), ())
    }
}

pub mod app_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <New as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = ();
            type Reply = ();
        }
    }
}
pub struct Keyring<R> {
    remoting: R,
}
impl<R> Keyring<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::Keyring for Keyring<R> {
    type Args = R::Args;
    fn bind_keyring_data_to_user_address(
        &mut self,
        user_address: ActorId,
        keyring_data: KeyringData,
    ) -> impl Call<Output = KeyringEvent, Args = R::Args> {
        RemotingAction::<_, keyring::io::BindKeyringDataToUserAddress>::new(
            self.remoting.clone(),
            (user_address, keyring_data),
        )
    }
    fn bind_keyring_data_to_user_coded_name(
        &mut self,
        user_coded_name: String,
        keyring_data: KeyringData,
    ) -> impl Call<Output = KeyringEvent, Args = R::Args> {
        RemotingAction::<_, keyring::io::BindKeyringDataToUserCodedName>::new(
            self.remoting.clone(),
            (user_coded_name, keyring_data),
        )
    }
    fn keyring_account_data(
        &self,
        keyring_address: ActorId,
    ) -> impl Query<Output = KeyringQueryEvent, Args = R::Args> {
        RemotingAction::<_, keyring::io::KeyringAccountData>::new(
            self.remoting.clone(),
            keyring_address,
        )
    }
    fn keyring_address_from_user_address(
        &self,
        user_address: ActorId,
    ) -> impl Query<Output = KeyringQueryEvent, Args = R::Args> {
        RemotingAction::<_, keyring::io::KeyringAddressFromUserAddress>::new(
            self.remoting.clone(),
            user_address,
        )
    }
    fn keyring_address_from_user_coded_name(
        &self,
        user_coded_name: String,
    ) -> impl Query<Output = KeyringQueryEvent, Args = R::Args> {
        RemotingAction::<_, keyring::io::KeyringAddressFromUserCodedName>::new(
            self.remoting.clone(),
            user_coded_name,
        )
    }
}

pub mod keyring {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct BindKeyringDataToUserAddress(());
        impl BindKeyringDataToUserAddress {
            #[allow(dead_code)]
            pub fn encode_call(user_address: ActorId, keyring_data: super::KeyringData) -> Vec<u8> {
                <BindKeyringDataToUserAddress as ActionIo>::encode_call(&(
                    user_address,
                    keyring_data,
                ))
            }
        }
        impl ActionIo for BindKeyringDataToUserAddress {
            const ROUTE: &'static [u8] = &[
                28, 75, 101, 121, 114, 105, 110, 103, 112, 66, 105, 110, 100, 75, 101, 121, 114,
                105, 110, 103, 68, 97, 116, 97, 84, 111, 85, 115, 101, 114, 65, 100, 100, 114, 101,
                115, 115,
            ];
            type Params = (ActorId, super::KeyringData);
            type Reply = super::KeyringEvent;
        }
        pub struct BindKeyringDataToUserCodedName(());
        impl BindKeyringDataToUserCodedName {
            #[allow(dead_code)]
            pub fn encode_call(
                user_coded_name: String,
                keyring_data: super::KeyringData,
            ) -> Vec<u8> {
                <BindKeyringDataToUserCodedName as ActionIo>::encode_call(&(
                    user_coded_name,
                    keyring_data,
                ))
            }
        }
        impl ActionIo for BindKeyringDataToUserCodedName {
            const ROUTE: &'static [u8] = &[
                28, 75, 101, 121, 114, 105, 110, 103, 120, 66, 105, 110, 100, 75, 101, 121, 114,
                105, 110, 103, 68, 97, 116, 97, 84, 111, 85, 115, 101, 114, 67, 111, 100, 101, 100,
                78, 97, 109, 101,
            ];
            type Params = (String, super::KeyringData);
            type Reply = super::KeyringEvent;
        }
        pub struct KeyringAccountData(());
        impl KeyringAccountData {
            #[allow(dead_code)]
            pub fn encode_call(keyring_address: ActorId) -> Vec<u8> {
                <KeyringAccountData as ActionIo>::encode_call(&keyring_address)
            }
        }
        impl ActionIo for KeyringAccountData {
            const ROUTE: &'static [u8] = &[
                28, 75, 101, 121, 114, 105, 110, 103, 72, 75, 101, 121, 114, 105, 110, 103, 65, 99,
                99, 111, 117, 110, 116, 68, 97, 116, 97,
            ];
            type Params = ActorId;
            type Reply = super::KeyringQueryEvent;
        }
        pub struct KeyringAddressFromUserAddress(());
        impl KeyringAddressFromUserAddress {
            #[allow(dead_code)]
            pub fn encode_call(user_address: ActorId) -> Vec<u8> {
                <KeyringAddressFromUserAddress as ActionIo>::encode_call(&user_address)
            }
        }
        impl ActionIo for KeyringAddressFromUserAddress {
            const ROUTE: &'static [u8] = &[
                28, 75, 101, 121, 114, 105, 110, 103, 116, 75, 101, 121, 114, 105, 110, 103, 65,
                100, 100, 114, 101, 115, 115, 70, 114, 111, 109, 85, 115, 101, 114, 65, 100, 100,
                114, 101, 115, 115,
            ];
            type Params = ActorId;
            type Reply = super::KeyringQueryEvent;
        }
        pub struct KeyringAddressFromUserCodedName(());
        impl KeyringAddressFromUserCodedName {
            #[allow(dead_code)]
            pub fn encode_call(user_coded_name: String) -> Vec<u8> {
                <KeyringAddressFromUserCodedName as ActionIo>::encode_call(&user_coded_name)
            }
        }
        impl ActionIo for KeyringAddressFromUserCodedName {
            const ROUTE: &'static [u8] = &[
                28, 75, 101, 121, 114, 105, 110, 103, 124, 75, 101, 121, 114, 105, 110, 103, 65,
                100, 100, 114, 101, 115, 115, 70, 114, 111, 109, 85, 115, 101, 114, 67, 111, 100,
                101, 100, 78, 97, 109, 101,
            ];
            type Params = String;
            type Reply = super::KeyringQueryEvent;
        }
    }
}
pub struct Service<R> {
    remoting: R,
}
impl<R> Service<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::Service for Service<R> {
    type Args = R::Args;
    fn collect_earnings(
        &mut self,
        lector_id: ActorId,
    ) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::CollectEarnings>::new(self.remoting.clone(), lector_id)
    }
    fn create_author(
        &mut self,
        actor_id: ActorId,
        nombre: String,
        email: String,
    ) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::CreateAuthor>::new(
            self.remoting.clone(),
            (actor_id, nombre, email),
        )
    }
    fn unlock_chapter(
        &mut self,
        lector_id: ActorId,
        capitulo_id: i32,
        manga_id: String,
    ) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::UnlockChapter>::new(
            self.remoting.clone(),
            (lector_id, capitulo_id, manga_id),
        )
    }
    fn query_author(
        &self,
        actor_id: ActorId,
    ) -> impl Query<Output = Option<Autor>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryAuthor>::new(self.remoting.clone(), actor_id)
    }
    fn query_manga(&self, manga_id: String) -> impl Query<Output = Option<Manga>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryManga>::new(self.remoting.clone(), manga_id)
    }
    fn query_reader(
        &self,
        lector_id: ActorId,
    ) -> impl Query<Output = Option<Lector>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryReader>::new(self.remoting.clone(), lector_id)
    }
}

pub mod service {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct CollectEarnings(());
        impl CollectEarnings {
            #[allow(dead_code)]
            pub fn encode_call(lector_id: ActorId) -> Vec<u8> {
                <CollectEarnings as ActionIo>::encode_call(&lector_id)
            }
        }
        impl ActionIo for CollectEarnings {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 60, 67, 111, 108, 108, 101, 99, 116, 69, 97,
                114, 110, 105, 110, 103, 115,
            ];
            type Params = ActorId;
            type Reply = super::Events;
        }
        pub struct CreateAuthor(());
        impl CreateAuthor {
            #[allow(dead_code)]
            pub fn encode_call(actor_id: ActorId, nombre: String, email: String) -> Vec<u8> {
                <CreateAuthor as ActionIo>::encode_call(&(actor_id, nombre, email))
            }
        }
        impl ActionIo for CreateAuthor {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 48, 67, 114, 101, 97, 116, 101, 65, 117, 116,
                104, 111, 114,
            ];
            type Params = (ActorId, String, String);
            type Reply = super::Events;
        }
        pub struct UnlockChapter(());
        impl UnlockChapter {
            #[allow(dead_code)]
            pub fn encode_call(lector_id: ActorId, capitulo_id: i32, manga_id: String) -> Vec<u8> {
                <UnlockChapter as ActionIo>::encode_call(&(lector_id, capitulo_id, manga_id))
            }
        }
        impl ActionIo for UnlockChapter {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 52, 85, 110, 108, 111, 99, 107, 67, 104, 97,
                112, 116, 101, 114,
            ];
            type Params = (ActorId, i32, String);
            type Reply = super::Events;
        }
        pub struct QueryAuthor(());
        impl QueryAuthor {
            #[allow(dead_code)]
            pub fn encode_call(actor_id: ActorId) -> Vec<u8> {
                <QueryAuthor as ActionIo>::encode_call(&actor_id)
            }
        }
        impl ActionIo for QueryAuthor {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 44, 81, 117, 101, 114, 121, 65, 117, 116, 104,
                111, 114,
            ];
            type Params = ActorId;
            type Reply = Option<super::Autor>;
        }
        pub struct QueryManga(());
        impl QueryManga {
            #[allow(dead_code)]
            pub fn encode_call(manga_id: String) -> Vec<u8> {
                <QueryManga as ActionIo>::encode_call(&manga_id)
            }
        }
        impl ActionIo for QueryManga {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 40, 81, 117, 101, 114, 121, 77, 97, 110, 103,
                97,
            ];
            type Params = String;
            type Reply = Option<super::Manga>;
        }
        pub struct QueryReader(());
        impl QueryReader {
            #[allow(dead_code)]
            pub fn encode_call(lector_id: ActorId) -> Vec<u8> {
                <QueryReader as ActionIo>::encode_call(&lector_id)
            }
        }
        impl ActionIo for QueryReader {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 44, 81, 117, 101, 114, 121, 82, 101, 97, 100,
                101, 114,
            ];
            type Params = ActorId;
            type Reply = Option<super::Lector>;
        }
    }
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct KeyringData {
    pub address: String,
    pub encoded: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringEvent {
    KeyringAccountSet,
    Error(KeyringError),
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringError {
    KeyringAddressAlreadyEsists,
    UserAddressAlreadyExists,
    UserCodedNameAlreadyExists,
    UserDoesNotHasKeyringAccount,
    KeyringAccountAlreadyExists,
    SessionHasInvalidCredentials,
    UserAndKeyringAddressAreTheSame,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringQueryEvent {
    LastWhoCall(ActorId),
    SignlessAccountAddress(Option<ActorId>),
    SignlessAccountData(Option<KeyringData>),
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    AuthorCreated(String),
    ChapterUnlocked((String, i32)),
    EarningsCollected(String),
    Error(String),
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
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
    pub nfts_creados: Vec<Nft>,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct RedSocial {
    pub plataforma: String,
    pub url: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PreferenciasNotificaciones {
    pub donaciones: bool,
    pub estadisticas: bool,
    pub mensajes: bool,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Manga {
    pub id: String,
    pub titulo: String,
    pub estado: String,
    pub cantidad_capitulos: i32,
    pub lecturas: i32,
    pub capitulos: Vec<Capitulo>,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Capitulo {
    pub id: i32,
    pub nombre: String,
    pub url: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Nft {
    pub id: String,
    pub descripcion: String,
    pub estado: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
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
    pub nfts_adquiridos: Vec<NftAdquirido>,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct CapituloDesbloqueado {
    pub capitulo_id: i32,
    pub manga_id: String,
    pub fecha_desbloqueo: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct NftAdquirido {
    pub nft_id: String,
    pub fecha_adquisicion: String,
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(&self) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait Keyring {
        type Args;
        fn bind_keyring_data_to_user_address(
            &mut self,
            user_address: ActorId,
            keyring_data: KeyringData,
        ) -> impl Call<Output = KeyringEvent, Args = Self::Args>;
        fn bind_keyring_data_to_user_coded_name(
            &mut self,
            user_coded_name: String,
            keyring_data: KeyringData,
        ) -> impl Call<Output = KeyringEvent, Args = Self::Args>;
        fn keyring_account_data(
            &self,
            keyring_address: ActorId,
        ) -> impl Query<Output = KeyringQueryEvent, Args = Self::Args>;
        fn keyring_address_from_user_address(
            &self,
            user_address: ActorId,
        ) -> impl Query<Output = KeyringQueryEvent, Args = Self::Args>;
        fn keyring_address_from_user_coded_name(
            &self,
            user_coded_name: String,
        ) -> impl Query<Output = KeyringQueryEvent, Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait Service {
        type Args;
        fn collect_earnings(
            &mut self,
            lector_id: ActorId,
        ) -> impl Call<Output = Events, Args = Self::Args>;
        fn create_author(
            &mut self,
            actor_id: ActorId,
            nombre: String,
            email: String,
        ) -> impl Call<Output = Events, Args = Self::Args>;
        fn unlock_chapter(
            &mut self,
            lector_id: ActorId,
            capitulo_id: i32,
            manga_id: String,
        ) -> impl Call<Output = Events, Args = Self::Args>;
        fn query_author(
            &self,
            actor_id: ActorId,
        ) -> impl Query<Output = Option<Autor>, Args = Self::Args>;
        fn query_manga(
            &self,
            manga_id: String,
        ) -> impl Query<Output = Option<Manga>, Args = Self::Args>;
        fn query_reader(
            &self,
            lector_id: ActorId,
        ) -> impl Query<Output = Option<Lector>, Args = Self::Args>;
    }
}
