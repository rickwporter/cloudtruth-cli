use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    Arg, ArgMatches, Shell, SubCommand,
};

pub const ADD_USER_OPT: &str = "username-to-add";
pub const API_KEY_OPT: &str = "api_key";
pub const AS_OF_ARG: &str = "datetime|tag";
pub const CONFIRM_FLAG: &str = "confirm";
pub const DESCRIPTION_OPT: &str = "description";
pub const ENV_NAME_ARG: &str = "env-name";
pub const ENV_NAME_OPT: &str = "env";
pub const FORMAT_OPT: &str = "format";
pub const IMMEDIATE_PARAMETERS_FLAG: &str = "immediate-parameters";
pub const INTEGRATION_NAME_ARG: &str = "integration-name";
pub const INVITE_NAME_ARG: &str = "e-mail";
pub const JMES_PATH_ARG: &str = "JMES";
pub const KEY_ARG: &str = "KEY";
pub const NAME_ARG: &str = "NAME";
pub const PARENT_ARG: &str = "parent";
pub const PROJECT_NAME_OPT: &str = "project";
pub const PULL_NAME_ARG: &str = "import-name";
pub const PUSH_NAME_ARG: &str = "push-name";
pub const RAW_FLAG: &str = "raw";
pub const RENAME_OPT: &str = "new-name";
pub const RM_USER_OPT: &str = "username-to-remove";
pub const ROLE_ARG: &str = "role";
pub const RULE_MAX_ARG: &str = "MAX";
pub const RULE_MIN_ARG: &str = "MIN";
pub const RULE_MAX_LEN_ARG: &str = "MAX-LEN";
pub const RULE_MIN_LEN_ARG: &str = "MIN-LEN";
pub const RULE_REGEX_ARG: &str = "REGEX";
pub const RULE_NO_MAX_ARG: &str = "NO-MAX";
pub const RULE_NO_MIN_ARG: &str = "NO-MIN";
pub const RULE_NO_MAX_LEN_ARG: &str = "NO-MAX-LEN";
pub const RULE_NO_MIN_LEN_ARG: &str = "NO-MIN-LEN";
pub const RULE_NO_REGEX_ARG: &str = "NO-REGEX";
pub const SHOW_TIMES_FLAG: &str = "show-time";
pub const SECRETS_FLAG: &str = "secrets";
pub const TAG_NAME_OPT: &str = "tag";
pub const TAG_NAME_ARG: &str = "tag-name";
pub const TEMPLATE_FILE_OPT: &str = "FILE";
pub const VALUES_FLAG: &str = "values";

pub const DELETE_SUBCMD: &str = "delete";
pub const DIFF_SUBCMD: &str = "differences";
pub const EDIT_SUBCMD: &str = "edit";
pub const GET_SUBCMD: &str = "get";
pub const HISTORY_SUBCMD: &str = "history";
pub const IMPORT_SUBCMD: &str = "imports";
pub const LIST_SUBCMD: &str = "list";
pub const PUSH_SUBCMD: &str = "pushes";
pub const SET_SUBCMD: &str = "set";
pub const SYNC_SUBCMD: &str = "sync";
pub const TAG_SUBCMD: &str = "tag";
pub const TASK_STEPS_SUBCMD: &str = "task-steps";
pub const TASKS_SUBCMD: &str = "tasks";
pub const TREE_SUBCMD: &str = "tree";

const TRUE_FALSE_VALUES: &[&str] = &["true", "false"];

const DELETE_ALIASES: &[&str] = &["del", "d"];
const DIFF_ALIASES: &[&str] = &["difference", "differ", "diff", "di"];
const EDIT_ALIASES: &[&str] = &["ed", "e"];
const HISTORY_ALIASES: &[&str] = &["hist", "h"];
const IMPORT_ALIASES: &[&str] = &["import", "imp", "im", "i"];
const LIST_ALIASES: &[&str] = &["ls", "l"];
const PUSH_ALIASES: &[&str] = &["push", "pu", "p"];
const SET_ALIASES: &[&str] = &["s"];
const SYNC_ALIASES: &[&str] = &["syn", "sy"];
const TASKS_ALIASES: &[&str] = &["task", "ta", "t"];
const TASK_STEPS_ALIASES: &[&str] = &["steps", "step", "st", "ts"];
const TREE_ALIASES: &[&str] = &["tr"];

const REGION_VALUES: &[&str] = &[
    "af-south-1",
    "ap-east-1",
    "ap-northeast-1",
    "ap-northeast-2",
    "ap-northeast-3",
    "ap-south-1",
    "ap-southeast-1",
    "ap-southeast-2",
    "ca-central-1",
    "cn-north-1",
    "cn-northwest-1",
    "eu-central-1",
    "eu-north-1",
    "eu-south-1",
    "eu-west-1",
    "eu-west-2",
    "eu-west-3",
    "me-south-1",
    "sa-east-1",
    "us-east-1",
    "us-east-2",
    "us-west-1",
    "us-west-2",
];

pub fn binary_name() -> String {
    option_env!("CARGO_PKG_NAME")
        .unwrap_or("cloudtruth")
        .to_string()
}

pub fn true_false_option(input: Option<&str>) -> Option<bool> {
    match input {
        Some("true") => Some(true),
        Some("false") => Some(false),
        _ => None,
    }
}

pub fn opposing_flags(args: &ArgMatches, true_flag: &str, false_flag: &str) -> Option<bool> {
    if args.is_present(true_flag) {
        Some(true)
    } else if args.is_present(false_flag) {
        Some(false)
    } else {
        None
    }
}

/// Checks for standard flags that would cause us to show the values (in some form).
///
/// The `occurances_of(FORMAT_OPT)` is another means of checking whether a format value is
/// provided as an argument, even though there is a default value.
pub fn show_values(args: &ArgMatches) -> bool {
    args.is_present(VALUES_FLAG)
        || args.is_present(SHOW_TIMES_FLAG)
        || args.is_present(SECRETS_FLAG)
        || args.occurrences_of(FORMAT_OPT) > 0
}

fn table_format_options() -> Arg<'static, 'static> {
    Arg::with_name(FORMAT_OPT)
        .short("f")
        .long(FORMAT_OPT)
        .takes_value(true)
        .default_value("table")
        .possible_values(&["table", "csv", "json", "yaml"])
}

fn values_flag() -> Arg<'static, 'static> {
    Arg::with_name(VALUES_FLAG).short("v").long(VALUES_FLAG)
}

fn secrets_display_flag() -> Arg<'static, 'static> {
    Arg::with_name(SECRETS_FLAG).short("s").long(SECRETS_FLAG)
}

fn immediate_parameters_flag() -> Arg<'static, 'static> {
    Arg::with_name(IMMEDIATE_PARAMETERS_FLAG)
        .short("i")
        .long("immediate_parameters")
        .help("Show only immediate parameters (no inherited parameters)")
}

fn confirm_flag() -> Arg<'static, 'static> {
    Arg::with_name(CONFIRM_FLAG)
        .alias(CONFIRM_FLAG)
        .short("y")
        .long("yes")
        .help("Avoid confirmation prompt(s)")
}

fn rename_option() -> Arg<'static, 'static> {
    Arg::with_name(RENAME_OPT)
        .short("r")
        .long("rename")
        .takes_value(true)
}

fn description_option() -> Arg<'static, 'static> {
    Arg::with_name(DESCRIPTION_OPT)
        .short("d")
        .long("desc")
        .takes_value(true)
}

fn template_body() -> Arg<'static, 'static> {
    Arg::with_name(TEMPLATE_FILE_OPT).help("File containing the template")
}

fn name_arg() -> Arg<'static, 'static> {
    Arg::with_name(NAME_ARG).required(true).index(1)
}

fn key_arg() -> Arg<'static, 'static> {
    Arg::with_name(KEY_ARG).required(true).index(1)
}

fn as_of_arg() -> Arg<'static, 'static> {
    Arg::with_name(AS_OF_ARG).long("as-of").takes_value(true)
}

fn param_as_of_arg() -> Arg<'static, 'static> {
    as_of_arg().help("Date/time (or tag) of parameter value(s)")
}

fn show_times_arg() -> Arg<'static, 'static> {
    Arg::with_name(SHOW_TIMES_FLAG)
        .long("show-times")
        .takes_value(false)
        .help("Show create and modified times.")
}

fn env_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(ENV_NAME_ARG)
        .takes_value(true)
        .required(true)
        .index(1)
        .help("Environment name")
}

fn env_name_opt() -> Arg<'static, 'static> {
    Arg::with_name(ENV_NAME_OPT)
        .takes_value(true)
        .long("env")
        .short("e")
        .help("Filter by environment name")
}

fn multi_env_name_arg() -> Arg<'static, 'static> {
    Arg::with_name("ENV")
        .short("e")
        .long("env")
        .takes_value(true)
        .multiple(true)
        .help("Up to two environment(s) to be compared.")
}

fn tag_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(TAG_NAME_ARG)
        .takes_value(true)
        .required(true)
        .index(2)
        .help("Tag name")
}

fn tag_name_opt() -> Arg<'static, 'static> {
    Arg::with_name(TAG_NAME_OPT)
        .takes_value(true)
        .long("tag")
        .short("t")
        .help("Filter by tag name")
}

fn project_name_opt() -> Arg<'static, 'static> {
    Arg::with_name(PROJECT_NAME_OPT)
        .long("project")
        .takes_value(true)
        .short("p")
        .help("Filter by project name")
}

fn api_key_arg() -> Arg<'static, 'static> {
    Arg::with_name(API_KEY_OPT)
        .short("k")
        .long("api-key")
        .help("CloudTruth API key")
        .takes_value(true)
}

fn raw_arg() -> Arg<'static, 'static> {
    Arg::with_name(RAW_FLAG).short("r").long("raw")
}

fn parent_arg() -> Arg<'static, 'static> {
    Arg::with_name(PARENT_ARG)
        .short("p")
        .long("parent")
        .takes_value(true)
}

fn role_arg() -> Arg<'static, 'static> {
    Arg::with_name("role")
        .takes_value(true)
        .long("role")
        .possible_values(&["owner", "admin", "contrib", "viewer"])
}

fn invitation_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(INVITE_NAME_ARG)
        .takes_value(true)
        .index(1)
        .required(true)
        .help("Email address for invitation")
}

fn integration_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(INTEGRATION_NAME_ARG)
        .takes_value(true)
        .index(1)
        .required(true)
        .help("Integration name")
}

fn integration_name_opt() -> Arg<'static, 'static> {
    Arg::with_name(INTEGRATION_NAME_ARG)
        .short("i")
        .long("integration")
        .takes_value(true)
        .value_name("name")
        .help("Integration name")
}

fn pull_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(PULL_NAME_ARG)
        .takes_value(true)
        .index(1)
        .required(true)
        .help("Import name")
}

fn push_name_arg() -> Arg<'static, 'static> {
    Arg::with_name(PUSH_NAME_ARG)
        .takes_value(true)
        .index(1)
        .required(true)
        .help("Push name")
}

fn jmes_path_arg() -> Arg<'static, 'static> {
    Arg::with_name(JMES_PATH_ARG)
        .short("j")
        .long("jmes")
        .takes_value(true)
        .value_name("jmes-path")
        .help("JMES path within FQN for external parameter")
}

fn environment_tag_validator(arg_value: String) -> Result<(), String> {
    let colons = arg_value.matches(':').count();
    match colons {
        1 => Ok(()),
        0 => Err("Use a ':' to separate the environment and tag names".to_string()),
        _ => Err("Can only have one ':' to separate the environment and tag names".to_string()),
    }
}

fn i32_validator(arg_value: String) -> Result<(), String> {
    match arg_value.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn serialized_format_arg() -> Arg<'static, 'static> {
    Arg::with_name(FORMAT_OPT)
        .takes_value(true)
        .short("f")
        .long("format")
        .possible_values(&["yaml", "json"])
        .default_value("yaml")
}

fn schema_format_arg() -> Arg<'static, 'static> {
    serialized_format_arg().help("Schema output format")
}

fn schema_version_arg() -> Arg<'static, 'static> {
    Arg::with_name("version")
        .long("version")
        .help("Display just the schema version")
}

fn rule_max_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_MAX_ARG)
        .long("max")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("Set parameter rule maximum value")
}

fn rule_min_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_MIN_ARG)
        .long("min")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("Set parameter rule minimum value")
}

fn rule_max_len_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_MAX_LEN_ARG)
        .long("max-len")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("Set parameter rule maximum length value")
}

fn rule_min_len_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_MIN_LEN_ARG)
        .long("min-len")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("Set parameter rule minimum length value")
}

fn rule_regex_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_REGEX_ARG)
        .long("regex")
        .takes_value(true)
        .help("Set parameter rule regex value")
}

fn rule_no_max_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_NO_MAX_ARG)
        .long("no-max")
        .help("Remove the parameter rule maximum value")
}

fn rule_no_min_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_NO_MIN_ARG)
        .long("no-min")
        .help("Remove the parameter rule minimum value")
}

fn rule_no_max_len_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_NO_MAX_LEN_ARG)
        .long("no-max-len")
        .help("Remove the parameter rule maximum length value")
}

fn rule_no_min_len_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_NO_MIN_LEN_ARG)
        .long("no-min-len")
        .help("Remove the parameter rule minimum length value")
}

fn rule_no_regex_arg() -> Arg<'static, 'static> {
    Arg::with_name(RULE_NO_REGEX_ARG)
        .long("no-regex")
        .help("Remove the parameter rule regex value")
}

fn push_dry_run_arg() -> Arg<'static, 'static> {
    Arg::with_name("DRY_RUN")
        .long("dry-run")
        .help("Dry-run the push without changing any data")
}

fn push_no_dry_run_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_DRY_RUN")
        .long("no-dry-run")
        .help("Create the push without being a dry-run")
}

fn push_check_owner_arg() -> Arg<'static, 'static> {
    Arg::with_name("CHECK_OWNER")
        .long("no-force")
        .visible_alias("check-owner")
        .help("Make sure CloudTruth is the destination owner ")
}

fn push_no_check_owner_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_CHECK_OWNER")
        .long("force")
        .visible_alias("no-check-owner")
        .help("Allow the push even if CloudTruth is not the destination owner")
}

fn push_local_arg() -> Arg<'static, 'static> {
    Arg::with_name("LOCAL")
        .long("local")
        .help("Push only parameters defined in the selected project(s)")
}

fn push_no_local_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_LOCAL")
        .long("no-local")
        .help("Push all parameters (default)")
}

fn push_coerce_params_arg() -> Arg<'static, 'static> {
    Arg::with_name("COERCE_PARAMS")
        .long("coerce-params")
        .help("Include non-secret CloudTruth parameters, even in a secret store destination")
}

fn push_no_coerce_params_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_COERCE_PARAMS")
        .long("no-coerce-params")
        .help("Do not include non-secret CloudTruth parameters in a secret store destination")
}

fn push_include_params_arg() -> Arg<'static, 'static> {
    Arg::with_name("INCLUDE_PARAMS")
        .long("include-parameters")
        .help("Include non-secret CloudTruth parameters in the values being pushed")
}

fn push_no_include_params_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_INCLUDE_PARAMS")
        .long("no-include-parameters")
        .help("Do not include non-secret CloudTruth parameters in the values being pushed")
}

fn push_include_secrets_arg() -> Arg<'static, 'static> {
    Arg::with_name("INCLUDE_SECRETS")
        .long("include-secrets")
        .help("Include secret CloudTruth parameters in the values being pushed")
}

fn push_no_include_secrets_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_INCLUDE_SECRETS")
        .long("no-include-secrets")
        .help("Do not include secret CloudTruth parameters in the values being pushed")
}

fn push_include_templates_arg() -> Arg<'static, 'static> {
    Arg::with_name("INCLUDE_TEMPLATES")
        .long("include-templates")
        .help("Include templates in the values being pushed.")
}

fn push_no_include_templates_arg() -> Arg<'static, 'static> {
    Arg::with_name("NO_INCLUDE_TEMPLATES")
        .long("no-include-templates")
        .help("Do not include templates in the values being pushed.")
}

fn add_user_option() -> Arg<'static, 'static> {
    Arg::with_name(ADD_USER_OPT)
        .long("add-user")
        .visible_aliases(&["add", "user"])
        .multiple(true)
        .takes_value(true)
        .help("Add user(s) to the group by name")
}

fn remove_user_option() -> Arg<'static, 'static> {
    Arg::with_name(RM_USER_OPT)
        .long("remove-user")
        .visible_aliases(&["remove", "rm", "rm-user"])
        .multiple(true)
        .takes_value(true)
        .help("Remove user(s) from the group by name")
}

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(api_key_arg())
        .arg(
            Arg::with_name("env")
                .short("e")
                .long("env")
                .help("The CloudTruth environment to work with")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("profile")
                .long("profile")
                .help("The configuration profile from the application configuration file to use")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The CloudTruth project to work with")
                .takes_value(true)
        )
        .subcommand(SubCommand::with_name("audit-logs")
            .about("Display audit logs")
            .visible_aliases(&["audit", "aud", "au", "log", "logs"])
            .subcommands(vec![
                SubCommand::with_name(LIST_SUBCMD)
                    .visible_aliases(LIST_ALIASES)
                    .about("List audit log details")
                    // TODO: object name? (API only appears to support ID), user name/email? before/after?
                    .arg(Arg::with_name("action")
                        .takes_value(true)
                        .short("a")
                        .long("action")
                        .possible_values(&["create", "delete", "update", "nothing"])
                        .help("Only show specified action"))
                    .arg(Arg::with_name("object-type")
                        .takes_value(true)
                        .short("t")
                        .long("type")
                        .help("Only show specified object type"))
                    .arg(Arg::with_name("contains")
                        .takes_value(true)
                        .short("n")
                        .long("name")
                        .help("Only show audit entries whose name contains the substring"))
                    .arg(Arg::with_name("max-entries")
                        .takes_value(true)
                        .short("m")
                        .long("max")
                        .default_value("50")
                        .help("Limit the maximum number of entries, 0 for no limit."))
                    .arg(Arg::with_name("before")
                        .long("before")
                        .takes_value(true)
                        .help("Show audit log entries before specified date/time"))
                    .arg(Arg::with_name("after")
                        .long("after")
                        .takes_value(true)
                        .help("Show audit log entries after specified date/time"))
                    .arg(Arg::with_name("username")
                        .short("u")
                        .long("user")
                        .takes_value(true)
                        .help("Show audit log entries only from specified user"))
                    .arg(Arg::with_name("project")
                        .long("project")
                        .takes_value(true)
                        .help("Show audit log entries only from specified project"))
                    .arg(Arg::with_name("environment")
                        .long("env")
                        .takes_value(true)
                        .help("Show audit log entries only from specified environment"))
                    .arg(Arg::with_name("parameter")
                        .long("parameter")
                        .takes_value(true)
                        .help("Show audit log entries only from specified parameter"))
                    .arg(table_format_options().help("Format for audit log details")),
                SubCommand::with_name("summary")
                    .visible_aliases(&["sum"])
                    .about("Display summary of audit logs"),
            ])
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completions for this application")
                .arg(Arg::with_name("SHELL").possible_values(&Shell::variants()).required(true))
        )
        .subcommand(SubCommand::with_name("configuration")
            .visible_aliases(&["config", "conf", "con", "co", "c"])
            .about("Configuration options for this application")
            .subcommands(vec![
                SubCommand::with_name(EDIT_SUBCMD)
                    .visible_aliases(EDIT_ALIASES)
                    .about("Edit your configuration data for this application"),
                SubCommand::with_name("profiles")
                    .visible_aliases(&["profile", "prof", "pr", "p"])
                    .about("Work with CloudTruth CLI profiles")
                    .subcommands(vec![
                        SubCommand::with_name(DELETE_SUBCMD)
                            .visible_aliases(DELETE_ALIASES)
                            .about("Delete specified CLI profile")
                            .arg(name_arg().help("Profile name"))
                            .arg(confirm_flag()),
                        SubCommand::with_name(LIST_SUBCMD)
                            .visible_aliases(LIST_ALIASES)
                            .about("List CLI profiles")
                            .arg(values_flag().help("Display profile information/values"))
                            .arg(table_format_options().help("Display profile value info format"))
                            .arg(secrets_display_flag().help("Display API key values")),
                        SubCommand::with_name(SET_SUBCMD)
                            .visible_aliases(SET_ALIASES)
                            .about("Create/modify CLI profile settings")
                            .arg(name_arg().help("Profile name"))
                            .arg(description_option().help("Profile's description"))
                            .arg(api_key_arg())
                            .arg(Arg::with_name("PROJECT")
                                .short("p")
                                .long("proj")
                                .takes_value(true)
                                .help("Default project for profile (use \"\" to remove)"))
                            .arg(Arg::with_name("ENVIRONMENT")
                                .short("e")
                                .long("env")
                                .takes_value(true)
                                .help("Default environment for profile (use \"\" to remove)"))
                            .arg(Arg::with_name("SOURCE")
                                .long("source")
                                .short("s")
                                .takes_value(true)
                                .help("Source (or parent) profile"))
                    ]),
                SubCommand::with_name("current")
                    .visible_aliases(&["curr", "cur", "c"])
                    .arg(table_format_options().help("Display table format"))
                    .arg(secrets_display_flag().help("Display API key values"))
                    .arg( Arg::with_name("extended")
                        .hidden(true)
                        .short("x").
                        help("Show extended values"))
                    .about("Show the current arguments and their sources.")
                ])
        )
        .subcommand(
            SubCommand::with_name("environments")
                .visible_aliases(&["environment", "envs", "env", "e"])
                .about("Work with CloudTruth environments")
                .subcommands(vec![
                    SubCommand::with_name(DELETE_SUBCMD)
                        .visible_aliases(DELETE_ALIASES)
                        .about("Delete specified CloudTruth environment")
                        .arg(name_arg().help("Environment name"))
                        .arg(confirm_flag()),
                    SubCommand::with_name(LIST_SUBCMD)
                        .visible_aliases(LIST_ALIASES)
                        .about("List CloudTruth environments")
                        .arg(show_times_arg())
                        .arg(values_flag().help("Display environment information/values"))
                        .arg(table_format_options().help("Format for environment values data")),
                    SubCommand::with_name(SET_SUBCMD)
                        .visible_aliases(SET_ALIASES)
                        .about("Create/update a CloudTruth environment")
                        .arg(name_arg().help("Environment name"))
                        .arg(description_option().help("Environment's description"))
                        .arg(rename_option().help("New environment name"))
                        .arg(parent_arg()
                            .help("Environment's parent name (only used for create)")),
                    SubCommand::with_name(TAG_SUBCMD)
                        .visible_aliases(&["ta"])
                        .subcommands(vec![
                            SubCommand::with_name(DELETE_SUBCMD)
                                .visible_aliases(DELETE_ALIASES)
                                .arg(env_name_arg())
                                .arg(tag_name_arg())
                                .arg(confirm_flag())
                                .about("Delete an environment tag value"),
                            SubCommand::with_name(LIST_SUBCMD)
                                .visible_aliases(LIST_ALIASES)
                                .arg(env_name_arg())
                                .arg(Arg::with_name("usage")
                                    .long("usage")
                                    .short("u")
                                    .help("Display tag usage data"))
                                .arg(values_flag().help("Display environment tag information"))
                                .arg(table_format_options().help("Format for environment tag values data"))
                                .about("List CloudTruth environment tags"),
                            SubCommand::with_name(SET_SUBCMD)
                                .visible_aliases(SET_ALIASES)
                                .arg(env_name_arg())
                                .arg(tag_name_arg())
                                .arg(description_option().help("Tag's description"))
                                .arg(rename_option().help("New tag name"))
                                .arg(Arg::with_name("timestamp")
                                    .takes_value(true)
                                    .short("t")
                                    .long("time")
                                    .help("Set the tag's timestamp value"))
                                .arg(Arg::with_name("current")
                                    .short("c")
                                    .long("current")
                                    .help("Update the tag's time to the current time"))
                                .about("Create/update an environment tag"),
                        ])
                        .about("View and manipulate environment tags"),
                    SubCommand::with_name(TREE_SUBCMD)
                        .visible_aliases(TREE_ALIASES)
                        .about("Show a tree representation of the environments")
                        .arg(name_arg()
                            .help("Show this environment with children")
                            .required(false)
                            .default_value("default")),
                ])
        )
        .subcommand(SubCommand::with_name("login")
            .about("Sets up a CloudTruth configuration profile api_key")
            .arg(confirm_flag()))
        .subcommand(SubCommand::with_name("logout")
            .about("Removes a CloudTruth configuration profile api_key")
            .arg(confirm_flag()))
        .subcommand(
            SubCommand::with_name("integrations")
                .visible_aliases(&["integration", "integrate", "integ", "int", "in"])
                .about("Work with CloudTruth integrations")
                .subcommands(vec![
                    SubCommand::with_name("explore")
                        .visible_aliases(&["exp", "ex", "e"])
                        .about("Explore integrations by Fully Qualified Name (FQN).")
                        .arg(Arg::with_name("FQN")
                            .index(1)
                            .takes_value(true)
                            .help("Integration FQN"))
                        .arg(table_format_options().help("Format integration values data."))
                        .arg(values_flag().help("Display integration values"))
                        .arg(jmes_path_arg())
                        .arg(raw_arg().help("Display raw file content (if only one file)"))
                        .arg(secrets_display_flag().help("Display raw values, even if secret")),
                    SubCommand::with_name(GET_SUBCMD)
                        .about("Gets all the information for the specified integration")
                        .arg(integration_name_arg()),
                    SubCommand::with_name(LIST_SUBCMD)
                        .visible_aliases(LIST_ALIASES)
                        .about("List CloudTruth integrations")
                        .arg(show_times_arg())
                        .arg(values_flag().help("Display integration information/values"))
                        .arg(table_format_options().help("Format for integration values data")),
                    SubCommand::with_name("refresh")
                        .visible_aliases(&["ref", "re", "r"])
                        .about("Refresh the integration connection status")
                        .arg(integration_name_arg()),
                ])
        )
        .subcommand(
            SubCommand::with_name("parameters")
                .visible_aliases(&["parameter", "params", "param", "par", "pa", "p"])
                .about("Work with CloudTruth parameters")
                .subcommands(vec![
                    SubCommand::with_name(DELETE_SUBCMD)
                        .visible_aliases(DELETE_ALIASES)
                        .about("Delete the parameter from the project")
                        .arg(confirm_flag())
                        .arg(key_arg().help("Name of parameter to delete")),
                    SubCommand::with_name("drift")
                        .visible_aliases(&["dri", "dr"])
                        .about("Determine drift between current environment and project parameters")
                        .args(&[
                            table_format_options().help("Format for differences"),
                            param_as_of_arg(),
                            secrets_display_flag(),
                            values_flag(),
                        ]),
                    SubCommand::with_name("export")
                        .visible_aliases(&["expo", "exp", "ex"])
                        .about(concat!("Export selected parameters to a known output format. ",
                            "Exported parameters are limited to alphanumeric and underscore  in ",
                            "key names. Formats available are: dotenv, docker, and shell."))
                        .arg(Arg::with_name("contains")
                            .long("contains")
                            .help("Return parameters with keys containing search")
                            .takes_value(true))
                        .arg(Arg::with_name("ends_with")
                            .long("ends-with")
                            .help("Return parameters with keys ending with search")
                            .takes_value(true))
                        .arg(Arg::with_name("export")
                            .long("export")
                            .help("Add 'export' to each declaration"))
                        .arg(Arg::with_name("FORMAT")
                            .required(true)
                            .possible_value("docker")
                            .possible_value("dotenv")
                            .possible_value("shell")
                            .index(1))
                        .arg(param_as_of_arg())
                        .arg(secrets_display_flag().help("Display the secret parameter values"))
                        .arg(Arg::with_name("starts_with")
                            .long("starts-with")
                            .help("Return parameters starting with search")
                            .takes_value(true)),
                    SubCommand::with_name("environment")
                        .visible_aliases(&["environ", "env"])
                        .about("Shows the environments with parameter overrides")
                        .arg(key_arg().help("Name of parameter to show environment values"))
                        .arg(Arg::with_name("all")
                            .short("a")
                            .long("all")
                            .help("Show even unset environments."))
                        .arg(param_as_of_arg())
                        .arg(show_times_arg())
                        .arg(table_format_options().help("Format for parameter values"))
                        .arg(secrets_display_flag().help("Display secret values in environments")),
                    SubCommand::with_name(GET_SUBCMD)
                        .about("Gets value for parameter in the selected environment")
                        .arg(param_as_of_arg())
                        .arg(Arg::with_name("details")
                            .short("d")
                            .long("details")
                            .help("Show all parameter details"))
                        .arg(key_arg().help("Name of parameter to get")),
                    SubCommand::with_name(LIST_SUBCMD)
                        .visible_aliases(LIST_ALIASES)
                        .about("List CloudTruth parameters")
                        .arg(Arg::with_name("external")
                            .long("external")
                            .alias("dynamic")
                            .help("Display the external values and FQN/JMES path."))
                        .arg(Arg::with_name("rules")
                            .long("rules")
                            .help("Display the parameter rules."))
                        .arg(Arg::with_name("evaluated")
                            .long("evaluated")
                            .help("Display the evaluated values"))
                        .arg(Arg::with_name("parents")
                            .long("parents")
                            .help("Display the parameters defined in a parent project"))
                        .arg(Arg::with_name("children")
                            .long("children")
                            .help("Display the parameters defined in a child project"))
                        .arg(values_flag().help("Display parameter information/values"))
                        .arg(param_as_of_arg())
                        .arg(show_times_arg())
                        .arg(table_format_options().help("Format for parameter values data"))
                        .arg(secrets_display_flag().help("Display the secret parameter values"))
                        .arg(immediate_parameters_flag()),
                    SubCommand::with_name(SET_SUBCMD)
                        .visible_aliases(SET_ALIASES)
                        .about(concat!("Set a value in the selected project/environment for ",
                            "an existing parameter or creates a new one if needed"))
                        .arg(key_arg().help("Name of parameter to set"))
                        .arg(description_option().help("Parameter description"))
                        .arg(Arg::with_name("FQN")
                            .short("f")
                            .long("fqn")
                            .takes_value(true)
                            .help("Fully Qualified Name (FQN) reference for external parameter."))
                        .arg(Arg::with_name("input-file")
                            .short("i")
                            .long("input")
                            .takes_value(true)
                            .help("Read the static value from the local input file"))
                        .arg(jmes_path_arg())
                        .arg(Arg::with_name("prompt")
                            .short("p")
                            .long("prompt")
                            .help("Set the static value using unecho'd terminal"))
                        .arg(rename_option().help("New parameter name"))
                        .arg(Arg::with_name("secret")
                            .long("secret")
                            .takes_value(true)
                            .possible_values(TRUE_FALSE_VALUES)
                            .help("Flags whether this is a secret parameter"))
                        .arg(Arg::with_name("evaluate")
                            .long("evaluate")
                            .short("e")
                            .alias("eval")
                            .takes_value(true)
                            .possible_values(TRUE_FALSE_VALUES)
                            .help("Flags whether this value gets evaluated")
                        )
                        .arg(Arg::with_name("param-type")
                            .short("t")
                            .long("type")
                            .takes_value(true)
                            .help("The parameter type. Fundamental types are: boolean, string, integer"))
                        .arg(rule_max_arg())
                        .arg(rule_no_max_arg())
                        .arg(rule_min_arg())
                        .arg(rule_no_min_arg())
                        .arg(rule_max_len_arg())
                        .arg(rule_no_max_len_arg())
                        .arg(rule_min_len_arg())
                        .arg(rule_no_min_len_arg())
                        .arg(rule_regex_arg())
                        .arg(rule_no_regex_arg())
                        .arg(Arg::with_name("create-child")
                            .long("create-child")
                            .help("Create a parameter in the child project"))
                        .arg(Arg::with_name("generate")
                            .long("generate")
                            .help("Generate a new value"))
                        .arg(Arg::with_name("value")
                            .short("v")
                            .long("value")
                            .takes_value(true)
                            .allow_hyphen_values(true)
                            .help("Static parameter value")),
                    SubCommand::with_name("unset")
                        .about(concat!("Remove a value/override from the selected ",
                            "project/environment and leaves the parameter in place."))
                        .arg(key_arg().help("Name of parameter to unset")),
                    SubCommand::with_name(DIFF_SUBCMD)
                        .visible_aliases(DIFF_ALIASES)
                        .about("Show differences between properties from two environments")
                        .arg(multi_env_name_arg())
                        .arg(Arg::with_name("properties")
                            .short("p")
                            .long("property")
                            .possible_values(&[
                                "value",
                                "type",
                                "environment",
                                "fqn",
                                "jmes-path",
                                "raw",
                                "rule-count",
                                "secret",
                                "created-at",
                                "modified-at",
                            ])
                            .multiple(true)
                            .default_value("value")
                            .help("List of the properties to compare."))
                        .arg(param_as_of_arg()
                            .multiple(true)
                            .help("Up to two times to be compared"))
                        .arg(table_format_options().help("Display difference format"))
                        .arg(secrets_display_flag().help("Show secret values"))
                        .arg(immediate_parameters_flag()),
                    SubCommand::with_name(PUSH_SUBCMD)
                        .visible_aliases(PUSH_ALIASES)
                        .about("Show push task steps for parameters")
                        .arg(key_arg().required(false).help("Parameter name"))
                        .arg(show_times_arg())
                        .arg(values_flag().help("Display push task step info"))
                        .arg(table_format_options().help("Format for push task step info")),
                ]),
        )
        .subcommand(SubCommand::with_name("templates")
            .visible_aliases(&["template", "temp", "te", "t"])
            .about("Work with CloudTruth templates")
            .subcommands(vec![
                SubCommand::with_name(DELETE_SUBCMD)
                    .visible_aliases(DELETE_ALIASES)
                    .about("Delete the CloudTruth template")
                    .arg(confirm_flag())
                    .arg(name_arg().help("Template name")),
                SubCommand::with_name(DIFF_SUBCMD)
                    .visible_aliases(DIFF_ALIASES)
                    .arg(name_arg().help("Template name"))
                    .arg(Arg::with_name("lines")
                        .long("context")
                        .short("c")
                        .takes_value(true)
                        .default_value("3")
                        .help("Number of lines of difference context"))
                    .arg(secrets_display_flag().help("Compare evaluated secret values"))
                    .arg(raw_arg().help("Compare unevaluated template bodies"))
                    .arg(multi_env_name_arg())
                    .arg(as_of_arg().multiple(true).help("Up to two times to be compared"))
                    .about("Show differences between templates"),
                SubCommand::with_name(EDIT_SUBCMD)
                    .visible_aliases(EDIT_ALIASES)
                    .about("Edit the specified template")
                    .arg(name_arg().help("Template name")),
                SubCommand::with_name(GET_SUBCMD)
                    .about("Get an evaluated template from CloudTruth")
                    .arg(raw_arg().help("Display unevaluated template body"))
                    .arg(as_of_arg().help(" Date/time (or tag) of template (and parameters)"))
                    .arg(secrets_display_flag().help("Display secret values in evaluation"))
                    .arg(name_arg().help("Template name")),
                SubCommand::with_name(HISTORY_SUBCMD)
                    .visible_aliases(HISTORY_ALIASES)
                    .arg(name_arg().help("Template name (optional)").required(false))
                    .arg(as_of_arg().help("Date/time (or tag) of template history"))
                    .arg(table_format_options().help("Format for the template history"))
                    .about("Display template history"),
                SubCommand::with_name(LIST_SUBCMD)
                    .visible_aliases(LIST_ALIASES)
                    .arg(values_flag().help("Display template information/values"))
                    .arg(table_format_options().help("Format for template values data"))
                    .arg(show_times_arg())
                    .about("List CloudTruth templates"),
                SubCommand::with_name("preview")
                    .about("Evaluate the provided local template file without storing")
                    .visible_aliases(&["prev", "pre"])
                    .arg(template_body().required(true).index(1))
                    .arg(param_as_of_arg())
                    .arg(secrets_display_flag().help("Display secret values in evaluation")),
                SubCommand::with_name(SET_SUBCMD)
                    .visible_aliases(SET_ALIASES)
                    .arg(name_arg().help("Template name"))
                    .arg(template_body().takes_value(true).short("b").long("body"))
                    .arg(rename_option().help("New template name"))
                    .arg(description_option().help("Template description"))
                    .about("Set the CloudTruth template"),
                SubCommand::with_name("validate")
                    .visible_aliases(&["valid", "val", "v"])
                    .arg(name_arg().help("Template name"))
                    .about("Validate a CloudTruth template"),
            ])
        )
        .subcommand(
            SubCommand::with_name("run")
                .visible_aliases(&["run", "ru", "r"])
                .about("Run a shell with the parameters in place")
                .args(&[
                    Arg::with_name("inheritance")
                        .long("inherit")
                        .short("i")
                        .takes_value(true)
                        .case_insensitive(true)
                        // TODO: Rick Porter 3/21 - pull subprocess::Inheritance enum value strings?
                        .possible_value("none")
                        .possible_value("underlay")
                        .possible_value( "overlay")
                        .possible_value("exclusive")
                        .default_value("overlay")
                        .help("Handle the relationship between local and CloudTruth environments"),
                    Arg::with_name("set")
                        .long("set")
                        .short("s")
                        .takes_value(true)
                        .multiple(true)
                        .help("Set the variables in this run, even possibly overriding the CloudTruth environment"),
                    Arg::with_name("remove")
                        .long("remove")
                        .short("r")
                        .takes_value(true)
                        .multiple(true)
                        .help("Remove the variables from the CloudTruth environment for this run"),
                    Arg::with_name("command")
                        .long("command")
                        .short("c")
                        .takes_value(true)
                        .help("Run this command"),
                    Arg::with_name("arguments")
                        .takes_value(true)
                        .multiple(true)
                        .allow_hyphen_values(true)
                        .last(true)
                        .help("Treat the rest of the arguments as the command"),
                    Arg::with_name("permissive")
                        .long("permissive")
                        .short("p")
                        .help("Allow CloudTruth application variables through"),
                    Arg::with_name("strict")
                        .long("strict")
                        .help("Fail when any parameters are unset"),
                    param_as_of_arg(),
                ])
        )
        .subcommand(
            SubCommand::with_name("projects")
                .visible_aliases(&["project", "proj"])
                .about("Work with CloudTruth projects")
                .subcommands(vec![
                    SubCommand::with_name(DELETE_SUBCMD)
                        .visible_aliases(DELETE_ALIASES)
                        .about("Delete specified CloudTruth project")
                        .arg(name_arg().help("Project name"))
                        .arg(confirm_flag()),
                    SubCommand::with_name(LIST_SUBCMD)
                        .visible_aliases(LIST_ALIASES)
                        .about("List CloudTruth projects")
                        .arg(show_times_arg())
                        .arg(values_flag().help("Display project information/values"))
                        .arg(table_format_options().help("Format for project values data")),
                    SubCommand::with_name(SET_SUBCMD)
                        .visible_aliases(SET_ALIASES)
                        .about("Create/update a CloudTruth project")
                        .arg(parent_arg().help("Parent project name, use empty string to remove parent"))
                        .arg(name_arg().help("Project name"))
                        .arg(rename_option().help("New project name"))
                        .arg(description_option().help("Project's description")),
                    SubCommand::with_name(TREE_SUBCMD)
                        .visible_aliases(TREE_ALIASES)
                        .about("Display CloudTruth project inheritance"),
                ])
        )
        .subcommand(SubCommand::with_name("actions")
            .visible_aliases(&["action", "act", "ac"])
            .about("Manage CloudTruth actions")
            .subcommands(vec![
                SubCommand::with_name(PUSH_SUBCMD)
                    .visible_aliases(PUSH_ALIASES)
                    .about("Manage CloudTruth pushes")
                    .subcommands(vec![
                        SubCommand::with_name(DELETE_SUBCMD)
                            .visible_aliases(DELETE_ALIASES)
                            .about("Delete a CloudTruth push")
                            .arg(confirm_flag())
                            .arg(integration_name_opt())
                            .arg(push_name_arg()),
                        SubCommand::with_name(GET_SUBCMD)
                            .about("Gets all the information for the specified CloudTruth push")
                            .arg(integration_name_opt())
                            .arg(push_name_arg()),
                        SubCommand::with_name(LIST_SUBCMD)
                            .visible_aliases(LIST_ALIASES)
                            .about("List CloudTruth pushes")
                            .arg(integration_name_opt())
                            .arg(values_flag().help("Show push info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Push info output format"))
                            .arg(tag_name_opt())
                            .arg(env_name_opt())
                            .arg(project_name_opt()),
                        SubCommand::with_name(SET_SUBCMD)
                            .visible_aliases(SET_ALIASES)
                            .about("Create/modify CloudTruth integration push")
                            .arg(integration_name_opt()
                                .help("Integration name (required for create)"))
                            .arg(push_name_arg())
                            .arg(rename_option().help("New push name"))
                            .arg(description_option().help("Description for the push"))
                            .arg(Arg::with_name("resource")
                                .long("resource")
                                .takes_value(true)
                                .help(concat!(
                                    "Resource string (required for create, [default: ",
                                    "'/{{ environment} }/{{ project }}/{{ parameter }}'])"
                                )))
                            .arg(Arg::with_name("project-add")
                                .value_name("project")
                                .takes_value(true)
                                .multiple(true)
                                .long("project")
                                .help("Project name(s) to be added"))
                            .arg(Arg::with_name("project-sub")
                                .value_name("project")
                                .takes_value(true)
                                .multiple(true)
                                .long("no-project")
                                .help("Project name(s) to be removed"))
                            .arg(Arg::with_name("tag-add")
                                .value_name("environment:tag")
                                .validator(environment_tag_validator)
                                .takes_value(true)
                                .multiple(true)
                                .long("tag")
                                .help("Tag name(s) to be added"))
                            .arg(Arg::with_name("tag-sub")
                                .value_name("environment:tag")
                                .validator(environment_tag_validator)
                                .takes_value(true)
                                .multiple(true)
                                .long("no-tag")
                                .help("Tag name(s) to be subtracted"))
                            .arg(push_dry_run_arg())
                            .arg(push_no_dry_run_arg())
                            .arg(push_check_owner_arg())
                            .arg(push_no_check_owner_arg())
                            .arg(push_local_arg())
                            .arg(push_no_local_arg())
                            .arg(push_coerce_params_arg())
                            .arg(push_no_coerce_params_arg())
                            .arg(push_include_params_arg())
                            .arg(push_no_include_params_arg())
                            .arg(push_include_secrets_arg())
                            .arg(push_no_include_secrets_arg())
                            .arg(push_include_templates_arg())
                            .arg(push_no_include_templates_arg())
                            .arg(Arg::with_name("region")
                                .long("region")
                                .takes_value(true)
                                .default_value("us-east-1")
                                .possible_values(REGION_VALUES)
                                .hide_possible_values(true) // list is too long, but want check
                                .help("Region where push tasks run (create only)"))
                            .arg(Arg::with_name("service")
                                .long("service")
                                .takes_value(true)
                                .default_value("ssm")
                                .possible_values(&["ssm", "secretsmanager"])
                                .help("Service for the push to use (create only)")),
                        SubCommand::with_name(SYNC_SUBCMD)
                            .visible_aliases(SYNC_ALIASES)
                            .about("Manually initiate action on existing push")
                            .arg(integration_name_opt())
                            .arg(push_name_arg())
                            .arg(push_dry_run_arg())
                            .arg(push_no_dry_run_arg())
                            .arg(push_check_owner_arg())
                            .arg(push_no_check_owner_arg())
                            .arg(push_coerce_params_arg())
                            .arg(push_no_coerce_params_arg())
                            .arg(push_include_params_arg())
                            .arg(push_no_include_params_arg())
                            .arg(push_include_secrets_arg())
                            .arg(push_no_include_secrets_arg())
                            .arg(push_include_templates_arg())
                            .arg(push_no_include_templates_arg()),
                        SubCommand::with_name(TASK_STEPS_SUBCMD)
                            .visible_aliases(TASK_STEPS_ALIASES)
                            .about("List task steps for the specified CloudTruth push")
                            .arg(integration_name_opt())
                            .arg(push_name_arg())
                            .arg(values_flag().help("Show push task step info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Push task steps info format")),
                        SubCommand::with_name(TASKS_SUBCMD)
                            .visible_aliases(TASKS_ALIASES)
                            .about("List tasks for the specified CloudTruth push")
                            .arg(integration_name_opt())
                            .arg(push_name_arg())
                            .arg(values_flag().help("Show push task info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Push task info format")),
                    ]),
                SubCommand::with_name(IMPORT_SUBCMD)
                    .visible_aliases(IMPORT_ALIASES)
                    .about("Manage CloudTruth imports")
                    .subcommands(vec![
                        SubCommand::with_name(DELETE_SUBCMD)
                            .visible_aliases(DELETE_ALIASES)
                            .about("Delete a CloudTruth import")
                            .arg(confirm_flag())
                            .arg(integration_name_opt())
                            .arg(pull_name_arg()),
                        SubCommand::with_name(GET_SUBCMD)
                            .about("Gets all the information for the specified CloudTruth import")
                            .arg(pull_name_arg())
                            .arg(integration_name_opt()),
                        SubCommand::with_name(LIST_SUBCMD)
                            .visible_aliases(LIST_ALIASES)
                            .about("List CloudTruth imports")
                            .arg(integration_name_opt())
                            .arg(values_flag().help("Show import info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Format for import info")),
                        SubCommand::with_name(SET_SUBCMD)
                            .visible_aliases(SET_ALIASES)
                            .about("Create/modify CloudTruth integration import")
                            .arg(pull_name_arg())
                            .arg(integration_name_opt().help("Integration name (required on create)"))
                            .arg(rename_option().help("New import name"))
                            .arg(description_option().help("Description for the import"))
                            .arg(Arg::with_name("dry-run")
                                .long("dry-run").help("Check that the import will work without doing it."))
                            .arg(Arg::with_name("resource")
                                .long("resource")
                                .takes_value(true)
                                .help(concat!(
                                    "Resource string (required for create, [default: ",
                                    "'/{{ environment} }/{{ project }}/{{ parameter }}'])"
                                )))
                            .arg(Arg::with_name("region")
                                .long("region")
                                .takes_value(true)
                                .default_value("us-east-1")
                                .possible_values(REGION_VALUES)
                                .hide_possible_values(true) // list is too long, but want check
                                .help("Region where import tasks run (create only)"))
                            .arg(Arg::with_name("service")
                                .long("service")
                                .takes_value(true)
                                .default_value("ssm")
                                .possible_values(&["ssm", "secretsmanager"])
                                .help("Service for the import to use (create only)")),
                        SubCommand::with_name(SYNC_SUBCMD)
                            .visible_aliases(SYNC_ALIASES)
                            .about("Manually initiate action on existing import")
                            .arg(pull_name_arg())
                            .arg(integration_name_opt()),
                        SubCommand::with_name(TASK_STEPS_SUBCMD)
                            .visible_aliases(TASK_STEPS_ALIASES)
                            .about("List task steps for the specified CloudTruth import")
                            .arg(integration_name_opt())
                            .arg(pull_name_arg())
                            .arg(values_flag().help("Show import task step info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Import task step info format")),
                        SubCommand::with_name(TASKS_SUBCMD)
                            .visible_aliases(TASKS_ALIASES)
                            .about("List tasks for the specified CloudTruth import")
                            .arg(pull_name_arg())
                            .arg(integration_name_opt())
                            .arg(values_flag().help("Show import task info values"))
                            .arg(show_times_arg())
                            .arg(table_format_options().help("Format for import task info")),

                    ]),
            ]))
        .subcommand(SubCommand::with_name("users")
            .visible_aliases(&["user", "us", "u"])
            .about("Work with CloudTruth users")
            .subcommands(vec![
                SubCommand::with_name("current")
                    .visible_aliases(&["cur", "c"])
                    .about("Display information about current user"),
                SubCommand::with_name(DELETE_SUBCMD)
                    .visible_aliases(DELETE_ALIASES)
                    .about("Delete specified CloudTruth account")
                    .arg(name_arg().help("Account name"))
                    .arg(confirm_flag()),
                SubCommand::with_name(GET_SUBCMD)
                    .about("Get detailed CloudTruth user information")
                    .arg(name_arg().help("Account name")),
                SubCommand::with_name(LIST_SUBCMD)
                    .visible_aliases(LIST_ALIASES)
                    .about("List CloudTruth users")
                    .arg(show_times_arg())
                    .arg(values_flag().help("Display account information/values"))
                    .arg(table_format_options().help("Format for account values data")),
                SubCommand::with_name(SET_SUBCMD)
                    .visible_aliases(SET_ALIASES)
                    .about("Create/update a CloudTruth service account")
                    .arg(name_arg().help("Account name"))
                    .arg(description_option().help("Account's description"))
                    .arg(role_arg()
                        .help("Account role [default: viewer (on create)]")),
                SubCommand::with_name("invitations")
                    .visible_aliases(&["invitation", "invite", "inv", "in", "i"])
                    .about("Manage user invitations")
                    .subcommands(vec![
                        SubCommand::with_name(DELETE_SUBCMD)
                            .visible_aliases(DELETE_ALIASES)
                            .about("Delete a user invitation")
                            .arg(invitation_name_arg())
                            .arg(confirm_flag()),
                        SubCommand::with_name(LIST_SUBCMD)
                            .visible_aliases(LIST_ALIASES)
                            .about("List user invitations")
                            .arg(values_flag().help("Display invitation values"))
                            .arg(table_format_options().help("Format for invitation values")),
                        SubCommand::with_name("resend")
                            .visible_aliases(&["re", "r"])
                            .about("Resend the user invitation to the specified user")
                            .arg(invitation_name_arg()),
                        SubCommand::with_name(SET_SUBCMD)
                            .visible_aliases(SET_ALIASES)
                            .about("Create/update a CloudTruth user invitation")
                            .arg(invitation_name_arg())
                            .arg(role_arg().help("Role for invited user [default: viewer (on create)]")),
                    ])
            ])
        )
        .subcommand(SubCommand::with_name("groups")
            .visible_aliases(&["group", "grp", "gr", "g"])
            .about("Manage CloudTruth user groups ")
            .subcommands(vec![
                SubCommand::with_name(DELETE_SUBCMD)
                    .visible_aliases(DELETE_ALIASES)
                    .about("Delete specified CloudTruth user group")
                    .arg(name_arg().help("Group name"))
                    .arg(confirm_flag()),
                SubCommand::with_name(GET_SUBCMD)
                    .about("Get detailed CloudTruth user group information")
                    .arg(name_arg().help("Group name")),
                SubCommand::with_name(LIST_SUBCMD)
                    .visible_aliases(LIST_ALIASES)
                    .about("List CloudTruth user groups")
                    .arg(show_times_arg())
                    .arg(values_flag().help("Display group information/values"))
                    .arg(table_format_options().help("Format for group values data")),
                SubCommand::with_name(SET_SUBCMD)
                    .visible_aliases(SET_ALIASES)
                    .about("Create/update a CloudTruth user group")
                    .arg(name_arg().help("Group name"))
                    .arg(description_option().help("Group description"))
                    .arg(rename_option().help("Rename the group"))
                    .arg(add_user_option())
                    .arg(remove_user_option())
            ])
        )
        .subcommand(SubCommand::with_name("schema")
            .about("View CloudTruth OpenAPI schema")
            .subcommands([
                SubCommand::with_name("server")
                    .visible_aliases(&["serv", "s"])
                    .arg(schema_format_arg())
                    .arg(schema_version_arg())
                    .about("Show the schema in use by the server"),
                SubCommand::with_name("local")
                    .visible_aliases(&["loc", "l"])
                    .arg(schema_format_arg())
                    .arg(schema_version_arg())
                    .about("Show the schema used to compile the CLI"),
                SubCommand::with_name(DIFF_SUBCMD)
                    .visible_aliases(DIFF_ALIASES)
                    .arg(schema_format_arg())
                    .arg(schema_version_arg())
                    .about("Compare the server and local schemas"),
            ])
        )
        .subcommand(SubCommand::with_name("versions")
            .visible_aliases(&["version", "vers", "ver", "ve", "v"])
            .about("Manage CloudTruth CLI versions")
            .subcommands([
                SubCommand::with_name(GET_SUBCMD)
                    .arg(Arg::with_name("latest")
                        .short("l")
                        .long("latest")
                        .help("Get the latest CLI version from the server"))
                    .about("Get the running CLI version"),
                SubCommand::with_name("check")
                    .visible_aliases(&["ch", "c"])
                    .about("Check the CLI is running the latest version")
                    .arg(Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Do not print version, just return error on outdated version.")),
                SubCommand::with_name("install")
                    .visible_aliases(&["inst", "in", "i"])
                    .about("Update to the latest CLI version")
                    .arg(Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Re-install even if already running the latest version"))
                    .arg(Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Do not print version, just return error on outdated version."))
            ])
        )
        .subcommand(SubCommand::with_name("import")
            .visible_aliases(&["imp", "im"])
            .about("Perform imports into the CloudTruth environment")
            .subcommands([
                SubCommand::with_name("parameters")
                    .visible_aliases(&["parameter", "param", "par", "pa", "p"])
                    .about("Import parameter values into a specified project and environment")
                    .args(&[
                        Arg::with_name("project")
                            .required(true)
                            .takes_value(true)
                            .index(1)
                            .help("Project name into which parameters are imported"),
                        Arg::with_name("file")
                            .required(true)
                            .takes_value(true)
                            .index(2)
                            .help("File that contains the text to import"),
                        Arg::with_name("environment")
                            .short("e")
                            .long("environment")
                            .alias("env")
                            .required(false)
                            .takes_value(true)
                            .help("Environment name into which parameters are imported"),
                        Arg::with_name("preview")
                            .long("preview")
                            .help("Simulate the import without saving any values"),
                        Arg::with_name("no-inherit")
                            .short("n")
                            .long("no-inherit")
                            .help("Do NOT inherit duplicate parameter values"),
                        Arg::with_name("ignore-param")
                            .short("i")
                            .long("ignore")
                            .value_name("param-name")
                            .takes_value(true)
                            .multiple(true)
                            .help("Parameters from the file to ignore"),
                        Arg::with_name("secret-param")
                            .long("secret")
                            .value_name("param-name")
                            .takes_value(true)
                            .multiple(true)
                            .help("Parameters from the file to treat as secrets"),
                        secrets_display_flag().help("Display secret values"),
                        table_format_options().help("Format for imported parameter"),
                        show_times_arg().help("Show import values created times")
                    ])
            ])
        )
        .subcommand(SubCommand::with_name("parameter-types")
            .visible_aliases(&["parameter-type", "param-types", "param-type", "types", "type", "ty"])
            .about("Manage parameter types in the CloudTruth environment")
            .subcommands([
                SubCommand::with_name(DELETE_SUBCMD)
                    .visible_aliases(DELETE_ALIASES)
                    .about("Delete specified CloudTruth parameter type")
                    .args(&[
                        name_arg().help("Parameter type name"),
                        confirm_flag(),
                    ]),
                SubCommand::with_name(LIST_SUBCMD)
                    .visible_aliases(LIST_ALIASES)
                    .about("List CloudTruth parameter types")
                    .args(&[
                        values_flag().help("Display parameter type information/values"),
                        show_times_arg(),
                        table_format_options().help("Display parameter type value info format"),
                        Arg::with_name("rules")
                            .long("rules")
                            .help("Display the parameter type rules."),
                    ]),
                SubCommand::with_name(SET_SUBCMD)
                    .visible_aliases(SET_ALIASES)
                    .about("Set parameter type and rules")
                    .args(&[
                        name_arg().help("Parameter type name"),
                        rename_option().help("New parameter type name"),
                        parent_arg().help("Parameter type parent"),
                        description_option().help("Parameter type description"),
                        rule_max_arg(),
                        rule_no_max_arg(),
                        rule_min_arg(),
                        rule_no_min_arg(),
                        rule_max_len_arg(),
                        rule_no_max_len_arg(),
                        rule_min_len_arg(),
                        rule_no_min_len_arg(),
                        rule_regex_arg(),
                        rule_no_regex_arg(),
                    ]),
                SubCommand::with_name(TREE_SUBCMD)
                        .visible_aliases(TREE_ALIASES)
                        .about("Show a tree representation of the parameter types"),
            ])
        )
        .subcommand(SubCommand::with_name("generate")
            .visible_aliases(&["gen", "ge"])
            .about("Generate items using CloudTruth service")
            .subcommands([
                SubCommand::with_name("password")
                    .visible_aliases(&["passwd", "pass", "pw", "pa"])
                    .about("Generate a password and print to console")
                    .args(&[
                        Arg::with_name("length")
                            .long("length")
                            .validator(i32_validator)
                            .default_value("15")
                            .help("Number of characters"),
                        Arg::with_name("hardware")
                            .long("hardware")
                            .help("Require hardware-based entropy"),
                        Arg::with_name("uppercase")
                            .long("uppercase")
                            .help("Require uppercase character"),
                        Arg::with_name("lowercase")
                            .long("lowercase")
                            .help("Require lowercase character"),
                        Arg::with_name("number")
                            .long("number")
                            .help("Require numeric character"),
                        Arg::with_name("symbol")
                            .long("symbol")
                            .help("Require symbol character"),
                        Arg::with_name("space")
                            .long("space")
                            .help("Require space character"),
                        Arg::with_name("no-hardware")
                            .long("no-hardware")
                            .help("Do not require hardware-based entropy"),
                        Arg::with_name("no-uppercase")
                            .long("no-uppercase")
                            .help("Do not require uppercase characters"),
                        Arg::with_name("no-lowercase")
                            .long("no-lowercase")
                            .help("Do not require lowercase character"),
                        Arg::with_name("no-number")
                            .long("no-number")
                            .help("Do not require numeric character"),
                        Arg::with_name("no-symbol")
                            .long("no-symbol")
                            .help("Do not require symbol character"),
                        Arg::with_name("no-space")
                            .long("no-space")
                            .help("Do not require space character"),
                    ])
            ])
        )
        .subcommand(SubCommand::with_name("backup")
            .visible_aliases(&["back", "ba"])
            .about("Manage backups of CloudTruth data")
            .subcommands([
                SubCommand::with_name("snapshot")
                    .visible_aliases(&["snap", "sn"])
                    .about("Take a snapshot of project/environment/type data for external storage")
                    .args(&[
                        serialized_format_arg().help("Backup snapshot format"),
                        confirm_flag(),
                    ])
            ])
        )
}
