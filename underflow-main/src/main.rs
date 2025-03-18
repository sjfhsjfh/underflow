mod config;
mod input;
mod scenes;
mod tween;
mod ui;
pub mod utils;

use input::{TOUCHES, on_frame};
use macroquad::prelude::*;
use once_cell::sync::Lazy;
use scenes::{NextScene, Scene};
use tokio::sync::Mutex;
use ui::Ui;
use utils::screen_to_world;

static SCENE_STACK: Lazy<Mutex<Vec<Box<dyn Scene + Send + Sync>>>> =
    Lazy::new(|| Mutex::new(vec![]));
static CONFIG: Lazy<Mutex<config::Config>> = Lazy::new(|| Mutex::new(config::Config::default()));

#[macroquad::main("Underflow")]
async fn main() {
    SCENE_STACK
        .lock()
        .await
        .push(Box::new(scenes::StartScene::new()));
    loop {
        clear_background(WHITE);
        let mut scene_stack = SCENE_STACK.lock().await;
        let current_scene = scene_stack.last_mut().unwrap();
        let mut ui = Ui::new(None);
        on_frame();
        TOUCHES.with(|t| t.borrow().0.clone()).retain_mut(|touch| {
            match current_scene.touch(&screen_to_world(&ui, touch)) {
                Ok(val) => !val,
                Err(err) => {
                    warn!("failed to handle touch: {:?}", err);
                    false
                }
            }
        });
        current_scene.render(&mut ui).unwrap();
        match current_scene.next_scene() {
            NextScene::None => {}
            NextScene::Pop => {
                scene_stack.pop();
            }
            NextScene::Replace(new_scene) => {
                scene_stack.pop();
                scene_stack.push(new_scene);
            }
        }
        next_frame().await
    }
}
