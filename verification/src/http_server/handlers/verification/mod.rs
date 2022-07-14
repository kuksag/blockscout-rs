#![allow(dead_code)]

use std::{collections::BTreeMap, fmt::Display};

use ethers_solc::CompilerInput;
use paperclip::{
    actix::Apiv2Schema,
    v2::{models::DefaultSchemaRaw, schema::Apiv2Schema},
};
use serde::{Deserialize, Serialize};

use crate::{compiler::CompilerVersion, solidity::VerificationSuccess, DisplayBytes};

pub mod solidity;
pub mod sourcify;

#[derive(Debug, Deserialize, Serialize, PartialEq, Apiv2Schema)]
pub struct VerificationResponse {
    pub message: String,
    pub result: Option<VerificationResult>,
    pub status: VerificationStatus,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct VerificationResult {
    pub file_name: String,
    pub contract_name: String,
    pub compiler_version: String,
    pub evm_version: String,
    pub constructor_arguments: Option<DisplayBytes>,
    pub optimization: Option<bool>,
    pub optimization_runs: Option<usize>,
    pub contract_libraries: BTreeMap<String, String>,
    pub abi: String,
    pub sources: BTreeMap<String, String>,
}

// We have to "impl Apiv2Schema" instead of "derive Apiv2Schema" because there is no default impl for DisplayBytes
impl Apiv2Schema for VerificationResult {
    fn name() -> Option<String> {
        Some("VerificationResult".to_string())
    }

    fn description() -> &'static str {
        "Verification result"
    }

    fn required() -> bool {
        false
    }

    fn raw_schema() -> DefaultSchemaRaw {
        let mut schema = DefaultSchemaRaw::default();
        schema.example = Some(
            serde_json::to_string(&VerificationResult {
                file_name: "contract.sol".to_string(),
                contract_name: "Contract".to_string(),
                compiler_version: "0.5.0".to_string(),
                evm_version: "byzantium".to_string(),
                constructor_arguments: Some(DisplayBytes::from(vec![0x01, 0x02, 0x03])),
                optimization: Some(true),
                optimization_runs: Some(1),
                contract_libraries: BTreeMap::new(),
                abi: "".to_string(),
                sources: BTreeMap::new(),
            })
            .unwrap(),
        );
        schema
            .properties
            .insert("file_name".to_string(), String::raw_schema().into());
        schema
            .properties
            .insert("contract_name".to_string(), String::raw_schema().into());
        schema
            .properties
            .insert("compiler_version".to_string(), String::raw_schema().into());
        schema
            .properties
            .insert("evm_version".to_string(), String::raw_schema().into());
        schema.properties.insert(
            "constructor_arguments".to_string(),
            String::raw_schema().into(),
        );
        schema
            .properties
            .insert("optimization".to_string(), bool::raw_schema().into());
        schema
            .properties
            .insert("optimization_runs".to_string(), usize::raw_schema().into());
        schema.properties.insert(
            "contract_libraries".to_string(),
            BTreeMap::<String, String>::raw_schema().into(),
        );
        schema
            .properties
            .insert("abi".to_string(), String::raw_schema().into());
        schema.properties.insert(
            "sources".to_string(),
            BTreeMap::<String, String>::raw_schema().into(),
        );

        schema
    }
}

impl From<(CompilerInput, CompilerVersion, VerificationSuccess)> for VerificationResult {
    fn from(
        (compiler_input, compiler_version, verification_success): (
            CompilerInput,
            CompilerVersion,
            VerificationSuccess,
        ),
    ) -> Self {
        VerificationResult {
            file_name: verification_success.file_path,
            contract_name: verification_success.contract_name,
            compiler_version: compiler_version.to_string(),
            evm_version: compiler_input
                .settings
                .evm_version
                .map(|v| v.to_string())
                .unwrap_or_else(|| "default".to_string()),
            constructor_arguments: verification_success.constructor_args,
            optimization: compiler_input.settings.optimizer.enabled,
            optimization_runs: compiler_input.settings.optimizer.runs,
            contract_libraries: compiler_input
                .settings
                .libraries
                .libs
                .into_iter()
                .flat_map(|(_path, libs)| libs)
                .collect(),
            abi: serde_json::to_string(&verification_success.abi)
                .expect("Is result of local compilation and, thus, should be always valid"),
            sources: compiler_input
                .sources
                .into_iter()
                .map(|(path, source)| (path.to_string_lossy().to_string(), source.content))
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Apiv2Schema)]
pub enum VerificationStatus {
    #[serde(rename = "0")]
    Ok,
    #[serde(rename = "1")]
    Failed,
}

impl VerificationResponse {
    pub fn ok(result: VerificationResult) -> Self {
        Self {
            message: "OK".to_string(),
            result: Some(result),
            status: VerificationStatus::Ok,
        }
    }

    pub fn err(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            result: None,
            status: VerificationStatus::Failed,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::tests::parse::test_serialize_json_ok;

    use super::*;

    #[test]
    fn parse_response() {
        test_serialize_json_ok(vec![
            (
                VerificationResponse::ok(VerificationResult {
                    file_name: "File.sol".to_string(),
                    contract_name: "contract_name".to_string(),
                    compiler_version: "compiler_version".to_string(),
                    evm_version: "evm_version".to_string(),
                    constructor_arguments: Some(DisplayBytes::from([0xca, 0xfe])),
                    optimization: Some(false),
                    optimization_runs: Some(200),
                    contract_libraries: BTreeMap::from([(
                        "some_library".into(),
                        "some_address".into(),
                    )]),
                    abi: "abi".to_string(),
                    sources: serde_json::from_str(
                        r#"{
                            "source.sol": "content"
                        }"#,
                    )
                    .unwrap(),
                }),
                json!({
                    "message": "OK",
                    "status": "0",
                    "result": {
                        "file_name": "File.sol",
                        "contract_name": "contract_name",
                        "compiler_version": "compiler_version",
                        "evm_version": "evm_version",
                        "constructor_arguments": "0xcafe",
                        "contract_libraries": {
                            "some_library": "some_address",
                        },
                        "optimization": false,
                        "optimization_runs": 200,
                        "abi": "abi",
                        "sources": {
                            "source.sol": "content",
                        },
                    },

                }),
            ),
            (
                VerificationResponse::err("Parse error"),
                json!({
                    "message": "Parse error",
                    "status": "1",
                    "result": null,
                }),
            ),
        ])
    }
}
