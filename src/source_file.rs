use crate::node_manager::manager;


fn gen_env_sh_linux_or_macos(version: &str, output: &str) {
    let bin_dir = manager::find_version(version);
    println!("bin_dir: {:?}", bin_dir);
    let sh = format!(r#"
export PATH={}:$PATH
echo set node version:
node -v
        "#, bin_dir);
    std::fs::write(output, sh).expect("write file failed");
}

#[cfg(target_os="macos")]
pub fn gen(version: &str, output: &str) {
    gen_env_sh_linux_or_macos(version, output)
}

#[cfg(target_os="linux")]
pub fn gen(version: &str, output: &str) {
    gen_env_sh_linux_or_macos(version, output)
}

#[cfg(target_os="windows")]
pub fn gen(version: &str, output: &str) {
    // gen a cmd file 
    let bin_dir = manager::find_version(version);
    let cmd = format!(r#"
@echo off
set PATH={};%PATH%
echo set node version:
node -v
        "#, bin_dir);
    std::fs::write(output, cmd).expect("write file failed");
}