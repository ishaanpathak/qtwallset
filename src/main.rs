use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use shellexpand;
use std::fs::{File, copy, read_dir, remove_file};
use std::io::Write;
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

fn get_colors_from_image(image_path: &PathBuf) -> Colors {
    let image_path = image_path.to_str().unwrap();
    let matugen_output = process::Command::new("sh")
        .arg("-c")
        .arg(format!("matugen image {:?} --json hex", image_path))
        .output()
        .expect("Failed to execute command");

    let matugen_output_stdout = String::from_utf8_lossy(&matugen_output.stdout).to_string();

    let generated_colours: GeneratedColours =
        serde_json::from_str(&matugen_output_stdout).expect("Failed to parse JSON");

    generated_colours.colors
}

fn clear_wallpaper_directory(active_wallpaper_directory: &PathBuf) {
    if active_wallpaper_directory.is_dir() {
        for entry in read_dir(active_wallpaper_directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                remove_file(path).expect("Failed to clear Wallpaper Directory!");
            }
        }
    }
}

fn write_wallpaper_config(config_directory: PathBuf, wallpaper_config: WallpaperConfig) {
    let mut config_file_path = config_directory;
    config_file_path.push("wallpaper_info.json");

    let mut file = match File::open(&config_file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Config file not found, creating it...");
            File::create(&config_file_path).expect("Failed to create config file.")
        }
    };

    let config_json = serde_json::to_string_pretty(&wallpaper_config).unwrap();

    file.write_all(&config_json.as_bytes())
        .expect("Failed to write to wallpaper config!");
}

fn copy_wallpaper(wallpaper_directory: PathBuf, wallpaper_path: PathBuf) {
    let wallpaper_file_name = wallpaper_path.file_name().unwrap();
    let mut active_wallpaper_path = wallpaper_directory;
    active_wallpaper_path.push(wallpaper_file_name);
    copy(wallpaper_path, active_wallpaper_path)
        .expect("Failed to copy wallpaper to Qtile Directory");
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

    let mut active_wallpaper_directory =
        PathBuf::from(shellexpand::tilde(output_directory).to_string());
    active_wallpaper_directory.push("wallpaper");
    active_wallpaper_directory.push("active");

    let mut wallpaper_info_directory =
        PathBuf::from(shellexpand::tilde(output_directory).to_string());
    wallpaper_info_directory.push("cache");

    let wallpaper_path = matches.get_one::<String>("wallpaper_path").unwrap();
    let wallpaper_path = PathBuf::from(wallpaper_path);

    let generated_colours = get_colors_from_image(&wallpaper_path);

    let mut new_wallpaper_path = active_wallpaper_directory.to_path_buf();
    new_wallpaper_path.push(&wallpaper_path.file_name().unwrap());

    let wallpaper_config: WallpaperConfig = WallpaperConfig {
        file_path: new_wallpaper_path.to_string_lossy().to_string(),
        colors: generated_colours,
    };

    clear_wallpaper_directory(&active_wallpaper_directory);

    copy_wallpaper(active_wallpaper_directory, wallpaper_path);

    write_wallpaper_config(wallpaper_info_directory, wallpaper_config);

    restart_qtile();
}
