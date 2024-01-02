use clap::{App, AppSettings, Arg};
use std::{env, path::PathBuf, process::{Command, Stdio}, ops::ControlFlow, io::{self, BufRead}};

fn main() {
    let matches = App::new("set_node_version")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("Tips:   使用 run 参数时，应当避免与本程序的参数冲突，可以使用\"npm run dev\"包裹即将执行的命令")
        .author("Author: https://github.com/LazyFury")
        .subcommand(App::new("gen").about("生成 env.sh file")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("output")
                    
                    .short('o')
                    .long("output")
                    .value_name("output")
                    .help("set output file")
                    .default_missing_value("env.sh")
                    .default_value("env.sh")
                    .takes_value(true),
            ).arg(
                Arg::with_name("version")
                    .short('v')
                    .long("version")
                    .value_name("version")
                    .help("set node version")
                    .takes_value(true),
            )
        )
        .arg(
            Arg::with_name("version")
                .short('v')
                .long("version")
                .value_name("version")
                .help("set node version")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("run")
                .short('r')
                .long("run")
                .value_name("run")
                .help("run command")
                .multiple_values(true)
        )
        .clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("gen") {
        let output = matches.value_of("output").unwrap();
        let version = matches.value_of("version").unwrap();
        if let ControlFlow::Break(_) = gen_env_sh(version, output) {
            return;
        }
    }

    let version = matches.value_of("version").unwrap();
    println!("version: {}", version);
    let mut cmd: String = String::from("");
    if let Some(cmds) = matches.values_of("run") {
        cmd = cmds.collect::<Vec<_>>().join(" ");
    }

    // check n is installed
    let has_installed_n = has_installed_n();
    print!("has_installed_n: {:?}\n", has_installed_n);
    if !has_installed_n {
        panic!("n is not installed, please install n first")
    }

    let bin_dir = n_find_version(version);
    env::set_var("PATH", format!("{}:{}", bin_dir, env::var("PATH").unwrap()));
    print!("running : {:?}\n", cmd);
    // run cmd
    let output = Command::new("sh")
        .args(["-c", cmd.as_str()])
        .output()
        .expect("failed to execute process");
    println!("output: {:?}", output);
    println!("result: {}", String::from_utf8(output.stdout).unwrap());
}

fn gen_env_sh(version: &str, output: &str) -> ControlFlow<()> {
    let bin_dir = n_find_version(version);
    println!("bin_dir: {:?}", bin_dir);
    let sh = format!(r#"
export PATH={}:$PATH
echo set node version:
node -v
        "#, bin_dir);
    std::fs::write(output, sh).expect("write file failed");
    return ControlFlow::Break(());
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
        .spawn()?;

    if let Some(stdout) = cmd.stdout.take() {
        let  reader = io::BufReader::new(stdout);
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

