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

pub fn checkconf() -> Result<(String, String, String)> {
    if !Path::new("/etc/raw.conf").exists() {
        anyhow::bail!("Exiting as raw.conf isn't configured")
    }
    let rawconf = fs::read_to_string("/etc/raw.conf").context("Failed to read /etc/raw.conf")?;
    let mut mode = String::new();
    let mut local: bool;
    let mut root: String,
    if rawconf.contains("mode=binary") {
        mode = "binary"
        if rawconf.lines().any(|l| l.contains("local=true")) {
            if rawconf.lines().find(|l| l.contains("local=true")).context("Failed to fetch the local line")?.starts_with("#") {
                local = false;
            } else {
                local = true;
                root = rawconf.lines().find(|l| l.starts_with("root=")).context("Failed to check local repository")?.map(|(_, root)| root).context("Failed to get local path")?;
            }
        }
        
    }
    if rawconf.contains("mode=source") {
        mode = "source";
        root = rawconf.lines().find(|l| l.starts_with("root=")).context("Failed to get path to pkgfiles")?.map(|(_, root)| root).context("Failed to get root path")?;
        local = false;
    }
    Ok((mode, local, root))
}