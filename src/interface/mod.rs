//! Interface 模組：使用者介面層
//!
//! 目前包含：
//!   - `repl`：互動式命令列（REPL）
//!   - `server`：Server mode（JSON over stdin/stdout）
//!   - `websocket`：WebSocket server mode

pub mod repl;
pub use repl::Repl;

pub mod server;
pub use server::Server;

pub mod websocket;
pub use websocket::WsServer;
