use std::ffi::{c_char, CStr};

use raylib::prelude::*;
use raylib_sys::{Color as FfiColor, GuiColorPicker};

pub trait Ent {
    fn is_colliding(&mut self, other: Vector2) -> bool;
}

#[derive(Clone, Debug)]
pub struct Pixel {
    center: Vector2,
    radius: f32,
    color: Color,
}

impl Ent for Pixel {
    fn is_colliding(&mut self, other: Vector2) -> bool {
        let distance = self.center.distance_to(other);
        if distance < 0.5 {
            return true;
        } else {
            return false;
        }
    }
}

impl Ent for Vector2 {
    fn is_colliding(&mut self, other: Vector2) -> bool {
        if self.distance_to(other) < 0.5 {
            return true;
        } else {
            return false;
        }
    }
}

pub struct Button {
    rect: Rectangle,
    button_type: &'static CStr,
}
impl Button {
    fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && self.rect.check_collision_point_rec(rl.get_mouse_position())
    }
}

pub struct World<T: Ent> {
    tiles: Vec<T>,
}

#[derive(PartialEq, Debug)]
pub enum LineState {
    Firstpos,
    Secondpos(Vector2),
}

#[derive(PartialEq, Debug)]
pub enum ToolbarState {
    Brush,
    Line,
    Poly,
    Erase,
}

pub fn toolbar_input<'a>(
    rl: &mut RaylibDrawHandle,
    mut tool_state: &'a mut ToolbarState,
    buttons: &[Button],
) -> () {
    for (i, button) in buttons.iter().enumerate() {
        if button.is_clicked(rl) {
            match i {
                0 => {
                    *tool_state = ToolbarState::Brush;
                }
                1 => {
                    *tool_state = ToolbarState::Line;
                }
                2 => {
                    *tool_state = ToolbarState::Poly;
                }
                3 => {
                    *tool_state = ToolbarState::Erase;
                }
                _ => {}
            }
        }
    }
}

pub fn draw_with_brush(
    mut rl: &mut RaylibDrawHandle,
    mut tiles: &mut World<Pixel>,
    color: &Color,
) -> () {
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let click_pos = rl.get_mouse_position();
        let pixel = Pixel {
            center: click_pos.clone(),
            radius: 10.0,
            color: color.clone(),
        };
        tiles.tiles.push(pixel.clone());
    }
}

pub fn draw_line_after_inputs<T: Ent>(
    mut rl: &mut RaylibDrawHandle,
    mut tiles: &mut World<T>,
    mut line_state: &mut LineState,
) -> () {
    todo!()
}
pub fn erase() {
    todo!()
}
pub fn into_ffi_color(mut color: Color) -> FfiColor {
    FfiColor {
        r: color.r.clone(),
        g: color.g.clone(),
        b: color.b.clone(),
        a: color.a.clone(),
    }
}
pub fn into_core_color(color: FfiColor) -> Color {
    Color {
        r: color.r.clone(),
        g: color.g.clone(),
        b: color.b.clone(),
        a: color.a.clone(),
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().width(800).height(600).build();
    let mut red_val = 0.0;
    let mut green_val = 0.0;
    let mut blue_val = 0.0;
    let mut color = raylib::prelude::core::color::Color::new(
        red_val as u8,
        green_val as u8,
        blue_val as u8,
        255,
    );
    let bar_left = Rectangle::new(0.0, 0.0, 200.0, rl.get_screen_height() as f32);
    let bar_right = Rectangle::new(
        (rl.get_screen_width() - 200) as f32,
        0.0,
        200.0,
        rl.get_screen_height() as f32,
    );
    let mut tiles: World<Pixel> = World { tiles: Vec::new() };
    rl.set_target_fps(90);
    let mut tool_state = ToolbarState::Line;
    let red_slider = Rectangle::new((rl.get_screen_width() - 120) as f32, 160.0, 120.0, 20.0);
    let green_slider = Rectangle::new((rl.get_screen_width() - 120) as f32, 280.0, 120.0, 20.0);
    let blue_slider = Rectangle::new((rl.get_screen_width() - 120) as f32, 400.0, 120.0, 20.0);
    let line = Rectangle::new(10.0, 100., 80.0, 80.0);
    let poly = Rectangle::new(10.0, 180., 80.0, 80.0);
    let erase = Rectangle::new(10.0, 260., 80.0, 80.0);
    let color_picker = Rectangle::new((rl.get_screen_width() - 120) as f32, 20., 120.0, 120.0);

    let buttons = [
        Button {
            rect: Rectangle::new(10.0, 20.0, 80.0, 80.0),
            button_type: rstr!("Brush"),
        },
        Button {
            rect: line.clone(),
            button_type: rstr!("Line"),
        },
        Button {
            rect: poly.clone(),
            button_type: rstr!("Poly"),
        },
        Button {
            rect: erase.clone(),
            button_type: rstr!("Erase"),
        },
    ];

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::GRAY);
        draw.gui_panel(bar_left, None);
        draw.gui_panel(bar_right, None);

        draw.gui_slider_bar(
            red_slider,
            Some(rstr!("Red")),
            None,
            &mut red_val,
            0.0,
            255.0,
        );
        draw.gui_slider_bar(
            green_slider,
            Some(rstr!("Green")),
            None,
            &mut green_val,
            0.0,
            255.0,
        );
        draw.gui_slider_bar(
            blue_slider,
            Some(rstr!("Blue")),
            None,
            &mut blue_val,
            0.0,
            255.0,
        );
        color = Color::new(red_val as u8, green_val as u8, blue_val as u8, 255);
        let mut to_ffi: FfiColor = into_ffi_color(color);
        unsafe { GuiColorPicker(color_picker.into(), std::ptr::null(), &mut to_ffi.clone()) };

        let mut picker_color = into_core_color(to_ffi);

        if picker_color != color {
            red_val = picker_color.r as f32;
            green_val = picker_color.g as f32;
            blue_val = picker_color.b as f32;
            color = picker_color; // Update current color with picker color
        }
        color;

        draw.gui_button(buttons[0].rect.clone(), Some(buttons[0].button_type));
        draw.gui_button(line.clone(), Some(buttons[1].button_type));
        draw.gui_button(poly.clone(), Some(buttons[2].button_type));
        draw.gui_button(erase.clone(), Some(buttons[3].button_type));
        draw.gui_set_style(
            GuiControl::DEFAULT,
            GuiDefaultProperty::BACKGROUND_COLOR as i32,
            GuiDefaultProperty::LINE_COLOR as i32,
        );
        let tool_input = toolbar_input(&mut draw, &mut tool_state, &buttons);
        match tool_state {
            ToolbarState::Brush => {
                // while mouse is not over ui nodes let user draw on click
                if bar_left.check_collision_point_rec(draw.get_mouse_position()) == false
                    && bar_right.check_collision_point_rec(draw.get_mouse_position()) == false
                {
                    draw_with_brush(&mut draw, &mut tiles, &mut into_core_color(to_ffi).into());
                }
            }
            ToolbarState::Line => {
                let mut line_state = LineState::Firstpos;
                dbg!(line_state);
            }
            ToolbarState::Poly => {
                dbg!(ToolbarState::Poly);
            }
            ToolbarState::Erase => {
                todo!()
            }
        }
        tiles
            .tiles
            .iter()
            .for_each(|item| draw.draw_circle_v(item.center, item.radius, item.color));
    }
}
