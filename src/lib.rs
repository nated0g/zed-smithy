use zed_extension_api::{self as zed, Result};

struct SmithyExtension;

impl zed::Extension for SmithyExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("smithy-language-server")
            .ok_or_else(|| "Smithy LSP must be installed manually. The recommended way is to install coursier (https://get-coursier.io/), and then run `cs install smithy-language-server`.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(SmithyExtension);
