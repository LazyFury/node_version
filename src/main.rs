use ansi_term::Color;
use clap::{App, AppSettings, Arg};
use std::{env, path::PathBuf, process::{Command, Stdio}, io::{self, BufRead}};

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
        return gen_env_sh(version, output)
    }

    default_command(matches); 

}

fn default_command(matches: clap::ArgMatches) {
    let version: &str = matches.value_of("version").unwrap();
    let mut cmd: String = String::from("");
    if let Some(cmds) = matches.values_of("run") {
        cmd = cmds.collect::<Vec<_>>().join(" ");
    }
    if cmd == "" {
        println!("please set --run command");
        return;
    }

    run_command(version, cmd)
}

fn run_command(version: &str, cmd: String){
    // check n is installed
    let has_installed_n = has_installed_n();
    print!("{}: {:?}\n",Color::Blue.paint("Has Installed n Package"), has_installed_n);
    if !has_installed_n {
        panic!("n is not installed, please install n first")
    }

    println!("{}",Color::Blue.paint("Find Nodejs Version Path..."));
    let bin_dir = n_find_version(version);
    println!("{}: {}",Color::Blue.paint("Nodejs Version"), version);

    env::set_var("PATH", format!("{}:{}", bin_dir, env::var("PATH").unwrap()));
    print!("{} : {:?}\n",Color::Green.paint("Running") , cmd);
    // run cmd
    let output = Command::new("sh")
        .args(["-c", cmd.as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    if let Ok(mut child) = output {
        if let Some(stdout) = child.stdout.take() {
            let  reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                println!("{} {}", Color::Green.bold().paint("[stdout]"),line.unwrap());
            }
        }
        if let Some(stderr) = child.stderr.take() {
        
            let  reader = io::BufReader::new(stderr);
            // fix \r not read

            for line in reader.lines() {
                println!("{} {}",Color::Red.blink().paint("[stderr]"), line.unwrap());
            }
        }
        let status = child.wait().unwrap();
        if !status.success() {
            panic!("run command failed");
        }
    } else {
        panic!("run command failed");
    }
}

fn gen_env_sh(version: &str, output: &str) {
    let bin_dir = n_find_version(version);
    println!("bin_dir: {:?}", bin_dir);
    let sh = format!(r#"
export PATH={}:$PATH
echo set node version:
node -v
        "#, bin_dir);
    std::fs::write(output, sh).expect("write file failed");
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

