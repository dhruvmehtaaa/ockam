use crate::OneTimeCode;
use anyhow::anyhow;
use minicbor::Decoder;
use ockam::identity::credential::Credential;
use ockam::{Context, Result};
use ockam_core::api::{Request, Response};
use ockam_core::errcode::{Kind, Origin};
use ockam_core::{Error, Route};

/// Using a one-time token created for an identity enrolled with some specific attributes
/// retrieve its credentials from a central authority
pub async fn get_credentials(ctx: &Context, authority_route: Route, token: OneTimeCode) -> Result<Credential<'static>> {
    let request = Request::post("/credential").body(token);
    let mut buf = Vec::new();
    request.encode(&mut buf)?;

    let response_bytes: Vec<u8> = ctx.send_and_receive(authority_route.clone(), buf).await?;
    let mut d = Decoder::new(&response_bytes);
    let _: Response = d.decode()?;
    let credentials: Credential = d.decode().map_err(|e| error(format!("{e}")))?;
    Ok(credentials.to_owned())
}

fn error(message: String) -> Error {
    Error::new(Origin::Application, Kind::Invalid, anyhow!(message))
}
