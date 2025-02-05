use crate::cli::{binary_name, CONFIRM_FLAG};
use crate::config::Config;
use crate::lib::{get_api_access_url, user_confirm, warning_message, API_KEY_PAGE, SEPARATOR};
use clap::ArgMatches;
use color_eyre::eyre::Result;
use indoc::printdoc;
use std::io::{stdin, stdout, Write};

fn warn_login_done(reason: &str) {
    warning_message(format!("Login not performed: {}", reason));
}

pub fn process_login_command(subcmd_args: &ArgMatches, config: &Config) -> Result<()> {
    let confirmed = subcmd_args.is_present(CONFIRM_FLAG);
    let profile_name = &config.profile_name;
    let api_url = &config.server_url;
    let api_key = &config.api_key;
    let bin_name = binary_name();

    // if there's already an API key in place, give them the option to continue
    if !api_key.is_empty() {
        if !confirmed {
            printdoc!(
                r#"

                  {}
                    An API key is already setup for profile '{}'.
                    Login will overwrite the current configuration profile API key.
                    Using a new API key will not remove access via the old API key.
                    Use '{} logout' to remove an existing API key from a profile.

                "#,
                SEPARATOR,
                profile_name,
                bin_name,
            );
            let msg = format!(
                "Do you want to update the API key in profile '{}'",
                profile_name
            );
            if !user_confirm(msg, Some(false)) {
                warn_login_done("using existing API key");
                return Ok(());
            }
        } else {
            let msg = format!("Updating API key in profile '{}'.", profile_name);
            warning_message(msg);
        }
    }

    if let Ok(api_key_url) = get_api_access_url(api_url) {
        let mut open_page = true;
        if !confirmed {
            printdoc!(
                r#"

              {}
                Use a browser to generate a new API token from the {} page
                ({}).

              "#,
                SEPARATOR,
                API_KEY_PAGE,
                api_key_url,
            );
            if !user_confirm(format!("Open the {} page", API_KEY_PAGE), Some(true)) {
                open_page = false;
            }
        } else {
            warning_message(format!(
                "Opening {} page ({}) in browser.",
                API_KEY_PAGE, api_key_url
            ));
        }

        if open_page {
            let open_result = webbrowser::open(&api_key_url);
            if open_result.is_err() {
                printdoc!(
                    r#"
                    "Failed to open browser:
                    {}

                    You must manually open '{}' to generate a new key."#,
                    open_result.unwrap_err().to_string(),
                    api_key_url,
                );
            }
        }

        printdoc!(
            r#"

              {}
                Use the "Generate New Token" button on the {} page, copy the value, and paste
                that value here.

            "#,
            SEPARATOR,
            API_KEY_PAGE,
        );
    } else {
        warning_message(format!("Unable to determine {} page URL", API_KEY_PAGE));
    }

    // TODO: sanity checks?
    // TODO: no echo (using rpassword)?
    println!("Enter the new \"API key\" here:");
    let mut input = String::new();
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut input);
    input = input.trim().to_string();
    if input.is_empty() {
        warn_login_done("no API key provided");
        return Ok(());
    }

    Config::update_profile(profile_name, Some(input.as_str()), None, None, None, None)?;
    println!(
        "Updated profile '{}' in {}",
        profile_name,
        Config::filename()
    );
    Ok(())
}
