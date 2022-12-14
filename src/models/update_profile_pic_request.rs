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
pub struct UpdateProfilePicRequest {
    /// Image file
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
}

impl UpdateProfilePicRequest {
    pub fn new(file: std::path::PathBuf) -> UpdateProfilePicRequest {
        UpdateProfilePicRequest {
            file,
        }
    }
}


