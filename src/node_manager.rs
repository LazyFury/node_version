use std::{process::{Command, Stdio}, io::{self, BufRead}, path::PathBuf};




#[cfg(target_os="macos")]
pub mod manager{

    use super::{n_find_version,has_installed_n};

    pub fn find_version(version: &str) -> String {
        n_find_version(version)
    }

    pub fn has_installed() -> bool {
        has_installed_n()
    }
}


#[cfg(target_os="linux")]
pub mod manager{
    use std::io;

    use super::{n_find_version, has_installed_n};

    pub fn find_version(version: &str) -> String {
        n_find_version(version)
    }

    pub fn has_installed() -> bool {
        has_installed_n()
    }
}



#[cfg(target_os="windows")]
pub mod manager{
    use std::{io, process::Command};

    pub fn find_version(version: &str) -> String {
        if !n_has_installed_version(version) {
            print!("n has not installed version: {:?}", version);
            n_install(version).expect("n install failed");
        }
        let cmd = Command::new("nvm")
            .args(["which", version])
            .output()
            .expect("failed to execute process 'nvm which'");
        let output = String::from_utf8(cmd.stdout).unwrap();
        return output.trim().to_string();
    }

    pub fn install(version: &str) -> io::Result<bool> {
        // nvm install node 
        let mut cmd = Command::new("nvm")
            .args(["install", version])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        if let Some(stdout) = cmd.stdout.take() {
            let  reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        if let Some(stderr) = cmd.stderr.take() {
            let  reader = io::BufReader::new(stderr);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        let status = cmd.wait()?;
        Ok(status.success())
    }

    pub fn has_installed_version(version: &str) -> bool {
        // use nvm check node version is installed 
        let versions = Command::new("nvm")
            .args(["list"])
            .output()
            .expect("failed to check has_installed_n");
        let output = String::from_utf8(versions.stdout).unwrap();
        return output.contains(version);
    }

    pub fn has_installed() -> bool {
        // has_installed_n()
        // check nvm is install 
        let is_installed = Command::new("where")
            .args(["nvm"])
            .output()
            .expect("failed to check has_installed_n");
        return is_installed.status.success();
    }
}



fn has_installed_n() -> bool {
    let output = Command::new("which")
        .args(["n"])
        .output()
        .expect("failed to check has_installed_n");
    return output.status.success();
}

fn n_has_installed_version(version: &str) -> bool {
    let output = Command::new("n")
        .args(["ls"])
        .output()
        .expect("failed to check n_has_installed_version");
    let output = String::from_utf8(output.stdout).unwrap();
    return output.contains(version);
}

fn n_install(version: &str) -> io::Result<bool> {
    let mut cmd = Command::new("n")
        .args(["install", version])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = cmd.stdout.take() {
        let  reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    if let Some(stderr) = cmd.stderr.take() {
        let  reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    let status = cmd.wait()?;
    Ok(status.success())
}

fn n_find_version(version: &str) -> String {
    if !n_has_installed_version(version) {
        print!("n has not installed version: {:?}", version);
        n_install(version).expect("n install failed");
    }

    let output = Command::new("n")
        .args(["bin",version])
        .output()
        .expect("failed to execute process 'n bin'");
    let output = String::from_utf8(output.stdout).unwrap();
    let path_buf = PathBuf::from(output.trim());
    let bin_dir = path_buf.parent().unwrap();
    return bin_dir.to_str().unwrap().to_string();
}