use serde::{Deserialize, Serialize};

// * Particular packages *//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageInfo {
    pub package: Package,
    pub maintainer: Maintainer,
    pub source: Source,
    pub bin: Bin,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub size: Option<String>,
    pub conditions: Option<Conditions>,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conditions {
    pub dependencies: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub hash: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maintainer {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bin {
    pub name: String,
}

//  *  Whole available packages *//
pub mod available_packages {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Available {
        pub packages: Vec<Packagefields>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Packagefields {
        pub name: String,
        pub binary_size: Option<String>,
        pub maintainer: Option<String>,
        pub email: Option<String>,
        pub description: Option<String>,
        pub version: Option<String>,
        pub homepage: Option<String>,
        pub license: Option<String>,
    }
    impl Packagefields {
        pub fn with_defaults(package: Packagefields) -> Packagefields {
            Packagefields {
                binary_size: package.binary_size.or(Some("Default Size".to_string())),
                maintainer: package
                    .maintainer
                    .or(Some("Default Maintainer".to_string())),
                email: package.email.or(Some("Default Email".to_string())),
                description: package
                    .description
                    .or(Some("Default Description".to_string())),
                version: package.version.or(Some("Default Version".to_string())),
                homepage: package.homepage.or(Some("Default Homepage".to_string())),
                license: package.license.or(Some("Default License".to_string())),
                ..package
            }
        }
    }
}
