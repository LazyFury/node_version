use std::{process::Command, path::PathBuf, io::Write, env};
use clap::{App, Arg, AppSettings};

fn main(){
    let matches = App::new("set_node_version")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("Tips:   使用 run 参数时，应当避免与本程序的参数冲突，可以使用\"npm run dev\"包裹即将执行的命令")
        .author("Author: https://github.com/LazyFury")
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

    let version = matches.value_of("version").unwrap();
    println!("version: {}", version);
    let mut cmd:String = String::from("");
    if let Some(cmds) = matches.values_of("run") {
        cmd = cmds.collect::<Vec<_>>().join(" ");
    }
    
    let bin_dir = gen_env_sh(version);
    env::set_var("PATH", format!("{}:{}", bin_dir, env::var("PATH").unwrap()));
    print!("cmd : {:?}", cmd);
    // run cmd 
    let output = Command::new("sh")
        .args([
            "-c",
            cmd.as_str()
        ])
        .output()
        .expect("failed to execute process");
    println!("output: {:?}", output);
}



fn gen_env_sh(version:&str) -> String{
    let current_dir = std::env::current_dir().unwrap();
    println!("Hello, world!");
    let node_bin = Command::new("n")
        .args([
            "bin",
            version
        ]).output().expect("failed to execute process");
    println!("nodeBin: {:?}", node_bin);
    // nodeBin= xxx/bin/node try get bin dir 
    let node_bin = String::from_utf8(node_bin.stdout).unwrap();
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
    return bin_dir_str.to_string()
}
