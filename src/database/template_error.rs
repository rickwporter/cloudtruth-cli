use cloudtruth_restapi::models::TemplateLookupError;
use std::error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TemplateError {
    Authentication(String),
    EvaluateFailed(TemplateLookupError),
    UnhandledError(String),
    ResponseError(String),
    NotFound(String, String),
    EnvironmentMissing(String, String),
}

pub fn template_eval_errors(tle: &TemplateLookupError) -> String {
    let mut failures: Vec<String> = tle
        .detail
        .iter()
        .map(|e| format!("{}: {}", e.parameter_name, e.error_detail))
        .collect();
    if failures.is_empty() {
        failures.push("No details available".to_string());
    }
    let prefix = "\n  ";
    format!("{}{}", prefix, failures.join(prefix))
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::Authentication(msg) => write!(f, "Not Authenticated: {}", msg),
            TemplateError::ResponseError(msg) => write!(f, "{}", msg),
            TemplateError::UnhandledError(msg) => write!(f, "{}", msg),
            TemplateError::EvaluateFailed(tle) => {
                write!(f, "Evaluation failed:{}", template_eval_errors(tle))
            }
            TemplateError::NotFound(template, project) => write!(
                f,
                "No template '{}' found in project '{}'",
                template, project
            ),
            TemplateError::EnvironmentMissing(env_name, condition) => {
                write!(f, "Did not find environment '{}'{}", env_name, condition)
            }
        }
    }
}

impl error::Error for TemplateError {}
