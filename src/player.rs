use pumpkin_plugin_api::{EventHandler, IntoBlockKey, Server};
use pumpkin_plugin_api::events::{EventData, PlayerDeathEvent, PlayerInteractEvent, PlayerRespawnEvent};
use pumpkin_plugin_api::Item::*;
use pumpkin_plugin_api::text::TextComponent;

pub struct PlayerInteract;
impl EventHandler<PlayerInteractEvent> for PlayerInteract {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerInteractEvent>,
    ) -> EventData<PlayerInteractEvent> {
        let interact_block = event.block.clone().into_block_key();
        // Doors
        if interact_block == MangroveDoor.to_string() {
            let world = event.player.get_world();
            event.player.teleport((-115.0, 130.0, 107.0), None, None, world);
            event.cancelled = true;
        }
        if interact_block == BambooDoor.to_string() {
            let world = event.player.get_world();
            event.player.teleport((-115.0, 130.0, 107.0), None, None, world);
            event.cancelled = true;
        }
        if interact_block == SpruceDoor.to_string() {
            let world = event.player.get_world();
            event.player.teleport((-115.0, 130.0, 107.0), None, None, world);
            event.cancelled = true;
        }
        if interact_block == DarkOakDoor.to_string() {
            let world = event.player.get_world();
            event.player.teleport((-115.0, 130.0, 107.0), None, None, world);
            event.cancelled = true;
        }
        // Campfires
        if interact_block == SoulCampfire.to_string() {
            let playerpos = event.player.get_position();
            event.player.set_respawn_location(playerpos);
            event.cancelled = true;
        }
        event
    }
}

pub struct PlayerDeath;
impl EventHandler<PlayerDeathEvent> for PlayerDeath {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerDeathEvent>
    ) -> EventData<PlayerDeathEvent> {
        let name = event.player.get_name();
        let _newscore = event.dropped_exp;
        event.keep_inventory = true;
        event.death_message = TextComponent::text(&format!("{name}'s body has perished. May their soul live on"));
        event.player.clear_ender_chest();
        event
    }
}
pub struct PlayerRespawn;
impl EventHandler<PlayerRespawnEvent> for PlayerRespawn {
    fn handle(&self,
              _server: Server,
              mut event: EventData<PlayerRespawnEvent>
    ) -> EventData<PlayerRespawnEvent> {
        let _world = event.player.get_world();
        // event.player.teleport((-120.0, 120.0, 109.0), Some(-90.0), Some(0.0), world);
        // event.position = (-120.5, 120.0, 109.5);
        event
    }
}