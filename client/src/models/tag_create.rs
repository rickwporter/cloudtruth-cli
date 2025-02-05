/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// TagCreate : Details for creating a tag.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TagCreate {
    /// The tag name. Tag names may contain alphanumeric, hyphen, underscore, or period characters. Tag names are case sensitive. The name cannot be modified.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the tag.  You may find it helpful to document how this tag is used to assist others when they need to maintain software that uses this content.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The point in time this tag represents. If not specified then the current time will be used.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl TagCreate {
    /// Details for creating a tag.
    pub fn new(name: String) -> TagCreate {
        TagCreate {
            name,
            description: None,
            timestamp: None,
        }
    }
}
