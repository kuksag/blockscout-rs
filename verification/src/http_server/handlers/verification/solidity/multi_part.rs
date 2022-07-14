use super::types::{MultiPartFiles, VerificationRequest};
use crate::{
    compiler::{CompilerVersion, Compilers},
    http_server::handlers::verification::{
        solidity::contract_verifier::{compile_and_verify_handler, Input},
        VerificationResponse,
    },
    solidity::CompilerFetcher,
};
use actix_web::{
    error,
    web::{self, Json},
    Error,
};
use std::str::FromStr;


#[utoipa::path(
    post,
    path = "/api/v1/solidity/verify/multiple-files",
    request_body(content = MultiPartFiles, content_type = "application/json", description = "Description for request body"),
    responses(
        (
            status = 200,
            description = "Verification successful",
            body = VerificationResponse,
            example = json!({
                "status": "success",
                "result": {
                    "contracts": {
                        "contract1": {
                            "abi": "...",
                            "bytecode": "...",
                            "compiler": "solc",
                            "compiler_version": "0.5.0",
                            "optimized": false,
                            "source": "...",
                            "source_map": "...",
                            "source_map_url": "...",
                            "source_url": "...",
                            "version": "0.5.0",
                        },
                        "contract2": {
                            "abi": "...",
                            "bytecode": "...",
                            "compiler": "solc",
                            "compiler_version": "0.5.0",
                            "optimized": false,
                            "source": "...",
                            "source_map": "...",
                            "source_map_url": "...",
                            "source_url": "...",
                            "version": "0.5.0",
                        },
                    },
                    "errors": [
                        {
                            "contract": "contract1",
                            "error": "...",
                            "file": "...",
                            "line": 0,
                            "location": "...",
                            "severity": "error",
                            "title": "...",
                        },
                        {
                            "contract": "contract2",
                            "error": "...",
                            "file": "...",
                            "line": 0,
                            "location": "...",
                            "severity": "error",
                            "title": "...",
                        },
                    ],
                    "metadata": {
                        "compiler": "solc",
                        "compiler_version": "0.5.0",
                        "language": "Solidity",
                        "language_version": "0.5.0",
                        "optimized": false,
                        "version": "0.5.0",
                    },
                }
            }),
        ),
    ),
    tag = "solidity"
)]
pub async fn verify(
    compilers: web::Data<Compilers<CompilerFetcher>>,
    params: Json<VerificationRequest<MultiPartFiles>>,
) -> Result<Json<VerificationResponse>, Error> {
    let params = params.into_inner();

    let compiler_input = params.content.try_into().map_err(error::ErrorBadRequest)?;
    let compiler_version =
        CompilerVersion::from_str(&params.compiler_version).map_err(error::ErrorBadRequest)?;
    let input = Input {
        compiler_version,
        compiler_input,
        creation_tx_input: &params.creation_bytecode,
        deployed_bytecode: &params.deployed_bytecode,
    };
    compile_and_verify_handler(&compilers, input, true)
        .await
        .map(Json)
}
