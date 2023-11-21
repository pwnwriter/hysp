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
    pub version: String,
    pub description: String,
    pub license: String,
    pub size: String,
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
    pub name: String,
    pub email: String,
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
        pub binary_size: String,
        pub maintainer: String,
        pub email: String,
        pub description: String,
        pub version: String,
        pub homepage: String,
        pub license: String,
    }
}
