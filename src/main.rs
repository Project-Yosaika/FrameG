use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

use frameg::{Config, FramegEntry, GameMessage, SerdableWidget, Story, StoryComponent, WindowScale};
use iced::application::{Update, View};
use iced::widget::image::Handle;
use iced::widget::{button, center, column, container, slider, Button, Image, Slider, Text};
use iced::{window, Element, Renderer, Task, Theme};
use ron::de::from_reader;

fn get_resource(file_path: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), file_path)
}

fn main() -> iced::Result {
    iced::application("Frameg Runner", FramegInstance::update, FramegInstance::view).run()
}
struct GameState {
    rendering_screen_name: String
}
struct FramegInstance {
    entry: FramegEntry,
    config: Config,
    state: GameState
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
    fn update(&mut self, message: GameMessage) -> Task<GameMessage> {
        let config = File::open(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to read config");

        self.config = match from_reader(config) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load your config: {}", e);
    
                std::process::exit(1);
            }
        };
        
        match message {
            GameMessage::Exit => window::get_latest().and_then(window::close),
            GameMessage::Screen { id } => {
                self.state.rendering_screen_name = id;
                Task::none()
            },
            GameMessage::ConfigValueChange { id, value } => {
                match id.as_str() {
                    "character_volume" => {
                        self.config.character_volume = value
                    },
                    _ => panic!("non-existing config object")
                }
                let data = ron::to_string(&self.config).expect("Serialization config failed");    
                let mut new_config = File::create(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to create config");
                write!(new_config, "{}", data).expect("Write config failed");
                
                Task::none()
            },
        }
    }
    
    fn view(&self) -> Element<GameMessage> {
        let mut content: iced::widget::Column<'_, GameMessage, Theme, Renderer> = column![];

        let screen = match self.entry.screen.widget.get(&self.state.rendering_screen_name) {
            Some(value) => value,
            None => panic!("Your screen is non-existing"),
        };

        for widget in screen {
            match widget {
                SerdableWidget::Button { pos, scale, action, text } => {
                    let button = column![
                        container(
                            button(text.as_str())
                                .width(scale.0)
                                .height(scale.1)
                                .on_press(action.clone())
                        )
                    ];

                    content = content.push(button);
                },
                SerdableWidget::Slider { pos, scale, value_id, id, max, min } => {
                    let value = match value_id.as_str() {
                        "character_volume" => {
                            self.config.character_volume
                        },
                        _ => {
                            panic!("non-existing config value");
                        }
                    };
                    let slider = column![
                        container(
                            slider(
                                min.clone()..=max.clone(), 
                                value,
                                |changed| GameMessage::ConfigValueChange { id: id.to_string(), value: changed }
                            )
                            .width(scale.0)
                            .height(scale.1)
                            .default(70)
                            .shift_step(10)
                            .step(1)
                        )
                    ];

                    content = content.push(slider);
                },
                _ => {
                    
                }
            }
        }

        center(content).into()
    }

    fn new(entry: FramegEntry, config: Config) -> FramegInstance {
        let state = GameState {
            rendering_screen_name: String::from("main_menu")
        };

        FramegInstance {
            entry,
            config,
            state
        }
    }
}

