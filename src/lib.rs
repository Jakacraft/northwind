mod player;

use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Server, permissions,
    events::{EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent},
    text::TextComponent,
};
use tracing::*;
use crate::player::PlayerDeath;
use crate::player::PlayerInteract;
use crate::player::PlayerRespawn;

struct Northwind;
impl Plugin for Northwind {
    fn new() -> Self {
        Northwind
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Northwind".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Jakacraft".into()],
            description: "Northwind Plugin. Yay!".into(),
            dependencies: vec![],
            permissions: vec![permissions::FS_READ_DATA.into()],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("[Northwind] COFFEE COFFEE COFFEE!");
        context.register_event_handler(JoinHandler, EventPriority::Normal, true)?;
        context.register_event_handler(LeaveHandler, EventPriority::Normal, true)?;
        context.register_event_handler(PlayerInteract, EventPriority::Normal, true)?;
        context.register_event_handler(PlayerDeath, EventPriority::Normal, true)?;
        context.register_event_handler(PlayerRespawn, EventPriority::Normal, true)?;
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("[Northwind] Gone to get more coffee");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(Northwind);

struct JoinHandler;
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let name = event.player.get_name();
        event.join_message = TextComponent::text(&format!("{name}, Welcome to Northwind"));
        event
    }
}

struct LeaveHandler;
impl EventHandler<PlayerLeaveEvent> for LeaveHandler {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerLeaveEvent>,
    ) -> EventData<PlayerLeaveEvent> {
        let name = event.player.get_name();
        event.leave_message = TextComponent::text(&format!("{name} left. I'm sure they'll be back..."));
        event
    }
}