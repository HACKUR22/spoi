use std::process::exit;

use tauri_plugin_cli::CliExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
                // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
                Ok(matches) => {
                    println!("args: {:?}", matches.args);
                    println!("subcommand: {:?}", matches.subcommand);
                    matches.subcommand.map(|subcommand| {
                        println!("subcommand: {}", subcommand.name);
                        subcommand.matches.args.iter().for_each(|(k, v)| {
                            println!("{}: {:?}", k, v);
                        });
                    });
                    matches.args.iter().for_each(|(k, v)| {
                        println!("{}: {:?}", k, v);
                    });
                }
                Err(_) => {}
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
