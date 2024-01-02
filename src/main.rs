use clap::{App, AppSettings, Arg, Subcommand};
use std::{env, io::Write, path::PathBuf, process::Command};

fn main() {
    let matches = App::new("set_node_version")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("Tips:   使用 run 参数时，应当避免与本程序的参数冲突，可以使用\"npm run dev\"包裹即将执行的命令")
        .author("Author: https://github.com/LazyFury")
        .subcommand(App::new("gen").about("生成 env.sh file")
            .arg(
                Arg::with_name("output")
                    .short('o')
                    .long("output")
                    .value_name("output")
                    .help("set output file")
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
        let bin_dir = n_find_version(version);
        println!("bin_dir: {:?}", bin_dir);
        let sh = format!(r#"
export PATH={}:$PATH
echo set node version:
node -v
        "#, bin_dir);
        std::fs::write(output, sh).expect("write file failed");
        return;
    }

    let version = matches.value_of("version").unwrap();
    println!("version: {}", version);
    let mut cmd: String = String::from("");
    if let Some(cmds) = matches.values_of("run") {
        cmd = cmds.collect::<Vec<_>>().join(" ");
    }

    // check n is installed
    let has_installed_n = has_installed_n();
    print!("has_installed_n: {:?}", has_installed_n);
    if !has_installed_n {
        panic!("n is not installed, please install n first")
    }

    let bin_dir = n_find_version(version);
    env::set_var("PATH", format!("{}:{}", bin_dir, env::var("PATH").unwrap()));
    print!("cmd : {:?}", cmd);
    // run cmd
    let output = Command::new("sh")
        .args(["-c", cmd.as_str()])
        .output()
        .expect("failed to execute process");
    println!("output: {:?}", output);
}

fn has_installed_n() -> bool {
    let output = Command::new("which")
        .args(["n"])
        .output()
        .expect("failed to execute process");
    return output.status.success();
}

fn n_has_installed_version(version: &str) -> bool {
    let output = Command::new("n")
        .args(["ls"])
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    return output.contains(version);
}

fn n_install(version: &str) {
    let output = Command::new("n")
        .args(["install", version])
        .output()
        .expect(format!("安装 node {:?} 失败!", version).as_str());
    println!("output: {:?}", output);
}

fn n_find_version(version: &str) -> String {
    let output = Command::new("n")
        .args(["bin",version])
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    let path_buf = PathBuf::from(output.trim());
    let bin_dir = path_buf.parent().unwrap();
    return bin_dir.to_str().unwrap().to_string();
}

fn gen_env_sh(version: &str) -> String {
    let current_dir = std::env::current_dir().unwrap();

    if !n_has_installed_version(version) {
        print!("n has not installed version: {:?}", version);
        n_install(version);
    }

    // nodeBin= xxx/bin/node try get bin dir
    let node_bin = n_find_version(version);
    let path_buf = PathBuf::from(node_bin);
    let bin_dir = path_buf.parent().unwrap();

    println!("bin_dir: {:?}", bin_dir);
    // export ./env.sh file set PATH
    let env_file = current_dir.join("env.sh");
    let env_file_str = env_file.to_str().unwrap();
    println!("env_file: {:?}", env_file_str);
    let f = std::fs::File::create(env_file_str).expect("create file failed");
    let mut f = std::io::BufWriter::new(f);
    let bin_dir_str = bin_dir.to_str().unwrap();
    let path = format!("export PATH={}:$PATH", bin_dir_str);
    // add node -v to ./env.sh
    let file: String = format!("{}\nnode -v", path);

    f.write_all(file.as_bytes()).expect("write file failed");
    f.flush().expect("flush file failed");
    return bin_dir_str.to_string();
}
