use serde::Deserialize;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONFIG_FILE_NAME: &str = "settings.toml";

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default)]
    pub discord: DiscordConfig,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct DiscordConfig {
    pub webhook_url: Option<String>,
}

impl Config {
    pub fn from_cli_args<I>(args: I) -> Result<ConfigLoad, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = OsString>,
    {
        let explicit_path = parse_config_path_arg(args)?;
        Self::load(explicit_path.as_deref())
    }

    pub fn load(explicit_path: Option<&Path>) -> Result<ConfigLoad, Box<dyn std::error::Error>> {
        let path = match explicit_path {
            Some(path) => Some(path.to_path_buf()),
            None => find_nearest_config(std::env::current_dir()?, DEFAULT_CONFIG_FILE_NAME),
        };

        let config = match &path {
            Some(path) => Self::from_file(path)?,
            None => Self::default(),
        };

        Ok(ConfigLoad { path, config })
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&raw)?;
        Ok(config)
    }
}

#[derive(Debug, Clone)]
pub struct ConfigLoad {
    pub path: Option<PathBuf>,
    pub config: Config,
}

pub fn find_nearest_config(start_dir: PathBuf, file_name: &str) -> Option<PathBuf> {
    for dir in start_dir.ancestors() {
        let candidate = dir.join(file_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}

fn parse_config_path_arg<I>(args: I) -> Result<Option<PathBuf>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let _program_name = args.next();

    let mut config_path = None;

    while let Some(arg) = args.next() {
        if arg == "--config" {
            let Some(path) = args.next() else {
                return Err("missing path after --config".into());
            };

            config_path = Some(PathBuf::from(path));
            continue;
        }

        return Err(format!("unrecognized argument: {}", PathBuf::from(arg).display()).into());
    }

    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn config_defaults_when_no_file_exists() {
        assert_eq!(Config::default().discord.webhook_url, None);
    }

    #[test]
    fn parses_explicit_config_path() {
        let root = unique_temp_dir();
        let path = root.join(DEFAULT_CONFIG_FILE_NAME);

        fs::create_dir_all(&root).unwrap();
        fs::write(&path, "").unwrap();

        let load = Config::from_cli_args([
            OsString::from("free-games-notifier"),
            OsString::from("--config"),
            path.as_os_str().to_os_string(),
        ])
        .unwrap();

        assert_eq!(load.path, Some(path));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn finds_nearest_config_in_ancestors() {
        let root = unique_temp_dir();
        let parent = root.join("parent");
        let child = parent.join("child");

        fs::create_dir_all(&child).unwrap();
        fs::write(root.join(DEFAULT_CONFIG_FILE_NAME), "ignored = true").unwrap();
        fs::write(parent.join(DEFAULT_CONFIG_FILE_NAME), "ignored = true").unwrap();
        fs::write(child.join(DEFAULT_CONFIG_FILE_NAME), "ignored = true").unwrap();

        let found = find_nearest_config(child.clone(), DEFAULT_CONFIG_FILE_NAME);

        assert_eq!(found, Some(child.join(DEFAULT_CONFIG_FILE_NAME)));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn loads_toml_config_file() {
        let root = unique_temp_dir();
        let path = root.join(DEFAULT_CONFIG_FILE_NAME);

        fs::create_dir_all(&root).unwrap();
        fs::write(
            &path,
            r#"
[discord]
webhook_url = "https://discord.example/webhook"
"#,
        )
        .unwrap();

        let config = Config::from_file(&path).unwrap();

        assert_eq!(
            config.discord.webhook_url,
            Some(String::from("https://discord.example/webhook"))
        );

        fs::remove_dir_all(root).unwrap();
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("free-games-notifier-{nanos}"))
    }
}
