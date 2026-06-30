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
use std::fs::File;
use std::path::Path;

pub fn building(path: String) -> Result<()> {
    env::set_current_dir(&path)?;
    let pkgfile = fs::read_to_string("Pkgfile")?;
    let name = pkgfile.lines().find(|l| l.starts_with("name=")).context("Failed to get package name")?.split_once("name=").map(|(_, name)| name).context("Failed to get package name")?;
    let version = pkgfile.lines().find(|l| l.starts_with("version=")).context("Failed to get version line")?.split_once("version=").map(|(_, version)| version).context("Failed to get updated version")?;
    let release = pkgfile.lines().find(|l| l.starts_with("release=")).context("Failed to get release line")?.split_once("release=").map(|(_, release)| release).context("Failed to get updated release")?;

    if !Path::new("/var/cache/raw-bot.log").exists() {
        File::create("/var/cache/raw-bot.log")?;
    }
    let log = fs::read_to_string("/var/cache/raw-bot.log")?;
    match package(None) {
        Ok(_) => {
            println!("Building succeded in {}", path);
            if log.contains(&path) {
                let log = log.lines().find(|l| l.starts_with(&path)).context("Failed to get the line")?;
                let modified = log.replace(log, &format!("{}|{}|{}|succeded", path, version, release));
                fs::write("/var/cache/raw-bot.log", modified).context("Failed to log properly")?;
            }
        }
        Err(e) => {
            File::create(format!("/var/cache/raw-bot.d/{}.log", name)).context("Failed to create log file")?;
            fs::write(format!("/var/cache/raw-bot.d/{}.log", name), e.to_string()).context("Failed to write the log error")?;
            let log = log.lines().find(|l| l.starts_with(&path)).context("Failed to get the line")?;
            let modified = log.replace(log, &format!("{}|{}|{}|failed", path, version, release));
            fs::write("/var/cache/raw-bot.log", modified).context("Failed to log properly")?;
        }
    }
    return Ok(());
}