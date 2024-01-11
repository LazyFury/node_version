use ansi_term::Color;
use clap::{App, AppSettings, Arg};
use std::{env, process::{Command, Stdio}, io::{self, Read, Write}};
mod source_file;
mod node_manager;
use node_manager::manager;

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
        return source_file::gen(version, output)
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
    let has_installed_n = manager::has_installed();
    print!("{}: {:?}\n",Color::Blue.paint("Has Installed n Package"), has_installed_n);
    if !has_installed_n {
        panic!("n is not installed, please install n first")
    }

    println!("{}",Color::Blue.paint("Find Nodejs Version Path..."));
    let bin_dir = manager::find_version(version);
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
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let mut stdout_reader = io::BufReader::new(stdout);
        let mut stderr_reader = io::BufReader::new(stderr);

        let out_thread = std::thread::spawn(move || {
            let mut buffer = [0;1024];
            loop{
                let n = stdout_reader.read(&mut buffer).unwrap();
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buffer[..n]);
                // split to arr 
                let lines:Vec<&str> = s.split(|c| c=='\n' || c=='\r')
                            .filter(|&line| !line.is_empty())
                            .collect();
                for line in lines {
                    io::stdout().flush().unwrap();
                    // let line = line.replace("\r", "");
                    print!("{} {}\n", Color::Green.paint("[stdout]"),line);
                }
            }
        });

        let err_thread = std::thread::spawn(move || {
            let mut buffer = [0;1024];
            loop{
                let n = stderr_reader.read(&mut buffer).unwrap();
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buffer[..n]);
                // split to arr
                let lines:Vec<&str> = s.split(|c| c=='\n' || c=='\r')
                            .filter(|&line| !line.is_empty())
                            .collect();
                for line in lines {
                    io::stderr().flush().unwrap();
                    // let line = line.replace("\r", "");
                    print!("{} {}\n", Color::Red.paint("[stderr]"),line);
                }
            }
        });

        
        let status = child.wait().unwrap();
        if !status.success() {
            panic!("run command failed");
        }

        out_thread.join().unwrap();
        err_thread.join().unwrap();
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();

        // handler ctrl c 
        ctrlc::set_handler(move || {
            println!("{}: {}", Color::Red.paint("Ctrl C"), "stop child process");
            child.kill().unwrap();
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");


    } else {
        panic!("run command failed");
    }
}










