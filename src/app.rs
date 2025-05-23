use crate::gui;

use device_query::{DeviceQuery, DeviceState, Keycode};
use eframe::{egui::Context, App, Frame};
use enigo::{Enigo, Key, KeyboardControllable};
use inputbot::MouseButton;
use screenshots::Screen;

use std::{
  cmp::PartialEq,
  fmt::{Display, Formatter, Result},
  thread,
  time::Duration,
};

#[derive(PartialEq, Debug)]
pub enum TriggerKey {
  Keyboard(Keycode),
  Mouse(MouseButton),
}

#[derive(PartialEq)]
pub struct Settings {
  pub resolution: Resolution,
  pub trigger_keys: Vec<TriggerKey>,
  pub trigger_delay: u64,
  pub trigger_area: f32,
  pub target_color: [i32; 3],
  pub color_tolerance: i32,
  pub always_open: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      resolution: Resolution {
        width: 3840,
        height: 2160,
      },
      trigger_keys: vec![TriggerKey::Keyboard(Keycode::LShift)],
      trigger_delay: 50,
      trigger_area: 10.0,
      target_color: [240, 90, 255],
      color_tolerance: 50,
      always_open: false,
    }
  }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Resolution {
  pub width: u32,
  pub height: u32,
}

pub struct TriggerArea {
  pub x_percent: f32,
  pub y_percent: f32,
  pub width_percent: f32,
  pub height_percent: f32,
}

pub struct Triggerbot {
  pub enabled: bool,
  pub device_state: DeviceState,
  pub screen: Screen,
  pub trigger_area: TriggerArea,
  pub enigo: Enigo,
  pub settings: Settings,
}

impl Triggerbot {
  pub fn default() -> Self {
    let mut triggerbot = Self {
      enabled: false,
      device_state: DeviceState::new(),
      screen: Screen::from_point(0, 0).unwrap(),
      trigger_area: TriggerArea {
        x_percent: 0.0,
        y_percent: 0.0,
        width_percent: 0.0,
        height_percent: 0.0,
      },
      enigo: Enigo::new(),
      settings: Settings::default(),
    };

    triggerbot.update_trigger_area();

    triggerbot
  }

  pub fn reset_settings(&mut self) {
    self.settings = Settings::default();
    self.update_trigger_area();
  }

  pub fn is_default_settings(&self) -> bool {
    self.settings == Settings::default()
  }

  pub fn set_resolution(&mut self, width: u32, height: u32) {
    self.settings.resolution = Resolution { width, height };

    self.update_trigger_area();
  }

  pub fn triggerbot(&mut self) {
    let keys = self.device_state.get_keys();

    let trigger_active = !self.settings.always_open
      && self
        .settings
        .trigger_keys
        .iter()
        .any(|trigger| match trigger {
          TriggerKey::Keyboard(key) => keys.contains(key),
          TriggerKey::Mouse(button) => button.is_pressed(),
        });

    if self.enabled && (trigger_active || self.settings.always_open) && self.should_trigger() {
      self.enigo.key_click(Key::Layout('k'));

      if self.settings.trigger_delay > 0 {
        thread::sleep(Duration::from_millis(self.settings.trigger_delay));
      }
    }
  }

  fn should_trigger(&self) -> bool {
    let (width, height) = (
      self.settings.resolution.width,
      self.settings.resolution.height,
    );

    let x = (self.trigger_area.x_percent * width as f32) as i32;
    let y = (self.trigger_area.y_percent * height as f32) as i32;
    let w = (self.trigger_area.width_percent * width as f32) as u32;
    let h = 1 as u32;

    let capture = self.screen.capture_area(x, y, w, h).unwrap();
    let pixels = capture.pixels();

    let capture_width = w as usize;
    let center_x = capture_width / 2;

    let mut left_found = false;
    let mut right_found = false;
    let mut left_pos = capture_width;
    let mut right_pos = 0;
    let mut pixelindex = 0;

    pixels.clone().for_each(|pixel| {
      // Compare each channel with tolerance
      if pixel
        .0
        .iter()
        .zip(&self.settings.target_color)
        .all(|(a, b)| ((*a as i32) - b).abs() <= self.settings.color_tolerance)
      {
        if pixelindex <= center_x {
          left_found = true;
          if pixelindex < left_pos {
            left_pos = pixelindex;
          }
        }
        if pixelindex >= center_x {
          right_found = true;
          if pixelindex > right_pos {
            right_pos = pixelindex;
          }
        }
      }

      pixelindex += 1;
    });

    // NOTE: the left and right boundaries are unused.
    // one might be tempted to move the mouse a number of pixels based on this information.
    // doing so might get you banned.

    if left_found && right_found {
      return true;
    } else {
      return false;
    }
  }

  pub fn update_trigger_area(&mut self) {
    let (width, height) = (
      self.settings.resolution.width,
      self.settings.resolution.height,
    );

    self.trigger_area = TriggerArea {
      x_percent: 0.5 - (self.settings.trigger_area / 2.0 / width as f32),
      y_percent: 0.5 - (self.settings.trigger_area / 2.0 / height as f32),
      width_percent: self.settings.trigger_area / width as f32,
      height_percent: self.settings.trigger_area / height as f32,
    };
  }

  pub fn get_keys(&self) -> Vec<TriggerKey> {
    let mut triggers = vec![
      TriggerKey::Mouse(MouseButton::LeftButton),
      TriggerKey::Mouse(MouseButton::MiddleButton),
      TriggerKey::Mouse(MouseButton::RightButton),
      TriggerKey::Mouse(MouseButton::X1Button),
      TriggerKey::Mouse(MouseButton::X2Button),
    ];

    for key in vec![
      Keycode::LShift,
      Keycode::RShift,
      Keycode::LControl,
      Keycode::RControl,
      Keycode::LAlt,
      Keycode::RAlt,
      Keycode::A,
      Keycode::F,
      Keycode::B,
      Keycode::G,
      Keycode::C,
      Keycode::H,
      Keycode::D,
      Keycode::I,
      Keycode::E,
      Keycode::J,
      Keycode::K,
      Keycode::P,
      Keycode::L,
      Keycode::Q,
      Keycode::M,
      Keycode::R,
      Keycode::N,
      Keycode::S,
      Keycode::O,
      Keycode::T,
      Keycode::U,
      Keycode::Z,
      Keycode::V,
      Keycode::Y,
      Keycode::W,
      Keycode::X,
    ] {
      triggers.push(TriggerKey::Keyboard(key));
    }

    triggers
  }

  pub fn get_keys_display_name(&self, trigger: &TriggerKey) -> String {
    match trigger {
      TriggerKey::Keyboard(key) => match key {
        Keycode::LShift => "Left Shift".to_string(),
        Keycode::RShift => "Right Shift".to_string(),
        Keycode::LControl => "Left Control".to_string(),
        Keycode::RControl => "Right Control".to_string(),
        Keycode::LAlt => "Left Alt".to_string(),
        Keycode::RAlt => "Right Alt".to_string(),
        _ => format!("{:?}", key),
      },
      TriggerKey::Mouse(key) => match key {
        MouseButton::LeftButton => "Mouse Left".to_string(),
        MouseButton::MiddleButton => "Mouse Middle (Wheel)".to_string(),
        MouseButton::RightButton => "Mouse Right".to_string(),
        MouseButton::X1Button => "Mouse Backward (X1)".to_string(),
        MouseButton::X2Button => "Mouse Forward (X2)".to_string(),
        _ => format!("{:?}", key),
      },
    }
  }
}

impl App for Triggerbot {
  fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
    gui::build(self, ctx);

    self.triggerbot();

    ctx.request_repaint();
  }
}

impl Display for Resolution {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}
