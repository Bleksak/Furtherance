// Furtherance - Track your time without being tracked
// Copyright (C) 2025  Ricky Kresslein <rk@unobserved.io>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use plotters::style::{
    full_palette::{BLACK, WHITE},
    RGBColor,
};

pub fn light_dark_color() -> RGBColor {
    match dark_light::detect() {
        Ok(mode) => match mode {
            dark_light::Mode::Light | dark_light::Mode::Unspecified => BLACK,
            dark_light::Mode::Dark => WHITE,
        },
        Err(_) => BLACK,
    }
}
