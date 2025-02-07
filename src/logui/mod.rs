use std::sync::{Arc, Mutex};

use crate::db::ChatDatabase;

use self::{buffer::LogBuffer, settings::ChatLogSettings};
use windows::System::VirtualKey;

pub mod buffer;
mod settings;
mod ui;

#[derive(Debug)]
struct LocalProps {
    pub account_filter: String,
    pub text_filter: String,
    pub account_width: f32,
}

impl LocalProps {
    pub fn new() -> Self {
        Self {
            account_filter: String::new(),
            text_filter: String::new(),
            account_width: 100.0,
        }
    }
}

pub struct LogUi {
    pub settings: ChatLogSettings,
    pub buffer: LogBuffer,
    pub chat_database: Option<Arc<Mutex<ChatDatabase>>>,
    ui_props: LocalProps,
}

impl LogUi {
    pub const DEFAULT_HOTKEY: u32 = VirtualKey::J.0 as u32;

    pub fn new() -> Self {
        Self {
            settings: ChatLogSettings::new(),
            buffer: LogBuffer::new(),
            chat_database: None,
            ui_props: LocalProps::new(),
        }
    }

    pub fn update_settings(&mut self) {
        self.buffer.colors = self.settings.color_settings;
        self.buffer.color_code_squad_updates = self.settings.color_code_squad_updates;
    }
}

impl Default for LogUi {
    fn default() -> Self {
        Self::new()
    }
}
