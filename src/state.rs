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

pub fn state() -> Result<()> {
    if Path::new("/etc/state.raw").exists() {
        fs::remove_file("/etc/state.raw")
    }
    let (mode, local, root) = checkconf().context("Failed to get current configuration")?;
    if mode == "binary" {
        if local == true {
            fs::copy(format!("{}/index.raw", "/etc/state.raw")).context("Failed to copy index to save the current state")?;
            Ok()
        } else {
            anyhow::bail!("Local variable isn't set to true, exiting");
        }
    }
    if mode == "source" {
        
    }

}