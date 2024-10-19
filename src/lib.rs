pub mod util;

use eframe::egui::{ahash::HashMap, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub character_volume: i32,
    pub sound_effect_volume: i32,
    pub music_volume: i32,
    pub text_playback_speed: i32,
    pub window_scale: WindowScale
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FramegEntry {
    pub name: String,
    pub has_multi_story: bool,
    pub ui: Ui
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ui {
    pub main_menu: MainMenu,
    pub settings_menu: SettingsMenu,
    pub in_game_hud: InGameHud
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainMenu {
    pub new_button_position: (f32, f32),
    pub continue_button_position: (f32, f32),
    pub gallery_button_position: (f32, f32),
    pub setting_button_position: (f32, f32),
    pub quit_button_position: (f32, f32),
    pub new_button_scale: (f32, f32),
    pub continue_button_scale: (f32, f32),
    pub gallery_button_scale: (f32, f32),
    pub setting_button_scale: (f32, f32),
    pub quit_button_scale: (f32, f32),
    pub new_text_size: f32,
    pub continue_text_size: f32,
    pub gallery_text_size: f32,
    pub setting_text_size: f32,
    pub quit_text_size: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsMenu {
    pub character_volume_position: (f32, f32),
    pub sound_effect_volume_position: (f32, f32),
    pub music_volume_position: (f32, f32),
    pub text_playback_speed_position: (f32, f32),
    pub character_volume_scale: (f32, f32),
    pub sound_effect_volume_scale: (f32, f32),
    pub music_volume_scale: (f32, f32),
    pub text_playback_speed_scale: (f32, f32),
    pub window_scale: WindowScale,
    pub window_scale_button_position: (f32, f32),
    pub window_scale_button_scale: (f32, f32),
    pub reset_button_position: (f32, f32),
    pub reset_button_scale: (f32, f32),
    pub reset_button_text_scale: f32,
    pub back_button_position: (f32, f32),
    pub back_button_scale: (f32, f32),
    pub back_button_text_scale: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InGameHud {
    pub dialog_box_position: (f32, f32),
    pub dialog_box_scale: (f32, f32),
    pub dialog_text_position: (f32, f32),
    pub dialog_text_scale: f32,
    pub character_name_position: (f32, f32),
    pub character_name_size: f32,
    pub save_button_position: (f32, f32),
    pub history_button_position: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub id: String,
    pub content: HashMap<(Option<StoryController>, usize), Vec<StoryComponent>>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum StoryComponent {
    SimpleText(String, CharacterName),
    Bg(String, usize),
    Cg(String, usize),
    ScreenFX(String, usize),
    Character(Character, usize)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TextComponents {
    RichText(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub face: String,
    pub pos: (f32, f32),
    pub scale: (f32, f32)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CharacterName {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum StoryController {
    Branch([Option<Choice>; 5]),
    Next(String),
    If(StoryLock, String),
    End
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum StoryLock {
    MultiTimesPlay(i32),
    UnlockedDifferentEnd(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Choice {
    pub text: String,
    pub next_story: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowScale {
    Small,
    Big,
    Large,
    FullScreen
}