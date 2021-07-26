/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeTypeEnum {
    #[serde(rename = "directory")]
    Directory,
    #[serde(rename = "file")]
    File,
}

impl ToString for NodeTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Directory => String::from("directory"),
            Self::File => String::from("file"),
        }
    }
}
