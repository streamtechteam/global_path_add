use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let local_dir_path = String::from(format!(
        "{}/.gpath_add/vars/",
        env::home_dir().expect("").to_str().expect("")
    ));
    let local_fish_path = String::from(format!(
        "{}/.gpath_add/vars/fish.sh",
        env::home_dir().expect("").to_str().expect("")
    ));
    let local_bash_path = String::from(format!(
        "{}/.gpath_add/vars/bash.sh",
        env::home_dir().expect("").to_str().expect("")
    ));
    let local_vars_path = String::from(format!(
        "{}/.gpath_add/vars/vars.sh",
        env::home_dir().expect("").to_str().expect("")
    ));
    let bashrc_path = String::from(env::home_dir().expect("").to_str().expect("")) + "/.bashrc";
    let mut is_everything_ok = false;

    if check_if_arg_exist(&args) {
        if check_if_starts(&bashrc_path, &local_vars_path) {
            println!("Bashrc already contains the Source line for vars.sh");
            if check_if_dir_and_files_exist(
                String::clone(&local_dir_path),
                String::clone(&local_bash_path),
                String::clone(&local_fish_path),
                String::clone(&local_vars_path),
            ) == true
            {
                println!("Bash files exist , Adding to path");
                is_everything_ok = true;
            } else {
                println!("Bash files not found, Creating them now");
                create_config_dir_and_files(
                    String::clone(&local_dir_path),
                    String::clone(&local_bash_path),
                    String::clone(&local_fish_path),
                    String::clone(&local_vars_path),
                );
            }
        } else {
            println!("Bashrc does not contain the Source line for vars.sh \n Adding it now.");
            write_to_bashrc(&bashrc_path);
            if check_if_dir_and_files_exist(
                String::clone(&local_dir_path),
                String::clone(&local_bash_path),
                String::clone(&local_fish_path),
                String::clone(&local_vars_path),
            ) == true
            {
                println!("Bash files exist , Adding to path");
                is_everything_ok = true;
            } else {
                println!("Bash files not found, Creating them now");
                create_config_dir_and_files(
                    String::clone(&local_dir_path),
                    String::clone(&local_bash_path),
                    String::clone(&local_fish_path),
                    String::clone(&local_vars_path),
                );
            }
        }
    } else {
        println!("No argument provided. Please provide a path to add to the global PATH.");
    }

    if is_everything_ok {
        let perv = read_from_local(&local_bash_path, &local_fish_path);
        write_to_local(
            &local_bash_path,
            &local_fish_path,
            &local_vars_path,
            format!(
                "{} \n fish -c 'set -Ux PATH \"{}\" $PATH'",
                perv[1], &args[1]
            ),
            format!("{} \n export PATH=\"{}:$PATH\"", perv[0], &args[1]),
            format!(
                "source \"{}\" \n source \"{}\"",
                local_bash_path, local_fish_path
            ),
        );
    }

    println!("[DEBUG] Bash files path : ");
    println!("{local_bash_path}");
    println!("{local_fish_path}");
    println!("{local_vars_path}");
}

fn check_if_starts(bashrc_path: &String, local_var_path: &String) -> bool {
    if fs::read_to_string(&bashrc_path)
        .expect("error on checking bashrc")
        .contains(format!("source \"{}\"", local_var_path).as_str())
        && !fs::read_to_string(&bashrc_path)
            .expect("error on checking bashrc")
            .contains(format!("#source {}", local_var_path).as_str())
    {
        // println!("yeah");
        return true;
    } else {
        // println!("no");
        return false;
    }
}

fn check_if_dir_and_files_exist(
    local_dir_path: String,
    local_bash_path: String,
    local_fish_path: String,
    local_vars_path: String,
) -> bool {
    if fs::exists(local_dir_path).expect("err") {
    } else {
        return false;
    }
    if fs::exists(local_vars_path).expect("Err") {
    } else {
        return false;
    }
    if fs::exists(local_bash_path).expect("err") {
    } else {
        return false;
    }
    if fs::exists(local_fish_path).expect("err") {
        return true;
    } else {
        return false;
    }
}

fn create_config_dir_and_files(
    local_dir_path: String,
    local_bash_path: String,
    local_fish_path: String,
    local_vars_path: String,
) {
    fs::create_dir_all(local_dir_path).expect("Error when creating dir");
    fs::File::create(local_bash_path).expect("error when making fish file");
    fs::File::create(local_fish_path).expect("error when making bash file");
    fs::File::create(local_vars_path).expect("error when making vars file");
}

fn check_if_arg_exist(args: &Vec<String>) -> bool {
    if args.len() >= 2 {
        // let path = args[1].clone();
        return true;
        // let perv = localbash;
        // fs::write(
        //     local_bash_path,
        //     String::from(perv + "\n" + "export PATH=\"" + &path + ":$PATH\"")
        // ).expect("msg");
        // if path.starts_with("/") || path.starts_with("~") || path.starts_with("./") {
        // let output = process::Command
        //     ::new("sh")
        //     .arg("-c")
        //     .arg("cat ~/.bashrc | grep PATH")
        //     .output()
        //     .expect("F..k");

        // if output.status.success() {
        //     let stdout = String::from_utf8_lossy(&output.stdout);
        //     println!("Output:\n{}", stdout);
        // }

        // println!("path: {}", path);
        // }
    } else {
        return false;
    }
}

fn read_from_local(local_bash_path: &String, local_fish_path: &String) -> [String; 2] {
    let local_fish = fs::read_to_string(&local_fish_path).expect("local fish sh file not found");
    let local_bash = fs::read_to_string(&local_bash_path).expect("local bash sh file not found");

    return [local_bash, local_fish];
}

fn write_to_local(
    local_bash_path: &String,
    local_fish_path: &String,
    local_vars_path: &String,
    fish_value: String,
    bash_value: String,
    vars_value: String,
) {
    fs::write(&local_bash_path, bash_value).expect("Failed to write to local bash file");
    fs::write(&local_fish_path, fish_value).expect("Failed to write to local fish file");
    fs::write(&local_vars_path, vars_value).expect("Failed to write to local vars file");
}

fn write_to_bashrc(bashrc_path: &String) {
    // if (!check_if_starts(&bashrc_path)){
    let prev = fs::read_to_string(&bashrc_path).expect("could not read bashrc");
    fs::write(
        &bashrc_path,
        prev + "\n"
            + format!(
                "source \"{}/.gpath_add/vars/vars.sh\"",
                env::home_dir().expect("").to_str().expect("")
            )
            .as_str(),
    )
    .expect("could not write to bashrc");
    // }
}
