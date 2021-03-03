use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
pub struct Dependencies {
    pub strapi: String,
}

// Minimized version of package.json
// Only strapi -> version is needed
