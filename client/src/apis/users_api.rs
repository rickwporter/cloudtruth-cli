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

/// struct for typed errors of method `users_destroy`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersDestroyError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_retrieve`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersRetrieveError {
    UnknownValue(serde_json::Value),
}

/// ### Description ###  Delete the specified user.  This removes all access the User may have to any Organization.  ### Pre-Conditions ###  - The user cannot be the only owner of any Organization. - The bearer token must belong to the user being deleted. - All of the memberships related to the User will be deleted, so all the membership deletion pre-conditions must also be met.
pub fn users_destroy(
    configuration: &mut configuration::Configuration,
    id: &str,
) -> Result<(), Error<UsersDestroyError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/users/{id}/",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

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
    if let Some(ref local_var_cookie) = configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::COOKIE, local_var_cookie);
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;
    if configuration.cookie.is_none() {
        if let Some(local_var_header) = local_var_resp.headers().get(reqwest::header::SET_COOKIE) {
            configuration.cookie = Some(local_var_header.to_str().unwrap().to_string());
        }
    }

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UsersDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn users_list(
    configuration: &mut configuration::Configuration,
    page: Option<i32>,
    page_size: Option<i32>,
    _type: Option<&str>,
) -> Result<crate::models::PaginatedUserList, Error<UsersListError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/users/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
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
    if let Some(ref local_var_cookie) = configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::COOKIE, local_var_cookie);
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;
    if configuration.cookie.is_none() {
        if let Some(local_var_header) = local_var_resp.headers().get(reqwest::header::SET_COOKIE) {
            configuration.cookie = Some(local_var_header.to_str().unwrap().to_string());
        }
    }

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn users_retrieve(
    configuration: &mut configuration::Configuration,
    id: &str,
) -> Result<crate::models::User, Error<UsersRetrieveError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/users/{id}/",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
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
    if let Some(ref local_var_cookie) = configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::COOKIE, local_var_cookie);
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;
    if configuration.cookie.is_none() {
        if let Some(local_var_header) = local_var_resp.headers().get(reqwest::header::SET_COOKIE) {
            configuration.cookie = Some(local_var_header.to_str().unwrap().to_string());
        }
    }

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
