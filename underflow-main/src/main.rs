mod scenes;

use macroquad::prelude::*;
use once_cell::sync::Lazy;
use scenes::{NextScene, Scene};
use tokio::sync::Mutex;

static SCENE_STACK: Lazy<Mutex<Vec<Box<dyn Scene + Send + Sync>>>> =
    Lazy::new(|| Mutex::new(vec![]));

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
        current_scene.render(&mut macroquad::ui::root_ui()).unwrap();
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
