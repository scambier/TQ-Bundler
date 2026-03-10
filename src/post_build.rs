use std::process::Command;

use crate::{config::Config, log, log_err};

fn split_command_template(command: &str) -> Result<Vec<String>, String> {
    let mut parts: Vec<String> = vec![];
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = command.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_single_quote {
            if ch == '\'' {
                in_single_quote = false;
            } else {
                current.push(ch);
            }
            continue;
        }

        if in_double_quote {
            if ch == '"' {
                in_double_quote = false;
            } else if ch == '\\' {
                if let Some(next) = chars.peek() {
                    if *next == '"' || *next == '\\' {
                        current.push(*next);
                        chars.next();
                    } else {
                        current.push(ch);
                    }
                } else {
                    current.push(ch);
                }
            } else {
                current.push(ch);
            }
            continue;
        }

        if ch.is_whitespace() {
            if !current.is_empty() {
                parts.push(current);
                current = String::new();
            }
            continue;
        }

        if ch == '\'' {
            in_single_quote = true;
            continue;
        }

        if ch == '"' {
            in_double_quote = true;
            continue;
        }

        current.push(ch);
    }

    if in_single_quote || in_double_quote {
        return Err("Unterminated quote in --post-build".to_string());
    }

    if !current.is_empty() {
        parts.push(current);
    }

    Ok(parts)
}

pub(crate) fn run_post_build_step(config: &Config) -> bool {
    let post_build = match config.post_build.as_ref() {
        Some(cmd) => cmd,
        None => return true,
    };

    let input_path = config.base_folder.join(&config.output_file);
    let output_path = config.base_folder.join(config.runtime_output_file());
    let mut args = match split_command_template(post_build) {
        Ok(parts) => parts,
        Err(e) => {
            log_err(format!("Invalid --post-build command: {}", e));
            return false;
        }
    };

    if args.is_empty() {
        log_err("Invalid --post-build command: empty command".to_string());
        return false;
    }

    for arg in args.iter_mut() {
        *arg = arg
            .replace("{input}", &input_path.to_string_lossy())
            .replace("{output}", &output_path.to_string_lossy());
    }

    log(format!("Running post-build command:\n{}", args.join(" ")));

    let program = args.remove(0);
    let status = Command::new(&program).args(&args).status();

    match status {
        Ok(exit_status) if exit_status.success() => true,
        Ok(exit_status) => {
            log_err(format!(
                "Post-build command failed with status {:?}",
                exit_status.code()
            ));
            false
        }
        Err(e) => {
            log_err(format!("Could not run post-build command: {:?}", e));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split_command_template;

    #[test]
    fn split_command_template_handles_quotes() {
        let args =
            split_command_template("python scripts/minify.py \"{input}\" \"{output}\"").unwrap();
        assert_eq!(
            args,
            vec![
                "python".to_string(),
                "scripts/minify.py".to_string(),
                "{input}".to_string(),
                "{output}".to_string()
            ]
        );
    }

    #[test]
    fn split_command_template_preserves_windows_paths() {
        let args = split_command_template(
            "\"C:\\Program Files\\Python\\python.exe\" scripts\\minify.py {input} {output}",
        )
        .unwrap();
        assert_eq!(args[0], "C:\\Program Files\\Python\\python.exe");
        assert_eq!(args[1], "scripts\\minify.py");
        assert_eq!(args[2], "{input}");
        assert_eq!(args[3], "{output}");
    }

    #[test]
    fn split_command_template_rejects_unterminated_quotes() {
        let err = split_command_template("python \"unterminated").unwrap_err();
        assert!(err.contains("Unterminated quote"));
    }
}
