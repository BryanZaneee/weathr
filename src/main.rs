mod config;
mod display;

use config::Config;
use display::AsciiDisplay;

fn main() {
    match Config::load() {
        Ok(config) => {
            println!(
                "Location: lat={}, lon={}",
                config.location.latitude, config.location.longitude
            );
            AsciiDisplay::render_house();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nPlease create a config file at:");
            eprintln!("  $XDG_CONFIG_HOME/weathr/config.toml");
            eprintln!("  or ~/.config/weathr/config.toml");
            eprintln!("\nExample config.toml:");
            eprintln!("  [location]");
            eprintln!("  latitude = 0.7893");
            eprintln!("  longitude = 113.9213");
            std::process::exit(1);
        }
    }
}
