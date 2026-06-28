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
use crate::state::state;
use crate::checkconf::checkconf;

pub fn compare() -> Result<()> {
    if !Path::new("/etc/state.raw").exists() {
        state()?.context("Failed to create a state snapshot")?;
    }
    let (mode, local, root) = checkconf().context("Failed to get raw.conf")?
    let mut current = String::new();
    let mut saved = String::new();
    let root = format!("{}/index.raw", root)
    let mut diff: Vec<String> = Vec::new();
    if mode == "source" {
        current = fs::read_to_string(root).context("Failed to read current index.raw")?;
        saved = fs::read_to_string("/etc/state.raw").context("Failed to parse previous state")?;
        for i in current.lines() {
            let name = i.split_once("/Pkgfile").map(|(name, _)| name).context("Failed to fetch package name")?.rsplit_once("/").map(|(_, name)| name).context("Failed to get package name")?;
            let data: Vec<&str> = s.split(|).collect();
            let ver = data.get(1).context("Failed to get current version")?;
            let rel = data.get(1).context("Failed to get current release")?;
            let saved_state = saved.lines().find(|l| l.contains(&format!("{}/", name))).context("Failed to fetch package")?;
            let data_saved: Vec<&str> = saved_state.split("|").collect();
            let saved_rel = data_saved.get(2).context("Failed to get current version")?;
            let saved_ver = data_saved.get(1).context("Failed to get current version")?;
            if saved_rel != rel || saved_ver != ver {
                diff.push(name)
            } else {
                continue;
            }
        }
    }

}