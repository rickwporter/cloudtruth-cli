/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use std::time::Instant;

use super::{configuration, Error};
use crate::apis::{handle_serde_error, ResponseContent};

/// struct for typed errors of method [`users_current_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersCurrentRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`users_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersDestroyError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`users_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`users_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersRetrieveError {
    UnknownValue(serde_json::Value),
}

/// Get user information about the current user.
pub fn users_current_retrieve(
    configuration: &configuration::Configuration,
) -> Result<crate::models::User, Error<UsersCurrentRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/users/current/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<UsersCurrentRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}

/// ### Description ###  Delete the specified user.  This removes all access the User may have to any Organization.  ### Pre-Conditions ###  - The user cannot be the only owner of any Organization. - The bearer token must belong to the user being deleted. - All of the memberships related to the User will be deleted, so all the membership deletion pre-conditions must also be met.
pub fn users_destroy(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<UsersDestroyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/users/{id}/",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        Ok(())
    } else {
        let local_var_entity: Option<UsersDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn users_list(
    configuration: &configuration::Configuration,
    ordering: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    _type: Option<&str>,
) -> Result<crate::models::PaginatedUserList, Error<UsersListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/users/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ordering {
        local_var_req_builder =
            local_var_req_builder.query(&[("ordering", &local_var_str.to_string())]);
    }
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
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<UsersListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn users_retrieve(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<crate::models::User, Error<UsersRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/users/{id}/",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<UsersRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}
