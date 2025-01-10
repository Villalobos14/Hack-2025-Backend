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
pub struct MyService<R> {
    remoting: R,
}
impl<R> MyService<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::MyService for MyService<R> {
    type Args = R::Args;
    fn hello(&mut self) -> impl Call<Output = String, Args = R::Args> {
        RemotingAction::<_, my_service::io::Hello>::new(self.remoting.clone(), ())
    }
}

pub mod my_service {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct Hello(());
        impl Hello {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Hello as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Hello {
            const ROUTE: &'static [u8] = &[
                36, 77, 121, 83, 101, 114, 118, 105, 99, 101, 20, 72, 101, 108, 108, 111,
            ];
            type Params = ();
            type Reply = String;
        }
    }
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
    pub trait MyService {
        type Args;
        fn hello(&mut self) -> impl Call<Output = String, Args = Self::Args>;
    }
}
