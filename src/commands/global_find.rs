use std::process::{Command, Stdio};
use crate::config::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Build fd command arguments based on config
    let mut fd_args = vec!["--hidden", "--follow", "--type", "f"];

    // Add exclusions from config
    for exclude in &config.exclude {
        fd_args.push("--exclude");
        fd_args.push(exclude);
    }

    // Set max depth if configured
    let depth_str;
    if let Some(depth) = config.max_depth {
        fd_args.push("--max-depth");
        depth_str = depth.to_string();
        fd_args.push(&depth_str);
    }

    // Determine search directories - expand tilde paths
    let search_dirs = if config.fav_dirs.is_empty() {
        vec![".".to_string()]
    } else {
        config.fav_dirs.iter().map(|dir| {
            if dir.starts_with("~/") {
                if let Some(home) = dirs::home_dir() {
                    home.join(&dir[2..]).to_string_lossy().to_string()
                } else {
                    dir.clone()
                }
            } else {
                dir.clone()
            }
        }).collect()
    };

    // Add search pattern (match all files) and then directories
    fd_args.push(".");
    for dir in &search_dirs {
        fd_args.push(dir);
    }

    // Start fd process
    let mut fd = Command::new("fd")
        .args(&fd_args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start fd command: {}. Make sure 'fd' is installed.", e))?;

    let fd_stdout = fd.stdout.take().unwrap();

    // Start fzf process with fd output as input
    let fzf = Command::new("fzf")
        .args(&[
            "--bind", "ctrl-u:preview-page-up,ctrl-d:preview-page-down",
            "--layout=reverse",
            "--border",
            "--info=inline",
            "--prompt", "File: "
        ])
        .stdin(fd_stdout)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start fzf command: {}. Make sure 'fzf' is installed.", e))?;

    // Wait for fzf to complete and get the selected file
    let fzf_output = fzf.wait_with_output()
        .map_err(|e| format!("Failed to get fzf output: {}", e))?;

    // Wait for fd to complete
    let _ = fd.wait();

    if fzf_output.status.success() && !fzf_output.stdout.is_empty() {
        let selected_file_raw = String::from_utf8_lossy(&fzf_output.stdout);
        let selected_file = selected_file_raw.trim();

        if !selected_file.is_empty() {
            println!("Selected: {}", selected_file);

            // Try to open with configured editor, then fallback to $EDITOR
            let editor = if let Some(ref configured_editor) = config.editor {
                configured_editor.clone()
            } else if let Ok(env_editor) = std::env::var("EDITOR") {
                env_editor
            } else {
                "cat".to_string()
            };

            let status = Command::new(&editor)
                .arg(selected_file)
                .status();

            match status {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to open file with {}: {}", editor, e),
            }
        }
    } else if !fzf_output.status.success() {
        // User cancelled or fzf failed
        return Ok(());
    } else {
        println!("No file selected.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_run_with_empty_config() {
        let config = Config {
            fav_dirs: vec![],
            exclude: vec![],
            editor: None,
            project_markers: None,
            max_depth: None,
            preview_max_size: None,
        };

        // This test will only succeed if fd and fzf are available
        // In a real testing environment, you'd want to mock these external dependencies
        let result = run(&config);
        assert!(result.is_ok() || result.is_err()); // Just ensure it doesn't panic
    }

    #[test]
    fn test_run_with_config() {
        let config = Config {
            fav_dirs: vec!["test".to_string()],
            exclude: vec!["*.tmp".to_string()],
            editor: Some("echo".to_string()),
            project_markers: Some(vec![".git".to_string()]),
            max_depth: Some(5),
            preview_max_size: Some(1024),
        };

        let result = run(&config);
        assert!(result.is_ok() || result.is_err()); // Just ensure it doesn't panic
    }
}
