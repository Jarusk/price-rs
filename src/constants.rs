/*
 * Copyright (C) 2017  Yaraskavitch, Matt <yaraskavitch.matt@gmail.com>
 * Author: Yaraskavitch, Matt <yaraskavitch.matt@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub const CONFIG_NAME: &str = ".price-rs.conf";
pub const XCHG_NAME: &str = "price-rs-cache.json";
pub const URL_LATEST: &str = "http://openexchangerates.org/api/latest.json";
pub const DEFAULT_BASE_CURRENCY: &str = "USD";
pub const DEFAULT_TARGET_CURRENCY: &str = "CAD";
pub const DEFAULT_TAX_RATE: f32 = 1.13;
pub const APPLY_TAX: bool = true;
