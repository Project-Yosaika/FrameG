use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

use frameg::{Config, FramegEntry, GameMessage, Story, StoryComponent, WindowScale};
use iced::application::{Update, View};
use iced::widget::{center, column};
use iced::{window, Element, Renderer, Task, Theme};
use ron::de::from_reader;

fn get_resource(file_path: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), file_path)
}

fn main() -> iced::Result {
    iced::application("Test", FramegInstance::update, FramegInstance::view).run()
}

struct FramegInstance {
    entry: FramegEntry,
    config: Config,
    stories: Vec<Story>,
    rendering_screen_name: String
}

impl Default for FramegInstance {
    fn default() -> Self {
        if File::open(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).is_err() {
            let new_config = Config {
                character_volume: 70,
                sound_effect_volume: 50,
                music_volume: 100,
                text_playback_speed: 60,
                window_scale: WindowScale::FullScreen,
            };
            let data = ron::to_string(&new_config).expect("Serialization config failed");
    
            let mut new_config = File::create(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to create config");
    
            write!(new_config, "{}", data).expect("Write config failed");
        }
        
        let input_path = get_resource("entry.ron");
        
    
        let file = File::open(input_path).expect("Failed opening file");
        let config = File::open(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to read config");
        
    
        let config_file: Config = match from_reader(config) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load your config: {}", e);
    
                std::process::exit(1);
            }
        };
    
        let entry: FramegEntry = match from_reader(file) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load your entry: {}", e);
    
                std::process::exit(1);
            }
        };
    
        FramegInstance::new(entry, config_file)
    }
}

impl FramegInstance {
    fn update(&mut self, message: GameMessage) -> Task<GameMessage>{
        match message {
            GameMessage::Exit => window::get_latest().and_then(window::close),
            GameMessage::Screen { id } => {
                self.rendering_screen_name = id;
                Task::none()
            },
        }
    }
    
    fn view(&self) -> Element<GameMessage> {
        let content = iced::widget::column![];

        center(content).into()   
    }

    fn new(entry: FramegEntry, config: Config) -> FramegInstance {
        let stories_path = get_resource("stories/");
        let mut stories = Vec::new();
    
        fs::read_dir(stories_path).unwrap_or_else(|o| {
            panic!("{}", o)
        }).for_each(|story| {
            story.iter().for_each(|f| {
                let s = f.path();
                let s = File::open(s);
                let s: Story = match from_reader(s.unwrap()) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Failed to load story: {}", e);
            
                        std::process::exit(1);
                    }
                };
                stories.push(s);
            });
        });

        FramegInstance {
            entry,
            config,
            stories,
            rendering_screen_name: String::default()
        }
    }
}

