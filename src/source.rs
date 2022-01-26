use std::fs;
use std::path;

use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::format::FlatpakManifestFormat;

pub const ARCHIVE: &str = "archive";
pub const GIT: &str = "git";
pub const BAZAAR: &str = "bzr";
pub const SVN: &str = "svn";
pub const DIR: &str = "dir";
pub const FILE: &str = "file";
pub const SCRIPT: &str = "script";
pub const SHELL: &str = "shell";
pub const PATCH: &str = "patch";
pub const EXTRA_DATA: &str = "extra-data";

lazy_static! {
    pub static ref SOURCE_TYPES: Vec<String> = vec![
        ARCHIVE.to_string(),
        GIT.to_string(),
        BAZAAR.to_string(),
        SVN.to_string(),
        DIR.to_string(),
        FILE.to_string(),
        SCRIPT.to_string(),
        SHELL.to_string(),
        PATCH.to_string(),
        EXTRA_DATA.to_string(),
    ];
    pub static ref CODE_TYPES: Vec<String> = vec![
        ARCHIVE.to_string(),
        GIT.to_string(),
        BAZAAR.to_string(),
        SVN.to_string(),
        DIR.to_string(),
    ];
    pub static ref VCS_TYPES: Vec<String> = vec![GIT.to_string(), BAZAAR.to_string(), SVN.to_string(),];
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Hash)]
pub enum FlatpakSourceType {
    Archive,
    Git,
    Bazaar,
    Svn,
    Dir,
    File,
    Script,
    Shell,
    Patch,
    ExtraData,
}
impl Default for FlatpakSourceType {
    fn default() -> Self {
        FlatpakSourceType::Archive
    }
}
impl FlatpakSourceType {
    pub fn to_string(&self) -> String {
        match &self {
            FlatpakSourceType::Archive => ARCHIVE.to_string(),
            FlatpakSourceType::Git => GIT.to_string(),
            FlatpakSourceType::Bazaar => BAZAAR.to_string(),
            FlatpakSourceType::Svn => SVN.to_string(),
            FlatpakSourceType::Dir => DIR.to_string(),
            FlatpakSourceType::File => FILE.to_string(),
            FlatpakSourceType::Script => SCRIPT.to_string(),
            FlatpakSourceType::Shell => SHELL.to_string(),
            FlatpakSourceType::Patch => PATCH.to_string(),
            FlatpakSourceType::ExtraData => EXTRA_DATA.to_string(),
        }
    }
    pub fn from_string(source_type: &str) -> Result<FlatpakSourceType, String> {
        if source_type == ARCHIVE {
            return Ok(FlatpakSourceType::Archive);
        }
        if source_type == GIT {
            return Ok(FlatpakSourceType::Git);
        }
        if source_type == BAZAAR {
            return Ok(FlatpakSourceType::Bazaar);
        }
        if source_type == SVN {
            return Ok(FlatpakSourceType::Svn);
        }
        if source_type == DIR {
            return Ok(FlatpakSourceType::Dir);
        }
        if source_type == FILE {
            return Ok(FlatpakSourceType::File);
        }
        if source_type == SCRIPT {
            return Ok(FlatpakSourceType::Script);
        }
        if source_type == SHELL {
            return Ok(FlatpakSourceType::Shell);
        }
        if source_type == PATCH {
            return Ok(FlatpakSourceType::Patch);
        }
        if source_type == EXTRA_DATA {
            return Ok(FlatpakSourceType::ExtraData);
        }
        Err(format!("Invalid source type {}.", source_type))
    }
}

pub fn serialize_to_string<S>(x: &FlatpakSourceType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_string())
}

pub fn deserialize_from_string<'de, D>(deserializer: D) -> Result<FlatpakSourceType, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    FlatpakSourceType::from_string(&buf).map_err(serde::de::Error::custom)
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
/// The sources are a list pointer to the source code that needs to be extracted into
/// the build directory before the build starts.
/// They can be of several types, distinguished by the type property.
///
/// Additionally, the sources list can contain a plain string, which is interpreted as the name
/// of a separate json or yaml file that is read and inserted at this
/// point. The file can contain a single source, or an array of sources.
pub enum FlatpakSourceItem {
    Path(String),
    Description(FlatpakSource),
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
/// These contain a pointer to the source that will be extracted into the
/// source directory before the build starts. They can be of several types,
/// distinguished by the type property.
pub struct FlatpakSource {
    /// Defines the type of the source description.
    /// It is not explicit in the flatpak-manifest man page,
    /// but we only found 1 source in all our dataset with an empty
    /// type, so we assume that the field is actually required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// An array of shell commands.
    /// types: script, shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,

    /// Filename to use inside the source dir.
    /// types: script, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_filename: Option<String>,

    /// The name to use for the downloaded extra data
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The url to the resource.
    /// types: extra-data, svn, bzr, git, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// A list of alternative urls that are used if the main url fails.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_urls: Option<Vec<String>>,

    /// The md5 checksum of the file, verified after download
    /// Note that md5 is no longer considered a safe checksum, we recommend you use at least sha256.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,

    /// The sha1 checksum of the file, verified after download
    /// Note that sha1 is no longer considered a safe checksum, we recommend you use at least sha256.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,

    /// The sha256 of the resource.
    /// types: extra-data, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,

    /// The sha512 checksum of the file, verified after download
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,

    /// The size of the extra data in bytes.
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Whether to initialise the repository as a git repository.
    /// types: archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_init: Option<bool>,

    /// The extra installed size this adds to the app (optional).
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_size: Option<i64>,

    /// A specific revision number to use
    /// types: svn, bzr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,

    /// The branch to use from the git repository
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// The type of archive if it cannot be guessed from the path.
    /// Possible values are:
    ///   * "rpm",
    ///   * "tar",
    ///   * "tar-gzip",
    ///   * "tar-compress",
    ///   * "tar-bzip2",
    ///   * "tar-lzip",
    ///   * "tar-lzma",
    ///   * "tar-lzop",
    ///   * "tar-xz",
    ///   * "zip",
    ///   * "7z",
    /// types: archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_type: Option<String>,

    /// The commit to use from the git repository.
    /// If branch is also specified, then it is verified that the branch/tag is at this specific commit.
    /// This is a readable way to document that you're using a particular tag,
    /// but verify that it does not change.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// The tag to use from the git repository
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// The path to associated with the resource.
    /// types: git, archive, dir, patch, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// An list of paths to a patch files that will be applied in the source dir, in order
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Whether to use "git apply" rather than "patch" to apply the patch, required when the
    /// patch file contains binary diffs.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_git: Option<bool>,

    /// Whether to use "git am" rather than "patch" to apply the patch, required when the patch
    /// file contains binary diffs.
    /// You cannot use this at the same time as use-git.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_git_am: Option<bool>,

    /// Extra options to pass to the patch command.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// Don't use transfer.fsckObjects=1 to mirror git repository. This may be needed for some
    /// (broken) repositories.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_fsckobjects: Option<bool>,

    /// Don't optimize by making a shallow clone when downloading the git repo.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_shallow_clone: Option<bool>,

    /// Don't checkout the git submodules when cloning the repository.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_submodules: Option<bool>,

    /// The number of initial pathname components to strip.
    /// defaults to 1.
    /// types: archive, patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_components: Option<i64>,

    /// Source files to ignore in the directory.
    /// types: dir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<String>>,

    /// If non-empty, only build the module on the arches listed.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_arches: Option<Vec<String>>,

    /// Don't build on any of the arches listed.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_arches: Option<Vec<String>>,

    /// Directory inside the source dir where this source will be extracted.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_checker_data: Option<FlatpakDataCheckerConfig>,
}
impl FlatpakSource {
    /// Get the type for the Flatpak source.
    pub fn get_type(&self) -> Option<String> {
        if let Some(t) = &self.r#type {
            return Some(t.clone());
        }
        None
    }

    pub fn file_path_matches(path: &str) -> bool {
        // The file path for a module is not necessarily in reverse DNS, so we can only test
        // for the extension of the file.
        crate::filename::extension_is_valid(path)
    }

    pub fn load_from_file(path: String) -> Result<Vec<FlatpakSource>, String> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            return Err(format!("{} is not a file.", path));
        }

        let manifest_format = match FlatpakManifestFormat::from_path(&path) {
            Some(f) => f,
            None => return Err(format!("{} is not a Flatpak source manifest.", path)),
        };

        let manifest_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!(
                    "Could not read file {}: {}!",
                    file_path.to_str().unwrap(),
                    e
                ))
            }
        };

        // A standalone source manifest can contain a single source, or an array
        // of sources!!
        if let Ok(source) = FlatpakSource::parse(manifest_format.clone(), &manifest_content) {
            return Ok(vec![source]);
        }
        if let Ok(sources) = FlatpakSource::parse_many(manifest_format, &manifest_content) {
            return Ok(sources);
        }

        return Err(format!("Failed to parse Flatpak source manifest at {}.", path));
    }

    pub fn parse(format: FlatpakManifestFormat, manifest_content: &str) -> Result<FlatpakSource, String> {
        let mut flatpak_source: FlatpakSource = match &format {
            FlatpakManifestFormat::YAML => match serde_yaml::from_str(&manifest_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak source manifest: {}.", e));
                }
            },
            FlatpakManifestFormat::JSON => {
                let json_content_without_comments = crate::utils::remove_comments_from_json(manifest_content);
                match serde_json::from_str(&json_content_without_comments) {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(format!("Failed to parse the Flatpak source manifest: {}.", e));
                    }
                }
            }
        };

        if let Err(e) = flatpak_source.is_valid() {
            return Err(e);
        }
        Ok(flatpak_source)
    }

    pub fn parse_many(
        format: FlatpakManifestFormat,
        manifest_content: &str,
    ) -> Result<Vec<FlatpakSource>, String> {
        let mut flatpak_sources: Vec<FlatpakSource> = match &format {
            FlatpakManifestFormat::YAML => match serde_yaml::from_str(&manifest_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak source manifest: {}.", e));
                }
            },
            FlatpakManifestFormat::JSON => {
                let json_content_without_comments = crate::utils::remove_comments_from_json(manifest_content);
                match serde_json::from_str(&json_content_without_comments) {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(format!("Failed to parse the Flatpak source manifest: {}.", e));
                    }
                }
            }
        };

        for flatpak_source in &flatpak_sources {
            if let Err(e) = flatpak_source.is_valid() {
                return Err(e);
            }
        }
        Ok(flatpak_sources)
    }

    pub fn is_valid(&self) -> Result<(), String> {
        if let Some(source_type) = &self.r#type {
            if !SOURCE_TYPES.contains(&source_type) {
                return Err(format!("Invalid source type {}.", source_type));
            }
        }
        Ok(())
    }

    pub fn get_url(&self) -> Option<String> {
        match &self.url {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    pub fn get_all_mirror_urls(&self) -> Vec<String> {
        let mut response: Vec<String> = vec![];
        if let Some(urls) = &self.mirror_urls {
            for url in urls {
                response.push(url.to_string());
            }
        }
        return response;
    }

    pub fn get_all_urls(&self) -> Vec<String> {
        let mut response: Vec<String> = vec![];
        if let Some(url) = &self.url {
            response.push(url.to_string());
        }
        if let Some(urls) = &self.mirror_urls {
            for url in urls {
                response.push(url.to_string());
            }
        }
        return response;
    }

    pub fn get_type_name(&self) -> String {
        if let Some(t) = &self.r#type {
            return t.to_string();
        }
        return "empty".to_string();
    }

    pub fn supports_mirror_urls(&self) -> bool {
        let type_name = self.get_type_name();
        // FIXME why are mirror urls not supported for types git, svn and bzr.
        if type_name == ARCHIVE || type_name == FILE {
            return true;
        }
        return false;
    }
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
/// See <https://github.com/flathub/flatpak-external-data-checker#changes-to-flatpak-manifests>
/// for the specification
pub struct FlatpakDataCheckerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_source: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_single_source_manifest() {
        match FlatpakSource::parse(
            FlatpakManifestFormat::YAML,
            r###"
            type: file
            path: apply_extra.sh
            "###,
        ) {
            Err(e) => std::panic::panic_any(e),
            Ok(source) => {
                assert_eq!(source.path, Some("apply_extra.sh".to_string()));
            }
        }
    }

    #[test]
    pub fn test_parse_multiple_source_manifests() {
        match FlatpakSource::parse_many(
            FlatpakManifestFormat::YAML,
            r###"
            - type: file
              path: apply_extra.sh

            - type: file
              path: com.wps.Office.metainfo.xml

            - type: file
              path: wps.sh

            - type: extra-data
              filename: wps-office.deb
              only-arches:
                - x86_64
              url: https://wdl1.pcfg.cache.wpscdn.com/wps-office_11.1.0.10702.XA_amd64.deb
              sha256: 390a8b358aaccdfda54740d10d5306c2543c5cd42a7a8fd5c776ccff38492992
              size: 275210770
              installed-size: 988671247
              x-checker-data:
                type: html
                url: https://linux.wps.com/js/meta.js
                version-pattern: version\s*=\s*"([\d.-]+)"
                url-pattern: download_link_deb\s*=\s*"(http[s]?://[\w\d$-_@.&+]+)"
            "###,
        ) {
            Err(e) => std::panic::panic_any(e),
            Ok(sources) => {
                assert_eq!(sources.len(), 4);
                let last_source = sources.last().unwrap();
                assert_eq!(last_source.filename, Some("wps-office.deb".to_string()));
            }
        }
    }

    #[test]
    pub fn test_parse_invalid_source_type() {
        let source_manifest = r###"
            type: not_a_valid_source_type
            path: apply_extra.sh
        "###;
        match FlatpakSource::parse(FlatpakManifestFormat::YAML, source_manifest) {
            Ok(_source) => {
                panic!("We should not be able to parse a source manifest with an invalid source type");
            }
            Err(e) => {
                assert!(e.to_string().contains("Invalid source type"));
            }
        }
    }

    #[test]
    pub fn test_parse_missing_source_type() {
        let source_manifest = r###"
            url: "https://ftp.gnu.org/gnu/gcc/gcc-7.5.0/gcc-7.5.0.tar.xz"
            sha256: "b81946e7f01f90528a1f7352ab08cc602b9ccc05d4e44da4bd501c5a189ee661"
        "###;
        match FlatpakSource::parse(FlatpakManifestFormat::YAML, source_manifest) {
            Ok(source) => {
                assert!(source.url.is_some());
            }
            Err(e) => {
                panic!(
                    "We should be able to parse a source manifest without a source type: {}",
                    e
                );
            }
        }
    }
}
