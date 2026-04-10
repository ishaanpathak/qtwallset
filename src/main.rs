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
struct ColorValue {
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ColorVariants {
    dark: ColorValue,
    light: ColorValue,
    default: ColorValue,
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
    fn pick(v: ColorVariants) -> (String, String) {
        (v.dark.color, v.light.color)
    }

    fn into_colors(self) -> Colors {
        let (background_d, background_l) = Self::pick(self.background);
        let (error_d, error_l) = Self::pick(self.error);
        let (error_container_d, error_container_l) = Self::pick(self.error_container);
        let (inverse_on_surface_d, inverse_on_surface_l) = Self::pick(self.inverse_on_surface);
        let (inverse_primary_d, inverse_primary_l) = Self::pick(self.inverse_primary);
        let (inverse_surface_d, inverse_surface_l) = Self::pick(self.inverse_surface);
        let (on_background_d, on_background_l) = Self::pick(self.on_background);
        let (on_error_d, on_error_l) = Self::pick(self.on_error);
        let (on_error_container_d, on_error_container_l) = Self::pick(self.on_error_container);
        let (on_primary_d, on_primary_l) = Self::pick(self.on_primary);
        let (on_primary_container_d, on_primary_container_l) =
            Self::pick(self.on_primary_container);
        let (on_primary_fixed_d, on_primary_fixed_l) = Self::pick(self.on_primary_fixed);
        let (on_primary_fixed_variant_d, on_primary_fixed_variant_l) =
            Self::pick(self.on_primary_fixed_variant);
        let (on_secondary_d, on_secondary_l) = Self::pick(self.on_secondary);
        let (on_secondary_container_d, on_secondary_container_l) =
            Self::pick(self.on_secondary_container);
        let (on_secondary_fixed_d, on_secondary_fixed_l) = Self::pick(self.on_secondary_fixed);
        let (on_secondary_fixed_variant_d, on_secondary_fixed_variant_l) =
            Self::pick(self.on_secondary_fixed_variant);
        let (on_surface_d, on_surface_l) = Self::pick(self.on_surface);
        let (on_surface_variant_d, on_surface_variant_l) = Self::pick(self.on_surface_variant);
        let (on_tertiary_d, on_tertiary_l) = Self::pick(self.on_tertiary);
        let (on_tertiary_container_d, on_tertiary_container_l) =
            Self::pick(self.on_tertiary_container);
        let (on_tertiary_fixed_d, on_tertiary_fixed_l) = Self::pick(self.on_tertiary_fixed);
        let (on_tertiary_fixed_variant_d, on_tertiary_fixed_variant_l) =
            Self::pick(self.on_tertiary_fixed_variant);
        let (outline_d, outline_l) = Self::pick(self.outline);
        let (outline_variant_d, outline_variant_l) = Self::pick(self.outline_variant);
        let (primary_d, primary_l) = Self::pick(self.primary);
        let (primary_container_d, primary_container_l) = Self::pick(self.primary_container);
        let (primary_fixed_d, primary_fixed_l) = Self::pick(self.primary_fixed);
        let (primary_fixed_dim_d, primary_fixed_dim_l) = Self::pick(self.primary_fixed_dim);
        let (scrim_d, scrim_l) = Self::pick(self.scrim);
        let (secondary_d, secondary_l) = Self::pick(self.secondary);
        let (secondary_container_d, secondary_container_l) = Self::pick(self.secondary_container);
        let (secondary_fixed_d, secondary_fixed_l) = Self::pick(self.secondary_fixed);
        let (secondary_fixed_dim_d, secondary_fixed_dim_l) = Self::pick(self.secondary_fixed_dim);
        let (shadow_d, shadow_l) = Self::pick(self.shadow);
        let (surface_d, surface_l) = Self::pick(self.surface);
        let (surface_bright_d, surface_bright_l) = Self::pick(self.surface_bright);
        let (surface_container_d, surface_container_l) = Self::pick(self.surface_container);
        let (surface_container_high_d, surface_container_high_l) =
            Self::pick(self.surface_container_high);
        let (surface_container_highest_d, surface_container_highest_l) =
            Self::pick(self.surface_container_highest);
        let (surface_container_low_d, surface_container_low_l) =
            Self::pick(self.surface_container_low);
        let (surface_container_lowest_d, surface_container_lowest_l) =
            Self::pick(self.surface_container_lowest);
        let (surface_dim_d, surface_dim_l) = Self::pick(self.surface_dim);
        let (surface_tint_d, surface_tint_l) = Self::pick(self.surface_tint);
        let (surface_variant_d, surface_variant_l) = Self::pick(self.surface_variant);
        let (tertiary_d, tertiary_l) = Self::pick(self.tertiary);
        let (tertiary_container_d, tertiary_container_l) = Self::pick(self.tertiary_container);
        let (tertiary_fixed_d, tertiary_fixed_l) = Self::pick(self.tertiary_fixed);
        let (tertiary_fixed_dim_d, tertiary_fixed_dim_l) = Self::pick(self.tertiary_fixed_dim);

        Colors {
            dark: ColorScheme {
                background: background_d,
                error: error_d,
                error_container: error_container_d,
                inverse_on_surface: inverse_on_surface_d,
                inverse_primary: inverse_primary_d,
                inverse_surface: inverse_surface_d,
                on_background: on_background_d,
                on_error: on_error_d,
                on_error_container: on_error_container_d,
                on_primary: on_primary_d,
                on_primary_container: on_primary_container_d,
                on_primary_fixed: on_primary_fixed_d,
                on_primary_fixed_variant: on_primary_fixed_variant_d,
                on_secondary: on_secondary_d,
                on_secondary_container: on_secondary_container_d,
                on_secondary_fixed: on_secondary_fixed_d,
                on_secondary_fixed_variant: on_secondary_fixed_variant_d,
                on_surface: on_surface_d,
                on_surface_variant: on_surface_variant_d,
                on_tertiary: on_tertiary_d,
                on_tertiary_container: on_tertiary_container_d,
                on_tertiary_fixed: on_tertiary_fixed_d,
                on_tertiary_fixed_variant: on_tertiary_fixed_variant_d,
                outline: outline_d,
                outline_variant: outline_variant_d,
                primary: primary_d,
                primary_container: primary_container_d,
                primary_fixed: primary_fixed_d,
                primary_fixed_dim: primary_fixed_dim_d,
                scrim: scrim_d,
                secondary: secondary_d,
                secondary_container: secondary_container_d,
                secondary_fixed: secondary_fixed_d,
                secondary_fixed_dim: secondary_fixed_dim_d,
                shadow: shadow_d,
                surface: surface_d,
                surface_bright: surface_bright_d,
                surface_container: surface_container_d,
                surface_container_high: surface_container_high_d,
                surface_container_highest: surface_container_highest_d,
                surface_container_low: surface_container_low_d,
                surface_container_lowest: surface_container_lowest_d,
                surface_dim: surface_dim_d,
                surface_tint: surface_tint_d,
                surface_variant: surface_variant_d,
                tertiary: tertiary_d,
                tertiary_container: tertiary_container_d,
                tertiary_fixed: tertiary_fixed_d,
                tertiary_fixed_dim: tertiary_fixed_dim_d,
            },
            light: ColorScheme {
                background: background_l,
                error: error_l,
                error_container: error_container_l,
                inverse_on_surface: inverse_on_surface_l,
                inverse_primary: inverse_primary_l,
                inverse_surface: inverse_surface_l,
                on_background: on_background_l,
                on_error: on_error_l,
                on_error_container: on_error_container_l,
                on_primary: on_primary_l,
                on_primary_container: on_primary_container_l,
                on_primary_fixed: on_primary_fixed_l,
                on_primary_fixed_variant: on_primary_fixed_variant_l,
                on_secondary: on_secondary_l,
                on_secondary_container: on_secondary_container_l,
                on_secondary_fixed: on_secondary_fixed_l,
                on_secondary_fixed_variant: on_secondary_fixed_variant_l,
                on_surface: on_surface_l,
                on_surface_variant: on_surface_variant_l,
                on_tertiary: on_tertiary_l,
                on_tertiary_container: on_tertiary_container_l,
                on_tertiary_fixed: on_tertiary_fixed_l,
                on_tertiary_fixed_variant: on_tertiary_fixed_variant_l,
                outline: outline_l,
                outline_variant: outline_variant_l,
                primary: primary_l,
                primary_container: primary_container_l,
                primary_fixed: primary_fixed_l,
                primary_fixed_dim: primary_fixed_dim_l,
                scrim: scrim_l,
                secondary: secondary_l,
                secondary_container: secondary_container_l,
                secondary_fixed: secondary_fixed_l,
                secondary_fixed_dim: secondary_fixed_dim_l,
                shadow: shadow_l,
                surface: surface_l,
                surface_bright: surface_bright_l,
                surface_container: surface_container_l,
                surface_container_high: surface_container_high_l,
                surface_container_highest: surface_container_highest_l,
                surface_container_low: surface_container_low_l,
                surface_container_lowest: surface_container_lowest_l,
                surface_dim: surface_dim_l,
                surface_tint: surface_tint_l,
                surface_variant: surface_variant_l,
                tertiary: tertiary_l,
                tertiary_container: tertiary_container_l,
                tertiary_fixed: tertiary_fixed_l,
                tertiary_fixed_dim: tertiary_fixed_dim_l,
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
        .arg("--source-color-index")
        .arg("0")
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
