use clap::{Arg, Command};
use log::info;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::result::Result;
use std::{
    fs::{copy, read_dir, remove_file, write},
    path::{Path, PathBuf},
    process,
};

#[derive(Serialize, Deserialize, Debug)]
struct ColorVariants {
    dark: String,
    light: String,
    default: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatugenColors {
    background: ColorVariants,
    error: ColorVariants,
    error_container: ColorVariants,
    inverse_on_surface: ColorVariants,
    inverse_primary: ColorVariants,
    inverse_surface: ColorVariants,
    on_background: ColorVariants,
    on_error: ColorVariants,
    on_error_container: ColorVariants,
    on_primary: ColorVariants,
    on_primary_container: ColorVariants,
    on_primary_fixed: ColorVariants,
    on_primary_fixed_variant: ColorVariants,
    on_secondary: ColorVariants,
    on_secondary_container: ColorVariants,
    on_secondary_fixed: ColorVariants,
    on_secondary_fixed_variant: ColorVariants,
    on_surface: ColorVariants,
    on_surface_variant: ColorVariants,
    on_tertiary: ColorVariants,
    on_tertiary_container: ColorVariants,
    on_tertiary_fixed: ColorVariants,
    on_tertiary_fixed_variant: ColorVariants,
    outline: ColorVariants,
    outline_variant: ColorVariants,
    primary: ColorVariants,
    primary_container: ColorVariants,
    primary_fixed: ColorVariants,
    primary_fixed_dim: ColorVariants,
    scrim: ColorVariants,
    secondary: ColorVariants,
    secondary_container: ColorVariants,
    secondary_fixed: ColorVariants,
    secondary_fixed_dim: ColorVariants,
    shadow: ColorVariants,
    surface: ColorVariants,
    surface_bright: ColorVariants,
    surface_container: ColorVariants,
    surface_container_high: ColorVariants,
    surface_container_highest: ColorVariants,
    surface_container_low: ColorVariants,
    surface_container_lowest: ColorVariants,
    surface_dim: ColorVariants,
    surface_tint: ColorVariants,
    surface_variant: ColorVariants,
    tertiary: ColorVariants,
    tertiary_container: ColorVariants,
    tertiary_fixed: ColorVariants,
    tertiary_fixed_dim: ColorVariants,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatugenOutput {
    colors: MatugenColors,
}

impl MatugenColors {
    fn into_colors(self) -> Colors {
        Colors {
            dark: ColorScheme {
                background: self.background.dark,
                error: self.error.dark,
                error_container: self.error_container.dark,
                inverse_on_surface: self.inverse_on_surface.dark,
                inverse_primary: self.inverse_primary.dark,
                inverse_surface: self.inverse_surface.dark,
                on_background: self.on_background.dark,
                on_error: self.on_error.dark,
                on_error_container: self.on_error_container.dark,
                on_primary: self.on_primary.dark,
                on_primary_container: self.on_primary_container.dark,
                on_primary_fixed: self.on_primary_fixed.dark,
                on_primary_fixed_variant: self.on_primary_fixed_variant.dark,
                on_secondary: self.on_secondary.dark,
                on_secondary_container: self.on_secondary_container.dark,
                on_secondary_fixed: self.on_secondary_fixed.dark,
                on_secondary_fixed_variant: self.on_secondary_fixed_variant.dark,
                on_surface: self.on_surface.dark,
                on_surface_variant: self.on_surface_variant.dark,
                on_tertiary: self.on_tertiary.dark,
                on_tertiary_container: self.on_tertiary_container.dark,
                on_tertiary_fixed: self.on_tertiary_fixed.dark,
                on_tertiary_fixed_variant: self.on_tertiary_fixed_variant.dark,
                outline: self.outline.dark,
                outline_variant: self.outline_variant.dark,
                primary: self.primary.dark,
                primary_container: self.primary_container.dark,
                primary_fixed: self.primary_fixed.dark,
                primary_fixed_dim: self.primary_fixed_dim.dark,
                scrim: self.scrim.dark,
                secondary: self.secondary.dark,
                secondary_container: self.secondary_container.dark,
                secondary_fixed: self.secondary_fixed.dark,
                secondary_fixed_dim: self.secondary_fixed_dim.dark,
                shadow: self.shadow.dark,
                surface: self.surface.dark,
                surface_bright: self.surface_bright.dark,
                surface_container: self.surface_container.dark,
                surface_container_high: self.surface_container_high.dark,
                surface_container_highest: self.surface_container_highest.dark,
                surface_container_low: self.surface_container_low.dark,
                surface_container_lowest: self.surface_container_lowest.dark,
                surface_dim: self.surface_dim.dark,
                surface_tint: self.surface_tint.dark,
                surface_variant: self.surface_variant.dark,
                tertiary: self.tertiary.dark,
                tertiary_container: self.tertiary_container.dark,
                tertiary_fixed: self.tertiary_fixed.dark,
                tertiary_fixed_dim: self.tertiary_fixed_dim.dark,
            },
            light: ColorScheme {
                background: self.background.light,
                error: self.error.light,
                error_container: self.error_container.light,
                inverse_on_surface: self.inverse_on_surface.light,
                inverse_primary: self.inverse_primary.light,
                inverse_surface: self.inverse_surface.light,
                on_background: self.on_background.light,
                on_error: self.on_error.light,
                on_error_container: self.on_error_container.light,
                on_primary: self.on_primary.light,
                on_primary_container: self.on_primary_container.light,
                on_primary_fixed: self.on_primary_fixed.light,
                on_primary_fixed_variant: self.on_primary_fixed_variant.light,
                on_secondary: self.on_secondary.light,
                on_secondary_container: self.on_secondary_container.light,
                on_secondary_fixed: self.on_secondary_fixed.light,
                on_secondary_fixed_variant: self.on_secondary_fixed_variant.light,
                on_surface: self.on_surface.light,
                on_surface_variant: self.on_surface_variant.light,
                on_tertiary: self.on_tertiary.light,
                on_tertiary_container: self.on_tertiary_container.light,
                on_tertiary_fixed: self.on_tertiary_fixed.light,
                on_tertiary_fixed_variant: self.on_tertiary_fixed_variant.light,
                outline: self.outline.light,
                outline_variant: self.outline_variant.light,
                primary: self.primary.light,
                primary_container: self.primary_container.light,
                primary_fixed: self.primary_fixed.light,
                primary_fixed_dim: self.primary_fixed_dim.light,
                scrim: self.scrim.light,
                secondary: self.secondary.light,
                secondary_container: self.secondary_container.light,
                secondary_fixed: self.secondary_fixed.light,
                secondary_fixed_dim: self.secondary_fixed_dim.light,
                shadow: self.shadow.light,
                surface: self.surface.light,
                surface_bright: self.surface_bright.light,
                surface_container: self.surface_container.light,
                surface_container_high: self.surface_container_high.light,
                surface_container_highest: self.surface_container_highest.light,
                surface_container_low: self.surface_container_low.light,
                surface_container_lowest: self.surface_container_lowest.light,
                surface_dim: self.surface_dim.light,
                surface_tint: self.surface_tint.light,
                surface_variant: self.surface_variant.light,
                tertiary: self.tertiary.light,
                tertiary_container: self.tertiary_container.light,
                tertiary_fixed: self.tertiary_fixed.light,
                tertiary_fixed_dim: self.tertiary_fixed_dim.light,
            },
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
struct Colors {
    dark: ColorScheme,
    light: ColorScheme,
}

#[derive(Serialize, Deserialize, Debug)]
struct WallpaperConfig {
    file_path: String,
    colors: Colors,
}

// Function to check if a command is available on the system
fn command_exists(command: &str) -> bool {
    process::Command::new("which")
        .arg(command)
        .output()
        .map(|output| !output.stdout.is_empty())
        .unwrap_or(false)
}

// Function to restart Qtile (assuming Qtile is installed)
fn restart_qtile() -> Result<(), String> {
    if !command_exists("qtile") {
        return Err("Qtile is not installed. Please install Qtile.".to_string());
    }

    const QTILE_RESTART_COMMAND: &str = "qtile cmd-obj -o cmd -f reload_config";
    process::Command::new("sh")
        .arg("-c")
        .arg(QTILE_RESTART_COMMAND)
        .spawn()
        .map_err(|e| format!("Failed to restart Qtile: {}", e))?;
    info!("Qtile restarted successfully.");
    Ok(())
}

// Function to check if matugen is installed
fn check_matugen_installed() -> Result<(), String> {
    if !command_exists("matugen") {
        return Err("matugen is not installed. Please install matugen.".to_string());
    }
    Ok(())
}

fn get_colors_from_image(image_path: &Path) -> Result<Colors, String> {
    let output = process::Command::new("matugen")
        .arg("image")
        .arg(image_path)
        .arg("--json")
        .arg("hex")
        .output()
        .map_err(|e| format!("Failed to execute matugen command: {}", e))?;

    let matugen_output = String::from_utf8_lossy(&output.stdout).to_string();
    let matugen: MatugenOutput = serde_json::from_str(&matugen_output)
        .map_err(|e| format!("Failed to parse matugen output: {}", e))?;

    Ok(matugen.colors.into_colors())
}

fn clear_wallpaper_directory(active_wallpaper_directory: &Path) -> Result<(), String> {
    if let Ok(entries) = read_dir(active_wallpaper_directory) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                remove_file(&path)
                    .map_err(|e| format!("Failed to remove file {:?}: {}", path, e))?;
            }
        }
    }
    info!("Wallpaper directory cleared.");
    Ok(())
}

fn write_wallpaper_config(
    config_directory: &Path,
    wallpaper_config: WallpaperConfig,
) -> Result<(), String> {
    let config_file_path = config_directory.join("wallpaper_info.json");
    let config_json = serde_json::to_string_pretty(&wallpaper_config)
        .map_err(|e| format!("Failed to serialize wallpaper config: {}", e))?;

    write(&config_file_path, config_json)
        .map_err(|e| format!("Failed to write to config file: {}", e))?;
    info!("Wallpaper config written to {:?}", config_file_path);
    Ok(())
}

fn copy_wallpaper(wallpaper_directory: &Path, wallpaper_path: &Path) -> Result<(), String> {
    let wallpaper_file_name = wallpaper_path
        .file_name()
        .ok_or_else(|| "Wallpaper file name is missing.".to_string())?;
    let destination = wallpaper_directory.join(wallpaper_file_name);
    copy(wallpaper_path, &destination)
        .map_err(|e| format!("Failed to copy wallpaper to {:?}: {}", destination, e))?;
    info!("Wallpaper copied to {:?}", destination);
    Ok(())
}

fn resolve_path(directory: &str, sub_path: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(directory).to_string()).join(sub_path)
}

fn main() -> Result<(), String> {
    // Initialize logging
    env_logger::init();

    const DEFAULT_QTILE_CONFIG_DIRECTORY: &str = "~/.config/qtile";

    let matches = Command::new("qtwallset")
        .version("0.1.0")
        .about("A tool to set Qtile wallpaper for my config")
        .arg(
            Arg::new("output_directory")
                .short('o')
                .long("output-directory")
                .default_value(DEFAULT_QTILE_CONFIG_DIRECTORY),
        )
        .arg(
            Arg::new("no_reload")
                .long("no-reload")
                .action(clap::ArgAction::SetTrue)
                .help("Do not reload Qtile")
                .required(false),
        )
        .arg(Arg::new("wallpaper_path").required(true))
        .get_matches();

    let output_directory = matches.get_one::<String>("output_directory").unwrap();
    let wallpaper_path = Path::new(matches.get_one::<String>("wallpaper_path").unwrap());

    // Step 1: Check for dependencies
    check_matugen_installed()?;

    // Step 2: Generate colors from wallpaper image
    let generated_colours = get_colors_from_image(wallpaper_path)?;

    // Step 3: Set up wallpaper config
    let active_wallpaper_directory = resolve_path(output_directory, "wallpaper/active");
    let wallpaper_info_directory = resolve_path(output_directory, "cache");

    let new_wallpaper_path = active_wallpaper_directory.join(wallpaper_path.file_name().unwrap());
    let wallpaper_config = WallpaperConfig {
        file_path: new_wallpaper_path.to_string_lossy().to_string(),
        colors: generated_colours,
    };

    // Step 4: Clear old wallpaper, copy new one, and write config
    clear_wallpaper_directory(&active_wallpaper_directory)?;
    copy_wallpaper(&active_wallpaper_directory, wallpaper_path)?;
    write_wallpaper_config(&wallpaper_info_directory, wallpaper_config)?;

    let do_not_reload = matches.get_flag("no_reload");
    // Step 5: Restart Qtile
    if !do_not_reload {
        restart_qtile()?;
    }

    Ok(())
}
