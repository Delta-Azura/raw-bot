use anyhow::{Result, Context};
use std::fs;
use crate::checkconf::checkconf;
use rawfetch::getdistant;

pub fn auto() -> Result<()> {
    let toupdate = getdistant()?;
    let matcharch = fs::read_to_string("/etc/raw-fetch").context("Failed to read /etc/raw-fetch")?;
    let mut realname = String::new();
    let (mode, local, root) = checkconf().context("Failed to check configuration")?;
    let indexpath = format!("{}/index.raw", root);
    let indexcontent = fs::read_to_string(indexpath).context("Failed to read index.raw")?;
    for (name, version, release) in toupdate {
        for line in matcharch.lines() {
            if line.ends_with(&name) {
                realname = line.split_once("=").map(|(name, _)| name).context("Failed to get correct name in /etc/raw-fetch")?.to_string();
            } else {
                realname = name.to_string();
            }
        }
        let path = indexcontent.lines().find(|l| l.contains(&format!("{}/Pkgfile", realname))).context("Failed to get in index.raw")?.split_once("/Pkgfile").map(|(path, _)| path).context("Failed to get path to package file")?;
        let pkgfile = fs::read_to_string(format!("{}/{}/Pkgfile", root, path))?;
        let currentver = pkgfile.lines().find(|l| l.starts_with("version=")).context("Failed to find version line in pkgfile")?;
        let currentrel = pkgfile.lines().find(|l| l.starts_with("release=")).context("Failed to find release line in pkgfile")?;
        let modified = pkgfile
            .replace(currentver, &format!("version={}", version))
            .replace(currentrel, &format!("release={}", release));
        fs::write(format!("{}/{}/Pkgfile", root, path), modified).context("Failed to remove pkgfile")?;
        }
    Ok(())
}