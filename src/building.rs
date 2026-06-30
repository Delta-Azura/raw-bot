// Raw-bot is an experimental automatic bot that pushes to compilation any modified package.
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use anyhow::{Result, Context};
use std::fs;
use raw::package;
use std::env;

pub fn building(path: String) -> Result<()> {
    //let configuration = fs::read_to_string("/etc/raw-bot/building").context("Shell script doesn't exist")?;
    env::set_current_dir(&path)?;
    match package(None) {
        Ok(_) => {
            println!("Building succeded in {}", path);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    return Ok(());
}