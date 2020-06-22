
struct BuildConfig {
    directory: PathBuf,
    dependencies: Vec<BuildRoot>,
};

trait Build {
    fn build(BuildConfig) -> bool;
}
