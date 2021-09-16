/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// Tag : The details of a tag.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "url")]
    pub url: String,
    /// A unique identifier for the tag.
    #[serde(rename = "id")]
    pub id: String,
    /// The tag name. Tag names may contain alphanumeric, hyphen, underscore, or period characters. Tag names are case sensitive. The name cannot be modified.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the tag.  You may find it helpful to document how this tag is used to assist others when they need to maintain software that uses this content.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The point in time this tag represents.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "usage")]
    pub usage: Option<Box<crate::models::TagReadUsage>>,
}

impl Tag {
    /// The details of a tag.
    pub fn new(
        url: String,
        id: String,
        name: String,
        timestamp: String,
        usage: Option<crate::models::TagReadUsage>,
    ) -> Tag {
        Tag {
            url,
            id,
            name,
            description: None,
            timestamp,
            usage: usage.map(Box::new),
        }
    }
}
