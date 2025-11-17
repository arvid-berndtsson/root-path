use anyhow::{Context, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let platform = env::consts::OS;
    let arch = env::consts::ARCH;

    // Map OS names to npm package naming conventions
    let (os_name, binary_name) = match (platform, arch) {
        ("windows", _) => ("win32", "cc-check-win32-x64.exe"),
        ("macos", "aarch64" | "arm64") => ("darwin", "cc-check-darwin-arm64"),
        ("macos", _) => ("darwin", "cc-check-darwin-x64"),
        (_, "aarch64" | "arm64") => ("linux", "cc-check-linux-aarch64"),
        _ => ("linux", "cc-check-linux-x64"),
    };

    // Get the directory where this binary is located (should be in bin/)
    let exe_path = env::current_exe().context("failed to determine current executable path")?;
    let bin_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("could not determine binary directory"))?;

    let source_path = bin_dir.join(binary_name);
    let target_name = if platform == "windows" {
        "cc-check.exe"
    } else {
        "cc-check"
    };
    let target_path = bin_dir.join(target_name);

    if !source_path.exists() {
        eprintln!("⚠ Binary not found: {}", source_path.display());
        return Ok(()); // Don't fail, just warn
    }

    fs::copy(&source_path, &target_path).with_context(|| {
        format!(
            "failed to copy binary from {} to {}",
            source_path.display(),
            target_path.display()
        )
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_path)
            .with_context(|| format!("failed to read metadata for {}", target_path.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_path, perms)
            .with_context(|| format!("failed to set permissions for {}", target_path.display()))?;
    }

    println!("✓ Installed cc-check for {} {}", os_name, arch);
    Ok(())
}
