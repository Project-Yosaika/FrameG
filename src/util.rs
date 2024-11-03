use std::{borrow::Cow, path::PathBuf, str::FromStr, time};

use iced::widget::{shader::wgpu::naga::ImageClass, Image};

fn uri(path: &str) -> String {
    let mut p = "file://".to_string();
    p.push_str(format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), path).as_str());
    p
}