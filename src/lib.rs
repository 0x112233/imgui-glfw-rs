use glfw::{Action, Key, Modifiers, MouseButton, StandardCursor, WindowEvent};
use imgui::sys as imgui_sys;
use imgui::{ImGui, ImGuiKey, ImGuiMouseCursor};
use std::time::Instant;

pub struct ImguiGLFW {
    last_frame: Instant,
    mouse_press: [bool; 5],
    cursor_pos: (f64, f64),
    _cursor: (ImGuiMouseCursor, Option<StandardCursor>),
}

impl ImguiGLFW {
    pub fn new(imgui: &mut ImGui) -> Self {
        {
            let io = unsafe { &mut *imgui_sys::igGetIO() };

            io.clipboard_user_data = std::ptr::null_mut();
        }

        {
            imgui.set_imgui_key(ImGuiKey::Tab, Key::Tab as u8);
            imgui.set_imgui_key(ImGuiKey::LeftArrow, Key::Left as u8);
            imgui.set_imgui_key(ImGuiKey::RightArrow, Key::Right as u8);
            imgui.set_imgui_key(ImGuiKey::UpArrow, Key::Up as u8);
            imgui.set_imgui_key(ImGuiKey::DownArrow, Key::Down as u8);
            imgui.set_imgui_key(ImGuiKey::PageUp, Key::PageUp as u8);
            imgui.set_imgui_key(ImGuiKey::PageDown, Key::PageDown as u8);
            imgui.set_imgui_key(ImGuiKey::Home, Key::Home as u8);
            imgui.set_imgui_key(ImGuiKey::End, Key::End as u8);
            imgui.set_imgui_key(ImGuiKey::Delete, Key::Delete as u8);
            imgui.set_imgui_key(ImGuiKey::Backspace, Key::Backspace as u8);
            imgui.set_imgui_key(ImGuiKey::Enter, Key::Enter as u8);
            imgui.set_imgui_key(ImGuiKey::Escape, Key::Escape as u8);
            imgui.set_imgui_key(ImGuiKey::A, Key::A as u8);
            imgui.set_imgui_key(ImGuiKey::C, Key::C as u8);
            imgui.set_imgui_key(ImGuiKey::V, Key::V as u8);
            imgui.set_imgui_key(ImGuiKey::X, Key::X as u8);
            imgui.set_imgui_key(ImGuiKey::Y, Key::Y as u8);
            imgui.set_imgui_key(ImGuiKey::Z, Key::Z as u8);
        }

        Self {
            last_frame: Instant::now(),
            mouse_press: [false; 5],
            cursor_pos: (0., 0.),
            _cursor: (ImGuiMouseCursor::None, None),
        }
    }

    pub fn handle_event(&mut self, imgui: &mut ImGui, event: &WindowEvent) {
        match *event {
            WindowEvent::MouseButton(mouse_btn, action, _) => {
                let index = match mouse_btn {
                    MouseButton::Button1 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                    _ => 0,
                };
                let press = action != Action::Release;
                self.mouse_press[index] = press;
                imgui.set_mouse_down(self.mouse_press);
            }
            WindowEvent::CursorPos(w, h) => {
                imgui.set_mouse_pos(w as f32, h as f32);
                self.cursor_pos = (w, h);
            }
            WindowEvent::Scroll(_, d) => {
                imgui.set_mouse_wheel(d as f32);
            }
            WindowEvent::Char(character) => {
                imgui.add_input_character(character);
            }
            WindowEvent::Key(_, scancode, action, modifier) => {
                Self::set_mod(imgui, modifier);
                if action != Action::Release {
                    imgui.set_key(scancode as u8, true);
                } else {
                    imgui.set_key(scancode as u8, false);
                }
            }
            _ => {}
        }
    }

    pub fn update<'a>(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        delta_s
    }

    fn set_mod(imgui: &mut ImGui, modifier: Modifiers) {
        let ctrl = modifier.intersects(Modifiers::Control);
        let alt = modifier.intersects(Modifiers::Alt);
        let shift = modifier.intersects(Modifiers::Shift);
        let super_ = modifier.intersects(Modifiers::Super);

        imgui.set_key_ctrl(ctrl);
        imgui.set_key_alt(alt);
        imgui.set_key_shift(shift);
        imgui.set_key_super(super_);
    }
}
