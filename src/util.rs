use std::{borrow::Cow, path::PathBuf, str::FromStr};

use eframe::egui::{self, include_image, Align, Area, Button, ComboBox, Context, DragValue, Id, Image, ImageSource, InnerResponse, Label, LayerId, Order, Pos2, Rect, Response, RichText, Sense, Slider, TextWrapMode, Ui, Vec2, Widget, WidgetText};

use crate::WindowScale;

pub fn create_button(text: RichText, ui: &mut Ui, pos: (f32, f32), size: (f32, f32)) -> Response {
    let button = Button::new(WidgetText::RichText(text)).sense(Sense::click());
    ui.put(Rect::from_min_size(Pos2::new(pos.0, pos.1), Vec2::new(size.0, size.1)), button)
}

pub fn create_slider(ctx: &Context, id: String, pos: (f32, f32), size: (f32, f32), default_value: &mut i32) {
    Area::new(id.into()).default_pos(pos).default_size(size).show(ctx, |ui| {
        ui.spacing_mut().slider_width = size.0;
        
        let slider = Slider::new(default_value, 0..=100).show_value(false);
    // ui.put(Rect::from_pos(Pos2::new(pos.0, pos.1)), Label::new(simple_text("0", 20.0)).wrap_mode(TextWrapMode::Wrap));
    // ui.put(Rect::from_pos(Pos2::new(pos.0 + size.0, pos.1)), Label::new(simple_text("100", 20.0)));
    
        slider.ui(ui);
    });
}

pub fn create_drag_value(ui: &mut Ui, pos: (f32, f32), size: (f32, f32), mut default_value: f32) -> Response {
    let slider = DragValue::new(&mut default_value).range(0..=100);
    ui.put(Rect::from_min_size(Pos2::new(pos.0, pos.1), Vec2::new(size.0, size.1)), slider)
}

pub fn simple_text(text: &str, size: f32) -> RichText {
    RichText::new(text).size(size)
}

fn uri(path: &str) -> String {
    let mut p = "file://".to_string();
    p.push_str(format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), path).as_str());
    p
}

pub fn ui_uri(path: &str) -> String {
    uri(format!("images/ui/{}.png", path).as_str())
}

pub fn bg_uri(path: &str) -> String {
    uri(format!("images/bg/{}.png", path).as_str())
}

pub fn character_uri(character: &str, face: &str) -> String {
    uri(format!("images/characters/{}/{}.png", character, face).as_str())
}

pub fn image(path: &str) -> ImageSource {
    ImageSource::Uri(Cow::Borrowed(&path))
}

pub fn ui_image(uri: &str) -> ImageSource {
    image(uri)
}

pub fn area(ctx: &Context, uri: &str, pos: (f32, f32), size: (f32, f32), affect: f32, order: Order) {
    Area::new(uri.to_string().into()).movable(false).interactable(false)
    .order(order)
    .fixed_pos((
        pos.0 * affect,
        pos.1 * affect
    )).default_size((
        size.0 * 1000.0 * affect,
        size.1 * 1000.0 * affect
    )).show(ctx, |ui| {
        ui.image(ui_image(&ui_uri("dialog_box")));
    });
}

pub fn show_text(ui: &mut Ui, text: &str, size: f32, pos: (f32, f32), affect: f32, extend: bool) -> Response {
    let label = if extend {
        Label::new(simple_text(text, size * affect)).extend().halign(Align::Center).selectable(false)
    } else {
        Label::new(simple_text(text, size * affect)).selectable(false)
    };

    ui.put(Rect::from_pos(Pos2::new(pos.0 * affect, pos.1 * affect)), label)
}

pub fn type_text(ui: &mut Ui, text: &str, pos: (f32, f32), size: f32, affect: f32, time: usize) {
    // let r = 0..=text.len();

    // r.into_iter().for_each(|i| {
    //     if i == time {
    //         let type_out = &text[0..i];
    //         let label = Label::new(simple_text(&type_out, size * affect)).selectable(false).extend();
    //         ui.put(Rect::from_pos(Pos2::new(pos.0 * affect, pos.1 * affect)), label);
    //     } else if i >= time {
    //         let type_out = &text[0..text.len()];
    //         let label = Label::new(simple_text(&type_out, size * affect)).selectable(false).extend();
    //         ui.put(Rect::from_pos(Pos2::new(pos.0 * affect, pos.1 * affect)), label);
    //     }
    // });
    let type_out = &text[0..text.len()];
    let label = Label::new(simple_text(&type_out, size * affect)).selectable(false).extend();
    ui.put(Rect::from_pos(Pos2::new(pos.0 * affect, pos.1 * affect)), label);
}