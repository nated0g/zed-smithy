use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result, Worktree};

const GITHUB_REPO: &str = "smithy-lang/smithy-language-server";

struct SmithyExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for SmithyExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["0".to_string()],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    /// Returns the workspace configuration options to pass to the language server.
    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }
}

impl SmithyExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // TODO: This **would** work, but the path that the binary gets installed at
        // has spaces in it, and that crashes the LSP :(.  Not sure if this is a bug in the lsp
        // or in Zed.

        // If we already have a cached path and it exists, use it
        // if let Some(path) = &self.cached_binary_path {
        //     if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
        //         return Ok(path.clone());
        //     }
        // }

        // // Update installation status
        // zed::set_language_server_installation_status(
        //     language_server_id,
        //     &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        // );

        // let release = zed::latest_github_release(
        //     GITHUB_REPO,
        //     zed::GithubReleaseOptions {
        //         require_assets: true,
        //         pre_release: false,
        //     },
        // )?;

        // let release = zed::github_release_by_tag_name(GITHUB_REPO, "0.4.0")?;

        // // Determine platform and architecture
        // let (platform, arch) = zed::current_platform();
        // let os = match platform {
        //     zed::Os::Mac => "darwin",
        //     zed::Os::Linux => "linux",
        //     zed::Os::Windows => "windows",
        // };
        // let arch_str = match arch {
        //     zed::Architecture::Aarch64 => "aarch64",
        //     zed::Architecture::X86 => "x86",
        //     zed::Architecture::X8664 => "x86_64",
        // };
        // let asset_name = format!("smithy-language-server-{os}-{arch_str}.zip",);

        // // Find the matching asset
        // let asset = release
        //     .assets
        //     .iter()
        //     .find(|asset| asset.name == asset_name)
        //     .ok_or_else(|| format!("No asset found matching {:?}", asset_name))?;

        // // Define the installation directory and binary path
        // let version_dir = format!("smithy-language-server-{}", release.version);
        // let arch_dir = format!("smithy-language-server-{os}-{arch_str}");
        // let binary_name = if platform == zed::Os::Windows {
        //     "smithy-language-server.bat"
        // } else {
        //     "smithy-language-server"
        // };
        // let binary_dir = format!("{version_dir}/{arch_dir}/bin");
        // let binary_path = format!("{binary_dir}/{binary_name}");

        // // Download and extract if necessary
        // if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &zed::LanguageServerInstallationStatus::Downloading,
        //     );

        //     // Download and extract the ZIP file
        //     zed::download_file(
        //         &asset.download_url,
        //         &version_dir,
        //         zed::DownloadedFileType::Zip,
        //     )
        //     .map_err(|e| format!("Failed to download language server: {}", e))?;

        //     // Make the binary and java executable
        //     // Make the smithy language server binary executable
        //     if let Err(e) = zed::make_file_executable(&binary_path) {
        //         return Err(format!(
        //             "Failed to set executable permission for {}: {}",
        //             binary_path, e
        //         ));
        //     }

        //     // // Make the java binary executable
        //     // let java_path = format!("{binary_dir}/java");
        //     // if let Err(e) = zed::make_file_executable(&java_path) {
        //     //     return Err(format!(
        //     //         "Failed to set executable permission for {}: {}",
        //     //         java_path, e
        //     //     ));
        //     // }

        //     // Clean up old installations
        //     let entries = fs::read_dir(".")
        //         .map_err(|e| format!("Failed to list working directory: {}", e))?;
        //     for entry in entries {
        //         let entry = entry.map_err(|e| format!("Failed to load directory entry: {}", e))?;
        //         let path = entry.path();
        //         let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        //         if file_name.starts_with("smithy-language-server-") && file_name != version_dir {
        //             fs::remove_dir_all(path).ok();
        //         }
        //     }
        // }

        // TODO: Use the downloaded path once the space bug is fixed:
        let binary_path = worktree
            .which("smithy-language-server")
            .ok_or_else(|| "Failed to find smithy-language-server binary".to_string())?;

        // Cache and return the binary path
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(SmithyExtension);
