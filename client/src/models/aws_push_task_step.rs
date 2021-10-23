/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// AwsPushTaskStep : Push task step for an AWS integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsPushTaskStep {
    #[serde(rename = "url")]
    pub url: String,
    /// Unique identifier for a task step.
    #[serde(rename = "id")]
    pub id: String,
    /// The operation performed, if any.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<crate::models::OperationEnum>>,
    /// Indicates if the operation was successful.
    #[serde(rename = "success")]
    pub success: bool,
    /// The fully-qualified name (FQN) this of the value that was changed.
    #[serde(rename = "fqn", skip_serializing_if = "Option::is_none")]
    pub fqn: Option<String>,
    /// The environment of the value this step pushed.
    #[serde(rename = "environment")]
    pub environment: Option<String>,
    /// The parameter this step pushed.
    #[serde(rename = "parameter")]
    pub parameter: Option<String>,
    /// The integration-native id for the resource.
    #[serde(rename = "venue_id", skip_serializing_if = "Option::is_none")]
    pub venue_id: Option<String>,
    /// The name of the item or resource as known by the integration.
    #[serde(rename = "venue_name", skip_serializing_if = "Option::is_none")]
    pub venue_name: Option<String>,
    /// An error code, if not successful.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Details on the error from the integration.
    #[serde(rename = "error_detail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl AwsPushTaskStep {
    /// Push task step for an AWS integration.
    pub fn new(
        url: String,
        id: String,
        success: bool,
        environment: Option<String>,
        parameter: Option<String>,
        created_at: String,
        modified_at: String,
    ) -> AwsPushTaskStep {
        AwsPushTaskStep {
            url,
            id,
            operation: None,
            success,
            fqn: None,
            environment,
            parameter,
            venue_id: None,
            venue_name: None,
            error_code: None,
            error_detail: None,
            created_at,
            modified_at,
        }
    }
}
