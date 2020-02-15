pub fn create_auto_complete_files() -> bool {
    use platform_dirs::{AppDirs, AppUI};
    use std::fs;

    let config_dir = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
        .unwrap()
        .config_dir;

    let shell_env_var = std::env::var("SHELL").expect("could not get SHELL env var");
    let shell_name: &str = shell_env_var.split("/").last().unwrap();

    match shell_name {
        "zsh" => {
            let auto_complete_zsh = {
                let mut x = config_dir.clone();
                x.push("_task.zsh");
                x
            };

            if !auto_complete_zsh.exists() {
                println!(
                    r#"
It looks like this is the first time you've run hypertask!
An auto completion file has been created for you at `{zc_file}`.
Please add the following snippet in your .zshrc file to enable auto completions for task

source {zc_file};

(hypertask will run as normal next time you run this command)
        "#,
                    zc_file = auto_complete_zsh.to_str().unwrap()
                );

                fs::write(auto_complete_zsh, include_str!("../../../_task.zsh"))
                    .expect("Unable to write zsh file completions file");

                true
            } else {
                false
            }
        }

        _ => false,
    }
}
