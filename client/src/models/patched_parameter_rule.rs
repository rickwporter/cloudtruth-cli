/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// PatchedParameterRule : A type of `ModelSerializer` that uses hyperlinked relationships with compound keys instead of primary key relationships.  Specifically:  * A 'url' field is included instead of the 'id' field. * Relationships to other instances are hyperlinks, instead of primary keys.  NOTE: this only works with DRF 3.1.0 and above.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchedParameterRule {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The parameter this rule is for.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ParameterRuleTypeEnum>,
    #[serde(rename = "constraint", skip_serializing_if = "Option::is_none")]
    pub constraint: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl PatchedParameterRule {
    /// A type of `ModelSerializer` that uses hyperlinked relationships with compound keys instead of primary key relationships.  Specifically:  * A 'url' field is included instead of the 'id' field. * Relationships to other instances are hyperlinks, instead of primary keys.  NOTE: this only works with DRF 3.1.0 and above.
    pub fn new() -> PatchedParameterRule {
        PatchedParameterRule {
            url: None,
            id: None,
            parameter: None,
            _type: None,
            constraint: None,
            created_at: None,
            modified_at: None,
        }
    }
}
