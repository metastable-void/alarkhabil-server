
#![feature(impl_trait_in_assoc_type)]

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use serde::{Serialize, Deserialize};

use hyper::{Body, Request, Response, StatusCode};

use std::{future::Future, marker::PhantomData};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMessage {
    algo: String,

    #[serde(with="base64")]
    pubk: Vec<u8>,

    #[serde(with="base64")]
    sig: Vec<u8>,

    #[serde(with="base64")]
    msg: Vec<u8>,
}

pub struct PrivateKey {
    algo: String,
    key: Vec<u8>,
}

impl PrivateKey {
    pub fn new(algo: &str) -> Result<PrivateKey, anyhow::Error> {
        if algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        let secret_key: [u8; 32] = rand::random();

        Ok(PrivateKey {
            algo: algo.to_string(),
            key: secret_key.to_vec(),
        })
    }

    pub fn algo(&self) -> &str {
        &self.algo
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }
}

impl SignedMessage {
    pub fn create(secret_key: PrivateKey, msg: &[u8]) -> Result<SignedMessage, anyhow::Error> {
        let algo = secret_key.algo();
        let secret_key = secret_key.key();

        if algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        let secret_key: SigningKey = secret_key.try_into().map_err(|_| anyhow::anyhow!("Invalid secret key length"))?;
        let public_key = secret_key.verifying_key();
        let signature = secret_key.sign(msg);

        Ok(SignedMessage {
            algo: algo.to_string(),
            pubk: public_key.to_bytes().to_vec(),
            sig: signature.to_bytes().to_vec(),
            msg: msg.to_vec(),
        })
    }

    pub fn algo(&self) -> &str {
        &self.algo
    }

    pub fn verify(&self) -> Result<&[u8], anyhow::Error> {
        if self.algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", self.algo));
        }

        let public_key_bytes: &[u8; 32] = self.pubk.as_slice().try_into().map_err(|_| anyhow::anyhow!("Invalid public key length"))?;
        let public_key = VerifyingKey::from_bytes(public_key_bytes)?;

        let signature = Signature::from_slice(self.sig.as_slice())?;
        public_key.verify_strict(self.msg.as_slice(), &signature)?;
        Ok(&self.msg)
    }
}

mod base64 {
    use serde::{Serialize, Deserialize};
    use serde::{Deserializer, Serializer};
    use base64::{Engine, engine::general_purpose::STANDARD as base64_engine};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64_engine.encode(v);
        String::serialize(&base64, s)
    }
    
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64_engine.decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub fn extract_ed25519_private_key(buf: &[u8]) -> Result<[u8; 32], anyhow::Error> {
    buf.try_into().map_err(|_| anyhow::anyhow!("Invalid length"))
}

// pub struct AppInitialContext {
//     request: Request<Body>,
// }

// pub struct AppErrorContext {
//     request: Request<Body>,
//     error: anyhow::Error,
// }

// pub struct AppResponseContext {
//     request: Request<Body>,
//     response: Response<Body>,
// }

// pub trait AppContext {
//     fn request(&self) -> &Request<Body>;
//     fn request_mut(&mut self) -> &mut Request<Body>;
//     fn has_error(&self) -> bool;
//     fn has_response(&self) -> bool;
// }

// pub struct AppContext {
//     request: Request<Body>,
//     result: Option<Result<Response<Body>, anyhow::Error>>,
// }

// impl AppContext {
//     pub fn new(request: Request<Body>) -> AppContext {
//         AppContext {
//             request,
//             result: None,
//         }
//     }

//     pub fn request(&self) -> &Request<Body> {
//         &self.request
//     }

//     pub fn request_mut(&mut self) -> &mut Request<Body> {
//         &mut self.request
//     }

//     pub fn set_result(&mut self, result: Result<Response<Body>, anyhow::Error>) {
//         self.result = Some(result);
//     }

//     pub fn has_result(&self) -> bool {
//         self.result.is_some()
//     }

//     pub fn has_response(&self) -> bool {
//         self.result.as_ref().map(|r| r.is_ok()).unwrap_or(false)
//     }

//     pub fn has_error(&self) -> bool {
//         self.result.as_ref().map(|r| r.is_err()).unwrap_or(false)
//     }

//     pub fn result(&self) -> Option<&Result<Response<Body>, anyhow::Error>> {
//         self.result.as_ref()
//     }
// }

// pub trait AppRouter {
//     fn handle<'a, Fut, InCtx, OutCtx>(&self, ctx: InCtx) -> Fut
//     where
//         InCtx: AppContext,
//         OutCtx: AppContext,
//         Fut: Future<Output = OutCtx> + Send + 'a;
// }

pub trait AppRequestHandler<'a>
{
    type Fut: Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync + 'a;

    fn handle_request(&self, request: &mut Request<Body>) -> Self::Fut;
}

pub trait AppErrorHandler<'a>
{
    type Fut: Future<Output = Response<Body>> + Send + Sync + 'a;

    fn handle_error(&self, error: anyhow::Error, request: Request<Body>) -> Self::Fut;
}

pub(crate) struct DefaultAppRequestHandler;

impl<'a> AppRequestHandler<'a> for DefaultAppRequestHandler {
    type Fut = impl Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync + 'a;
    fn handle_request(&self, _request: &mut Request<Body>) -> Self::Fut {
        async move {
            Ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from("Hello, world!"))
                    .unwrap()
            )
        }
    }
}

pub struct App<'req, 'err, ReqHandler, ErrHandler>
where
    ReqHandler: AppRequestHandler<'req>,
    ErrHandler: AppErrorHandler<'err>,
{
    request_handler: ReqHandler,
    error_handler: ErrHandler,
    phantom: PhantomData<(&'req (), &'err ())>,
}

pub struct AppBuilder<'req, 'err, ReqHandler, ErrHandler>
where
    ReqHandler: AppRequestHandler<'req>,
    ErrHandler: AppErrorHandler<'err>,
{
    request_handler: Option<ReqHandler>,
    error_handler: Option<ErrHandler>,
    phantom: PhantomData<(&'req (), &'err ())>,
}

impl<'req, 'err, ReqHandler, ErrHandler> AppBuilder<'req, 'err, ReqHandler, ErrHandler>
where
    ReqHandler: AppRequestHandler<'req>,
    ErrHandler: AppErrorHandler<'err>,
{
    pub fn new() -> AppBuilder<'req, 'err, ReqHandler, ErrHandler> {
        AppBuilder {
            request_handler: None,
            error_handler: None,
            phantom: PhantomData,
        }
    }

    pub fn request_handler(mut self, request_handler: ReqHandler) -> AppBuilder<'req, 'err, ReqHandler, ErrHandler> {
        self.request_handler = Some(request_handler);
        self
    }

    pub fn error_handler(mut self, error_handler: ErrHandler) -> AppBuilder<'req, 'err, ReqHandler, ErrHandler> {
        self.error_handler = Some(error_handler);
        self
    }

    pub fn build(self) -> App<'req, 'err, ReqHandler, ErrHandler> {
        let request_handler = self.request_handler.unwrap_or(
            DefaultAppRequestHandler
        );
        App {
            request_handler: self.request_handler.unwrap(),
            error_handler: self.error_handler.unwrap(),
            phantom: PhantomData,
        }
    }
}


