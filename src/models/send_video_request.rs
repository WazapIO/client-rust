/*
 * WhatsAPI Go
 *
 * The V2 of WhatsAPI Go
 *
 * The version of the OpenAPI document: 2.0
 * Contact: manjit@sponsorbook.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendVideoRequest {
    /// Video file
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
}

impl SendVideoRequest {
    pub fn new(file: std::path::PathBuf) -> SendVideoRequest {
        SendVideoRequest {
            file,
        }
    }
}

