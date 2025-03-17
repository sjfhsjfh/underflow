pub(crate) mod game;
pub(crate) mod start;

pub(crate) use game::GameScene;
use macroquad::input::Touch;
pub(crate) use start::StartScene;

use crate::ui::Ui;

#[derive(Default)]
pub enum NextScene {
    #[default]
    None,
    Pop,
    Replace(Box<dyn Scene + Send + Sync>),
}

pub trait Scene {
    fn next_scene(&self) -> NextScene;
    fn render(&mut self, ui: &mut Ui) -> anyhow::Result<()>;
    /// `Some(true)` if the event was consumed
    fn touch(&mut self, _touch: &Touch) -> anyhow::Result<bool> {
        Ok(false)
    }
}
