use anyhow::{Context, Result};
use std::process::Command;
use std::path::PathBuf;

/// Supported terminal applications
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalApp {
    /// iTerm2 (macOS)
    ITerm2,
    /// Terminal.app (macOS)
    Terminal,
    /// Alacritty (cross-platform)
    Alacritty,
    /// Kitty (cross-platform)
    Kitty,
    /// Warp (macOS)
    Warp,
    /// Custom command
    Custom(String),
}

impl TerminalApp {
    /// Detect available terminals on the system
    pub fn detect_available() -> Vec<TerminalApp> {
        let mut available = Vec::new();

        #[cfg(target_os = "macos")]
        {
            if Self::is_app_installed("iTerm") {
                available.push(TerminalApp::ITerm2);
            }
            if Self::is_app_installed("Terminal") {
                available.push(TerminalApp::Terminal);
            }
            if Self::is_app_installed("Warp") {
                available.push(TerminalApp::Warp);
            }
        }

        // Cross-platform terminals
        if Self::is_command_available("alacritty") {
            available.push(TerminalApp::Alacritty);
        }
        if Self::is_command_available("kitty") {
            available.push(TerminalApp::Kitty);
        }

        available
    }

    /// Get default terminal for the platform
    pub fn default() -> TerminalApp {
        #[cfg(target_os = "macos")]
        {
            if Self::is_app_installed("iTerm") {
                return TerminalApp::ITerm2;
            }
            return TerminalApp::Terminal;
        }

        #[cfg(not(target_os = "macos"))]
        {
            if Self::is_command_available("alacritty") {
                return TerminalApp::Alacritty;
            }
            if Self::is_command_available("kitty") {
                return TerminalApp::Kitty;
            }
            // Fallback to custom with default terminal emulator
            TerminalApp::Custom("x-terminal-emulator".to_string())
        }
    }

    /// Check if a macOS app is installed
    #[cfg(target_os = "macos")]
    fn is_app_installed(app_name: &str) -> bool {
        Command::new("mdfind")
            .args(&[
                "kMDItemKind == 'Application'",
                "&&",
                &format!("kMDItemFSName == '{}.app'", app_name),
            ])
            .output()
            .map(|output| !output.stdout.is_empty())
            .unwrap_or(false)
    }

    /// Check if a command is available in PATH
    fn is_command_available(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get display name for the terminal
    pub fn display_name(&self) -> &str {
        match self {
            TerminalApp::ITerm2 => "iTerm2",
            TerminalApp::Terminal => "Terminal.app",
            TerminalApp::Alacritty => "Alacritty",
            TerminalApp::Kitty => "Kitty",
            TerminalApp::Warp => "Warp",
            TerminalApp::Custom(_) => "Custom",
        }
    }
}

impl std::fmt::Display for TerminalApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}


/// Launch SSH connection in external terminal
pub fn launch_ssh_connection(
    terminal: &TerminalApp,
    hostname: &str,
    port: u16,
    username: &str,
    identity_path: Option<&PathBuf>,
) -> Result<()> {
    let mut ssh_args = vec![];

    // Add identity file if provided
    if let Some(key_path) = identity_path {
        ssh_args.push("-i".to_string());
        ssh_args.push(key_path.to_string_lossy().to_string());
    }

    // Add connection details
    ssh_args.push(format!("{}@{}", username, hostname));

    // Add port if not default
    if port != 22 {
        ssh_args.push("-p".to_string());
        ssh_args.push(port.to_string());
    }

    match terminal {
        #[cfg(target_os = "macos")]
        TerminalApp::ITerm2 => {
            let ssh_command = format!("ssh {}", ssh_args.join(" "));
            Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "tell application \"iTerm\"\n\
                         activate\n\
                         create window with default profile command \"{}\"\n\
                         end tell",
                        ssh_command
                    ),
                ])
                .spawn()
                .context("Failed to launch iTerm2")?;
        }

        #[cfg(target_os = "macos")]
        TerminalApp::Terminal => {
            let ssh_command = format!("ssh {}", ssh_args.join(" "));
            Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "tell application \"Terminal\"\n\
                         activate\n\
                         do script \"{}\"\n\
                         end tell",
                        ssh_command
                    ),
                ])
                .spawn()
                .context("Failed to launch Terminal.app")?;
        }

        #[cfg(target_os = "macos")]
        TerminalApp::Warp => {
            let ssh_command = format!("ssh {}", ssh_args.join(" "));
            Command::new("open")
                .args(&["-a", "Warp", "--args", &ssh_command])
                .spawn()
                .context("Failed to launch Warp")?;
        }

        TerminalApp::Alacritty => {
            let mut cmd = Command::new("alacritty");
            cmd.arg("-e").arg("ssh");
            for arg in &ssh_args {
                cmd.arg(arg);
            }
            cmd.spawn().context("Failed to launch Alacritty")?;
        }

        TerminalApp::Kitty => {
            let mut cmd = Command::new("kitty");
            cmd.arg("ssh");
            for arg in &ssh_args {
                cmd.arg(arg);
            }
            cmd.spawn().context("Failed to launch Kitty")?;
        }

        TerminalApp::Custom(command) => {
            let ssh_command = format!("ssh {}", ssh_args.join(" "));
            Command::new("sh")
                .args(&["-c", &format!("{} -e '{}'", command, ssh_command)])
                .spawn()
                .context("Failed to launch custom terminal")?;
        }

        #[cfg(not(target_os = "macos"))]
        _ => {
            anyhow::bail!("Terminal not supported on this platform");
        }
    }

    Ok(())
}

/// Write SSH key to temporary file with secure permissions
pub fn write_temp_key(key_data: &str) -> Result<PathBuf> {
    use std::fs;
    use std::io::Write;

    let temp_dir = std::env::temp_dir();
    let key_path = temp_dir.join(format!("nebulavault_key_{}", uuid::Uuid::new_v4()));

    let mut file = fs::File::create(&key_path)
        .context("Failed to create temporary key file")?;

    // Set secure permissions (owner read/write only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&key_path, perms)?;
    }

    file.write_all(key_data.as_bytes())
        .context("Failed to write key data")?;

    Ok(key_path)
}

/// Clean up temporary key file
pub fn cleanup_temp_key(key_path: &PathBuf) -> Result<()> {
    if key_path.exists() {
        std::fs::remove_file(key_path)
            .context("Failed to remove temporary key file")?;
    }
    Ok(())
}
