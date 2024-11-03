pub mod util;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMessage {
    Exit,
    Screen {
        id: String
    }
}

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
    pub screen: Screen
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Screen {
    pub widget: HashMap<String, Vec<SerdableWidget>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SerdableWidget {
    Button {
        pos: (i32, i32),
        scale: (i32, i32)
    },
    Slider {
        pos: (i32, i32),
        scale: (i32, i32)
    },
    Image {
        path: String,
        pos: (i32, i32),
        scale: (i32, i32)
    }
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