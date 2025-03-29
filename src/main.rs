use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use shellexpand;
use std::path::PathBuf;
use std::process;

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct ColorScheme {
    background: String,
    error: String,
    error_container: String,
    inverse_on_surface: String,
    inverse_primary: String,
    inverse_surface: String,
    on_background: String,
    on_error: String,
    on_error_container: String,
    on_primary: String,
    on_primary_container: String,
    on_primary_fixed: String,
    on_primary_fixed_variant: String,
    on_secondary: String,
    on_secondary_container: String,
    on_secondary_fixed: String,
    on_secondary_fixed_variant: String,
    on_surface: String,
    on_surface_variant: String,
    on_tertiary: String,
    on_tertiary_container: String,
    on_tertiary_fixed: String,
    on_tertiary_fixed_variant: String,
    outline: String,
    outline_variant: String,
    primary: String,
    primary_container: String,
    primary_fixed: String,
    primary_fixed_dim: String,
    scrim: String,
    secondary: String,
    secondary_container: String,
    secondary_fixed: String,
    secondary_fixed_dim: String,
    shadow: String,
    surface: String,
    surface_bright: String,
    surface_container: String,
    surface_container_high: String,
    surface_container_highest: String,
    surface_container_low: String,
    surface_container_lowest: String,
    surface_dim: String,
    surface_tint: String,
    surface_variant: String,
    tertiary: String,
    tertiary_container: String,
    tertiary_fixed: String,
    tertiary_fixed_dim: String,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct Colors {
    dark: ColorScheme,
    light: ColorScheme,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct GeneratedColours {
    colors: Colors,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct WallpaperConfig {
    file_path: String,
    colors: Colors,
}

fn restart_qtile() {
    const QTILE_RESTART_COMMAND: &str = "qtile cmd-obj -o cmd -f reload_config";

    process::Command::new("sh")
        .arg("-c")
        .arg(QTILE_RESTART_COMMAND)
        .spawn()
        .expect("Failed to Restart Qtile!");
}

fn main() {
    static DEFAULT_QTILE_CONFIG_DIRECTORY: &str = "~/.config/qtile";

    let matches = Command::new("qtwallset")
        .version("0.1.0")
        .about("A tool to set Qtile wallpaper for my config")
        .arg(
            Arg::new("output_directory")
                .short('o')
                .long("output-directory")
                .required(false)
                .default_value(DEFAULT_QTILE_CONFIG_DIRECTORY),
        )
        .arg(Arg::new("wallpaper_path").required(true))
        .get_matches();

    let output_directory = matches.get_one::<String>("output_directory").unwrap();
    let mut output_directory = PathBuf::from(shellexpand::tilde(output_directory).to_string());
    output_directory.push("wallpaper");
    println!("Output Directory: {:?}", output_directory);

    let wallpaper_path = matches.get_one::<String>("wallpaper_path").unwrap();
    println!("Wallpaper Path: {:?}", wallpaper_path);

    let generated_colours = process::Command::new("sh")
        .arg("-c")
        .arg(format!("matugen image {:?} --json hex", wallpaper_path))
        .output()
        .expect("Failed to execute command");

    let json_string = String::from_utf8_lossy(&generated_colours.stdout).to_string();

    let parsed_data: GeneratedColours =
        serde_json::from_str(&json_string).expect("Failed to parse JSON");

    let wallpaper_config: WallpaperConfig = WallpaperConfig {
        file_path: wallpaper_path.to_string(),
        colors: parsed_data.colors,
    };

    let wallpaper_config_json = serde_json::to_string_pretty(&wallpaper_config).unwrap();

    println!("Wallpaper Config: {}", wallpaper_config_json);
}
