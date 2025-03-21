use std::path::Path;

pub(super) mod cargo;
pub(super) mod maven;

pub trait DependencyResolver {
    fn get_dependencies(&self, path: &Path) -> Vec<(String, Vec<String>)>;
}
