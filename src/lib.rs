use std::path::Path;
use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;

pub enum DepGraphResolver {
    Cargo
}

impl DepGraphResolver {
    pub fn topological_sort(&self, path: impl AsRef<Path>) -> Vec<String> {
        let mut graph = DiGraphMap::new();
        let mut all_packages = vec![];

        let dependencies = &self.get_dependencies(path);
        for (package, deps) in dependencies {
            graph.add_node(package);
            all_packages.push(package);
            for dep in deps {
                graph.add_node(dep);
                graph.add_edge(dep, package, 1);
            }
        }

        toposort(&graph, None).expect("Cycle detected! Dependencies must be acyclic.")
            .into_iter()
            .cloned()
            .collect()
    }

    fn get_dependencies(&self, path: impl AsRef<Path>) -> Vec<(String, Vec<String>)> {
        match self {
            DepGraphResolver::Cargo => {
                Self::resolve_cargo_workspace_dependencies(path)
            }
        }
    }


    fn resolve_cargo_workspace_dependencies(path: impl AsRef<Path> + Sized) -> Vec<(String, Vec<String>)> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(path.as_ref())
            .exec()
            .unwrap();

        let cargo_packages = metadata.workspace_packages();
        let mut deps: Vec<(String, Vec<String>)> = Vec::with_capacity(cargo_packages.len());

        for p in &cargo_packages {
            let packages_depedencies: Vec<_> = p
                .dependencies
                .iter()
                .filter(|d| cargo_packages.iter().any(|p| p.name == d.name))
                .collect();

            let package_deps = packages_depedencies.iter().map(|d| d.name.clone()).collect();
            deps.push((p.name.clone(), package_deps));
        }

        deps
    }
}