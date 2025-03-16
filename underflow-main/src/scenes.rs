use macroquad::ui::Ui;

pub(crate) mod game;
pub(crate) mod start;

pub(crate) use game::GameScene;
pub(crate) use start::StartScene;

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
}
