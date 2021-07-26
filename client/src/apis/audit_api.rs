/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `audit_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `audit_retrieve`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `audit_summary_retrieve`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditSummaryRetrieveError {
    UnknownValue(serde_json::Value),
}

/// A searchable log of all the actions taken by users and service accounts within the organization.
pub fn audit_list(
    configuration: &configuration::Configuration,
    action: Option<&str>,
    earliest: Option<String>,
    latest: Option<String>,
    object_id: Option<&str>,
    object_type: Option<&str>,
    page: Option<i32>,
    user_id: Option<&str>,
) -> Result<crate::models::PaginatedAuditTrailList, Error<AuditListError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/audit/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = action {
        local_var_req_builder =
            local_var_req_builder.query(&[("action", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = earliest {
        local_var_req_builder =
            local_var_req_builder.query(&[("earliest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = latest {
        local_var_req_builder =
            local_var_req_builder.query(&[("latest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("object_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("object_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("user_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuditListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve one record from the audit log.
pub fn audit_retrieve(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<crate::models::AuditTrail, Error<AuditRetrieveError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/audit/{id}/", configuration.base_path, id = id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuditRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Summary information about the organization's audit trail.
pub fn audit_summary_retrieve(
    configuration: &configuration::Configuration,
) -> Result<crate::models::AuditTrailSummary, Error<AuditSummaryRetrieveError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/audit/summary/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuditSummaryRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
