use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

use libcitadel::Result;

#[derive(Deserialize)]
pub struct BuildConfig {
    #[serde(rename = "image-type")]
    image_type: String,
    channel: String,
    version: usize,
    timestamp: String,
    source: String,
    #[serde(default)]
    compress: bool,
    #[serde(rename = "kernel-version")]
    kernel_version: Option<String>,
    #[serde(rename = "kernel-id")]
    kernel_id: Option<String>,

    #[serde(rename = "realmfs-name")]
    realmfs_name: Option<String>,

    #[serde(skip)]
    basedir: PathBuf,
    #[serde(skip)]
    src_path: PathBuf,
    #[serde(skip)]
    img_name: String,
}

impl BuildConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<BuildConfig> {
        let mut path = path.as_ref().to_owned();
        if path.is_dir() {
            path.push("mkimage.conf");
        }

        let mut config = match BuildConfig::from_path(&path) {
            Ok(config) => config,
            Err(e) => bail!("Failed to load config file {}: {}", path.display(), e),
        };

        path.pop();
        config.basedir = path;
        config.src_path = PathBuf::from(&config.source);
        config.img_name = match config.kernel_version {
            Some(ref version) => format!("{}-{}", &config.image_type, version),
            None => config.image_type.to_owned(),
        };
        Ok(config)
    }

    fn from_path(path: &Path) -> Result<BuildConfig> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let config = toml::from_str::<BuildConfig>(&s)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        let itype = self.image_type.as_str();
        if itype != "extra" && itype != "rootfs" && itype != "kernel" && itype != "realmfs" {
            bail!("Invalid image type '{}'", self.image_type);
        };
        let src = Path::new(&self.source);
        if !src.is_file() {
            bail!(
                "Source path '{}' does not exist or is not a regular file",
                src.display()
            );
        }
        if self.image_type == "kernel" && self.kernel_version.is_none() {
            bail!("Cannot build 'kernel' image without kernel-version field");
        }

        Ok(())
    }

    pub fn timestamp(&self) -> &str { &self.timestamp }

    pub fn source(&self) -> &Path {
        &self.src_path
    }

    pub fn workdir_path<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.basedir.join(filename.as_ref())
    }

    pub fn img_name(&self) -> &str {
        &self.img_name
    }

    pub fn kernel_version(&self) -> Option<&str> {
        self.kernel_version.as_ref().map(|s| s.as_str())
    }

    pub fn kernel_id(&self) -> Option<&str> {
        self.kernel_id.as_ref().map(|s| s.as_str())
    }

    pub fn realmfs_name(&self) -> Option<&str> {
        self.realmfs_name.as_ref().map(|s| s.as_str())
    }

    pub fn version(&self) -> usize {
        self.version
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn image_type(&self) -> &str {
        &self.image_type
    }

    pub fn compress(&self) -> bool {
        self.compress
    }
}
