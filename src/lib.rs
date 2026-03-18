use zed_extension_api::{self as zed, Command, ContextServerId, Result};

/// Headroom extension for Zed.
///
/// Registers Headroom as an MCP context server, giving Zed's AI agent
/// three tools: headroom_compress, headroom_retrieve, headroom_stats.
///
/// Requires `headroom` CLI in PATH (`pip install "headroom-ai[mcp]"`).
struct HeadroomExtension;

impl zed::Extension for HeadroomExtension {
    fn new() -> Self {
        HeadroomExtension
    }

    fn context_server_command(
        &mut self,
        _id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<Command> {
        Ok(Command {
            command: "headroom".to_string(),
            args: vec!["mcp".to_string(), "serve".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(HeadroomExtension);
