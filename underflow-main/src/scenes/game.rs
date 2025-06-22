use comui::{component::Component, layout::Layout, utils::Transform};
use underflow_core::server::FlowServer;

use crate::scenes::preflight::Player;

pub struct GameScene {
    pub players: Vec<Player>,
    pub game_server: FlowServer,
}

impl Layout for GameScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        vec![]
    }
}
