use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct Paths {
    pub data: PathBuf,
    pub config: PathBuf,
    pub cache: PathBuf,
    pub state: PathBuf,
}

pub fn resolve(app: &AppHandle) -> Result<Paths, String> {
    let path = app.path();

    let root = path
        .app_local_data_dir()
        .map_err(|e| format!("failed to resolve app local data dir: {e}"))?;
    let data = root.join("opencode-dev-data");
    let state = root.join("opencode-dev-state");
    let config = path
        .app_config_dir()
        .map_err(|e| format!("failed to resolve app config dir: {e}"))?
        .join("opencode-dev-config");
    let cache = path
        .app_cache_dir()
        .map_err(|e| format!("failed to resolve app cache dir: {e}"))?
        .join("opencode-dev-cache");

    Ok(Paths {
        data,
        config,
        cache,
        state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_struct_has_expected_fields() {
        let paths = Paths {
            data: PathBuf::from("/tmp/data"),
            config: PathBuf::from("/tmp/config"),
            cache: PathBuf::from("/tmp/cache"),
            state: PathBuf::from("/tmp/state"),
        };
        assert_eq!(paths.data, PathBuf::from("/tmp/data"));
        assert_eq!(paths.config, PathBuf::from("/tmp/config"));
        assert_eq!(paths.cache, PathBuf::from("/tmp/cache"));
        assert_eq!(paths.state, PathBuf::from("/tmp/state"));
    }
}
