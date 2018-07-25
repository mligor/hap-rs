use std::rc::Rc;

use hyper::{self, Uri, StatusCode, server::Response};
use failure::Error;
use futures::{future, Future};
use uuid::Uuid;

use config::ConfigPtr;
use db::{DatabasePtr, AccessoryList};
use transport::http::{tlv_response, status_response, server::EventSubscriptions};
use protocol::tlv::{self, Encodable};
use event::EmitterPtr;

pub mod accessories;
pub mod characteristics;
pub mod identify;
pub mod pair_setup;
pub mod pair_verify;
pub mod pairings;

pub trait Handler {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: Rc<Option<Uuid>>,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessories: &AccessoryList,
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>>;
}

pub trait TlvHandler {
    type ParseResult;
    type Result: Encodable;
    fn parse(&self, body: Vec<u8>) -> Result<Self::ParseResult, tlv::ErrorContainer>;
    fn handle(
        &mut self,
        step: Self::ParseResult,
        config: &ConfigPtr,
        database: &DatabasePtr,
        event_emitter: &EmitterPtr,
    ) -> Result<Self::Result, tlv::ErrorContainer>;
}

pub struct TlvHandlerType<T: TlvHandler>(T);

impl<T: TlvHandler> From<T> for TlvHandlerType<T> {
    fn from(inst: T) -> TlvHandlerType<T> {
        TlvHandlerType(inst)
    }
}

impl<T: TlvHandler> Handler for TlvHandlerType<T> {
    fn handle(
        &mut self,
        _: Uri,
        body: Vec<u8>,
        _: Rc<Option<Uuid>>,
        _: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        _: &AccessoryList,
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let response = match self.0.parse(body) {
            Err(e) => e.encode(),
            Ok(step) => match self.0.handle(step, config, database, event_emitter) {
                Err(e) => e.encode(),
                Ok(res) => res.encode(),
            }
        };
        Box::new(future::ok(tlv_response(response, StatusCode::Ok)))
    }
}

pub trait JsonHandler {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: Rc<Option<Uuid>>,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        event_emitter: &EmitterPtr,
    ) -> Result<Response, Error>;
}

pub struct JsonHandlerType<T: JsonHandler>(T);

impl<T: JsonHandler> From<T> for JsonHandlerType<T> {
    fn from(inst: T) -> JsonHandlerType<T> {
        JsonHandlerType(inst)
    }
}

impl<T: JsonHandler> Handler for JsonHandlerType<T> {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: Rc<Option<Uuid>>,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let response = match self.0.handle(
            uri,
            body,
            controller_id,
            event_subscriptions,
            config,
            database,
            accessory_list,
            event_emitter,
        ) {
            Ok(res) => res,
            Err(e) => match e.cause() {
                // TODO - explore the error cause further
                _ => status_response(StatusCode::InternalServerError),
            },
        };
        Box::new(future::ok(response))
    }
}
