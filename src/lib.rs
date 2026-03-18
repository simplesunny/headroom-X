use zed_extension_api::{self as zed, ContextServerId, Command, Result};

/// Headroom extension for Zed.
///
/// Provides context compression for Zed's AI agent via MCP:
/// - headroom_compress: Compress content on demand
/// - headroom_retrieve: Retrieve original uncompressed content
/// - headroom_stats: Session compression statistics
///
/// For automatic compression of ALL traffic, configure Zed to use
/// the Headroom proxy as a custom OpenAI-compatible endpoint:
///
/// 1. Start proxy: `headroom proxy`
/// 2. In Zed settings, set:
///    "language_models": {
///      "openai": { "api_url": "http://localhost:8787" }
///    }
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
        // Try headroom in PATH first, fall back to python -m
        Ok(Command {
            command: "headroom".to_string(),
            args: vec!["mcp".to_string(), "serve".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(HeadroomExtension);
