use regex::Regex;

use crate::utils::commands::CLI;

use super::Protontricks;

impl Protontricks {
    /// Installs the following components if they aren't already installed
    pub async fn install_components(app_id: &str, components: &Vec<String>) -> Result<u32, String> {
        let args = [app_id.to_string(), "-q".to_string()];
        let args = args.iter().cloned().chain(components.into_iter().cloned());

        let output = Protontricks::exec(args).await?;

        // example: 'gdiplus already installed, skipping'
        let re = Regex::new(r"(?m)^(?<name>[\w-]+) already installed, skipping$").unwrap();

        let total_tweaks: u32 = components.len().try_into().unwrap();
        let tweaks_already_installed: u32 = re.captures_iter(&output).count().try_into().unwrap();

        Ok(total_tweaks - tweaks_already_installed)
    }
}
