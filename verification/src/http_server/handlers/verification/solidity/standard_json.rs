use std::str::FromStr;

use actix_web::{error, Error};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};

use crate::{
    compiler::{CompilerVersion, Compilers},
    http_server::handlers::verification::{
        solidity::{
            contract_verifier::{compile_and_verify_handler, Input},
            types::StandardJson,
        },
        VerificationResponse,
    },
    solidity::CompilerFetcher,
};

use super::types::VerificationRequest;

#[api_v2_operation]
pub async fn verify(
    compilers: web::Data<Compilers<CompilerFetcher>>,
    params: Json<VerificationRequest<StandardJson>>,
) -> Result<Json<VerificationResponse>, Error> {
    let params = params.into_inner();

    let compiler_input = params.content.into();
    let compiler_version =
        CompilerVersion::from_str(&params.compiler_version).map_err(error::ErrorBadRequest)?;
    let input = Input {
        compiler_version,
        compiler_input,
        creation_tx_input: &params.creation_bytecode,
        deployed_bytecode: &params.deployed_bytecode,
    };
    compile_and_verify_handler(&compilers, input, false)
        .await
        .map(Json)
}
