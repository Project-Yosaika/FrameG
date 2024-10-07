use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::time;

use eframe::*;
use eframe::egui::*;
use frameg::StoryController;
use frameg::util::{area, bg_uri, character_uri, create_button, create_slider, image, show_text, simple_text, type_text, ui_image, ui_uri};
use frameg::{Config, FramegEntry, Story, StoryComponent, WindowScale};
use ron::de::from_reader;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameStates {
    MainMenu,
    Gallery,
    Settings,
    SelectingSaved,
    SelectingStory,
    History,
    InGame
}

fn get_resource(file_path: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), file_path)
}

fn main() -> Result<(), Error> {
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
    let stories_path = get_resource("stories/");

    let file = File::open(input_path).expect("Failed opening file");
    let config = File::open(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to read config");
    let mut stories = Vec::new();

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

    fs::read_dir(stories_path).unwrap().for_each(|story| {
        let s = story.unwrap().path();
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

    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;

    eframe::run_native(&entry.name, native_options, Box::new(|cc| Ok(Box::new(FramegInstance::new(cc, &entry, config_file, stories)))))
}

struct FramegInstance<'a> {
    entry: &'a FramegEntry,
    config: Config,
    stories: Vec<Story>,
    state: GameStates,
    zoom_effect: f32,
    read_process: usize,
    text_play_process: f32
}

impl<'a> FramegInstance<'a> {
    fn new(cc: &eframe::CreationContext<'_>, entry: &'a FramegEntry, config: Config, stories: Vec<Story>) -> Self {
        cc.egui_ctx.set_theme(Theme::Light);
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            entry,
            config,
            stories,
            state: GameStates::MainMenu,
            zoom_effect: 1.0,
            read_process: 0,
            text_play_process: 0.0
        }
    }
}

impl<'a> eframe::App for FramegInstance<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.config.window_scale {
            WindowScale::Small => {
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(400.0, 225.0)));
                self.zoom_effect = 0.25;
            },
            WindowScale::Big => {
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(800.0, 450.0)));
                self.zoom_effect = 0.5;
            },
            WindowScale::Large => {
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(1600.0, 900.0)));
                self.zoom_effect = 1.0;
            },
            WindowScale::FullScreen => {
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
                self.zoom_effect = 1.0;
            },
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                GameStates::MainMenu => {
                    let new_game = create_button(
                        simple_text("New Game", self.entry.ui.main_menu.new_text_size * self.zoom_effect),
                        ui, 
                        (
                            self.entry.ui.main_menu.new_button_position.0 * self.zoom_effect,
                            self.entry.ui.main_menu.new_button_position.1 * self.zoom_effect
                        ),
                        (
                            self.entry.ui.main_menu.new_button_scale.0 * self.zoom_effect,
                            self.entry.ui.main_menu.new_button_scale.1 * self.zoom_effect
                        )
                    );
                    let continue_game = create_button(
                        simple_text("Continue", self.entry.ui.main_menu.continue_text_size * self.zoom_effect),
                        ui, 
                        (
                            self.entry.ui.main_menu.continue_button_position.0 * self.zoom_effect,
                            self.entry.ui.main_menu.continue_button_position.1 * self.zoom_effect
                        ), 
                        (
                            self.entry.ui.main_menu.continue_button_scale.0 * self.zoom_effect,
                            self.entry.ui.main_menu.continue_button_scale.1 * self.zoom_effect
                        )
                    );
                    let gallery = create_button(
                        simple_text("Gallery", self.entry.ui.main_menu.gallery_text_size * self.zoom_effect),
                        ui, 
                        (
                            self.entry.ui.main_menu.gallery_button_position.0 * self.zoom_effect,
                            self.entry.ui.main_menu.gallery_button_position.1 * self.zoom_effect
                        ), 
                        (
                            self.entry.ui.main_menu.gallery_button_scale.0 * self.zoom_effect,
                            self.entry.ui.main_menu.gallery_button_scale.1 * self.zoom_effect
                        )
                    );
                    let settings = create_button(
                        simple_text("Settings", self.entry.ui.main_menu.setting_text_size * self.zoom_effect),
                        ui,
                        (
                            self.entry.ui.main_menu.setting_button_position. 0* self.zoom_effect,
                            self.entry.ui.main_menu.setting_button_position.1 * self.zoom_effect
                        ), 
                        (
                            self.entry.ui.main_menu.setting_button_scale.0 * self.zoom_effect,
                            self.entry.ui.main_menu.setting_button_scale.1 * self.zoom_effect
                        )
                    );
                    let quit = create_button(
                        simple_text("Quit", self.entry.ui.main_menu.quit_text_size * self.zoom_effect),
                        ui, 
                        (self.entry.ui.main_menu.quit_button_position.0 * self.zoom_effect,
                            self.entry.ui.main_menu.quit_button_position.1 * self.zoom_effect
                        ), 
                        (
                            self.entry.ui.main_menu.quit_button_scale.0 * self.zoom_effect,
                            self.entry.ui.main_menu.quit_button_scale.1 * self.zoom_effect
                        )
                    );

                    if new_game.clicked() {
                        if self.entry.has_multi_story {
                            self.state = GameStates::SelectingStory
                        } else {
                            self.state = GameStates::InGame
                        }
                    }
                    if continue_game.clicked() {
                        self.state = GameStates::SelectingSaved
                    }
                    if gallery.clicked() {
                        self.state = GameStates::Gallery
                    }
                    if settings.clicked() {
                        self.state = GameStates::Settings
                    }
                    if quit.clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                },
                GameStates::Gallery => todo!(),
                GameStates::Settings => {
                    ui.label("Screen size:");
                    ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.config.window_scale))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.config.window_scale, WindowScale::Small, "400 * 225");
                            ui.selectable_value(&mut self.config.window_scale, WindowScale::Big, "800 * 450");
                            ui.selectable_value(&mut self.config.window_scale, WindowScale::Large, "1600 * 900");
                            ui.selectable_value(&mut self.config.window_scale, WindowScale::FullScreen, "Full Screen");
                        }
                    );
                    create_slider(
                        ui, 
                        (self.entry.ui.settings_menu.character_volume_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.character_volume_position.0 * self.zoom_effect
                        ),
                        (self.entry.ui.settings_menu.character_volume_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.character_volume_scale.0 * self.zoom_effect
                        ), 
                        &mut self.config.character_volume
                    );
                    create_slider(
                        ui, 
                        (self.entry.ui.settings_menu.sound_effect_volume_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.sound_effect_volume_position.0 * self.zoom_effect
                        ),
                        (self.entry.ui.settings_menu.sound_effect_volume_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.sound_effect_volume_scale.0 * self.zoom_effect
                        ), 
                        &mut self.config.sound_effect_volume
                    );
                    create_slider(
                        ui, 
                        (self.entry.ui.settings_menu.text_playback_speed_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.text_playback_speed_position.0 * self.zoom_effect
                        ),
                        (self.entry.ui.settings_menu.text_playback_speed_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.text_playback_speed_scale.0 * self.zoom_effect
                        ), 
                        &mut self.config.text_playback_speed
                    );
                    create_slider(
                        ui, 
                        (self.entry.ui.settings_menu.character_volume_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.character_volume_position.0 * self.zoom_effect
                        ),
                        (self.entry.ui.settings_menu.character_volume_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.character_volume_scale.0 * self.zoom_effect
                        ), 
                        &mut self.config.character_volume
                    );
                    let reset = create_button(
                        simple_text("Reset", self.entry.ui.settings_menu.reset_button_text_scale * self.zoom_effect), 
                        ui, 
                        (
                            self.entry.ui.settings_menu.reset_button_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.reset_button_position.1 * self.zoom_effect
                        ),
                        (
                            self.entry.ui.settings_menu.reset_button_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.reset_button_scale.1 * self.zoom_effect
                        )
                    );
                    let back = create_button(
                        simple_text("Quit", self.entry.ui.settings_menu.back_button_text_scale * self.zoom_effect), 
                        ui,
                        (
                            self.entry.ui.settings_menu.back_button_position.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.back_button_position.1 * self.zoom_effect
                        ),
                        (
                            self.entry.ui.settings_menu.back_button_scale.0 * self.zoom_effect,
                            self.entry.ui.settings_menu.back_button_scale.1 * self.zoom_effect
                        )
                    );

                    let data = ron::to_string(&self.config).expect("Serialization config failed");    
                    let mut new_config = File::create(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to create config");
                    write!(new_config, "{}", data).expect("Write config failed");

                    if reset.clicked() {
                        self.config = Config {
                            character_volume: 70,
                            sound_effect_volume: 50,
                            music_volume: 100,
                            text_playback_speed: 60,
                            window_scale: WindowScale::FullScreen
                        };
                        
                        let data = ron::to_string(&self.config).expect("Serialization config failed");
                        
                        let mut new_config = File::create(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"))).expect("Failed to create config");
                        
                        write!(new_config, "{}", data).expect("Write config failed");
                    }
                    if back.clicked() {
                        self.state = GameStates::MainMenu
                    }
                },
                GameStates::SelectingSaved => todo!(),
                GameStates::SelectingStory => todo!(),
                GameStates::History => todo!(),
                GameStates::InGame => {
                    self.text_play_process += 0.1 * (self.config.text_playback_speed as f32 / 100.0);

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.read_process += 1;
                        self.text_play_process = 0.0;
                    }

                    area(
                        ctx,
                        &ui_uri("dialog_box"),
                        self.entry.ui.in_game_hud.dialog_box_position,
                        self.entry.ui.in_game_hud.dialog_box_scale,
                        self.zoom_effect,
                        Order::Foreground
                    );

                    self.stories.iter().for_each(|single_story| {
                        let controllers: Vec<_> = single_story.content.keys().collect();

                        controllers.iter().for_each(|controller| {
                            if controller.1 == self.read_process {
                                let component = single_story.content.get(&controller).unwrap();
                                component.iter().for_each(|part| {
                                    if controller.0.is_some() {
                                        let c = controller.0.as_ref().unwrap();
                                        match c {
                                            StoryController::Branch(choice_list) => {
                                                let choices: Vec<_> = choice_list.iter().filter(|f| f.is_some()).map(|m| m.clone().unwrap()).collect();
                                                for i in 0..=choices.len() - 1 {
                                                    ui.label(choices.get(i).unwrap().text.clone());
                                                }
                                            },
                                            StoryController::Next(_) => todo!(),
                                            StoryController::If(story_lock, next_story) => todo!(),
                                            StoryController::End => {
                                                self.state = GameStates::MainMenu
                                            },
                                        }
                                    }

                                    match part {
                                        StoryComponent::SimpleText(text, said_by) => {
                                            Area::new("texts".into()).order(Order::Tooltip).show(ctx, |ui| {
                                                show_text(
                                                    ui,
                                                    &said_by.name,
                                                    self.entry.ui.in_game_hud.character_name_size,
                                                    self.entry.ui.in_game_hud.character_name_position,
                                                    self.zoom_effect,
                                                    true
                                                );
                                                type_text(
                                                    ui,
                                                    &text,
                                                    self.entry.ui.in_game_hud.dialog_text_position,
                                                    self.entry.ui.in_game_hud.dialog_text_scale,
                                                    self.zoom_effect,
                                                    self.text_play_process as usize
                                                );
                                            });
                                        },
                                        StoryComponent::SpecialText(_, _) => todo!(),
                                        StoryComponent::Bg(pointer, _) => {
                                            Area::new("background".into()).fade_in(false).order(Order::Background).default_pos((0.0, 0.0)).default_size((500.0 * self.zoom_effect, 500.0 * self.zoom_effect)).show(ctx, |ui| {
                                                ui.image(image(&bg_uri(&pointer)));
                                            });
                                        },
                                        StoryComponent::Cg(_, _) => todo!(),
                                        StoryComponent::ScreenFX(_, _) => todo!(),
                                        StoryComponent::Character(character, _) => {
                                            Area::new("characters".into()).order(Order::Middle).current_pos(character.pos).default_size(character.scale).show(ctx, |ui| {
                                                ui.image(image(&character_uri(&character.name, &character.face)));
                                            });
                                        },
                                    }
                                });
                            }
                        });
                    });
                }
            }
        });
    }
}
