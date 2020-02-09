/// Deeply nested enum that represents a location where a bucket might store its files.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Location {
    /// Objects are stored in a single location.
    Single(SingleRegion),
    /// Objects are stored redundantly across multiple locations.
    Multi(MultiRegion),
    /// Objects are stored redundantly accross two locations.
    Dual(DualRegion),
}

impl Default for Location {
    fn default() -> Location {
        Location::Single(SingleRegion::NorthAmerica(NALocation::SouthCarolina))
    }
}

/// The possible options for single regions.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SingleRegion {
    /// All options in North America.
    NorthAmerica(NALocation),
    /// All options in South America.
    SouthAmerica(SALocation),
    /// All options in Europe.
    Europe(EuropeLocation),
    /// All options in Asia.
    Asia(AsiaLocation),
    /// All options in Australia.
    Australia(AusLocation),
}

/// All options in North America.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NALocation {
    /// Store the files in Montréal.
    #[serde(rename = "NORTHAMERICA-NORTHEAST1")]
    Montreal,
    /// Store the files in Iowa.
    #[serde(rename = "US-CENTRAL1")]
    Iowa,
    /// Store the files in South Carolina.
    #[serde(rename = "US-EAST1")]
    SouthCarolina,
    /// Store the files in Northern Virginia.
    #[serde(rename = "US-EAST4")]
    NorthernVirginia,
    /// Store the files in Oregon.
    #[serde(rename = "US-WEST1")]
    Oregon,
    /// Store the files in Los Angeles.
    #[serde(rename = "US-WEST2")]
    LosAngeles,
}

/// All options in South America.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SALocation {
    /// Store the files in Soa Paulo.
    #[serde(rename = "SOUTHAMERICA-EAST1")]
    SaoPaulo,
}

/// All options in Europe.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EuropeLocation {
    /// Store the files in Finland.
    #[serde(rename = "EUROPE-NORTH1")]
    Finland,
    /// Store the files in Belgium.
    #[serde(rename = "EUROPE-WEST1")]
    Belgium,
    /// Store the files in London.
    #[serde(rename = "EUROPE-WEST2")]
    London,
    /// Store the files in Frankfurt.
    #[serde(rename = "EUROPE-WEST3")]
    Frankfurt,
    /// Store the files in the Netherlands.
    #[serde(rename = "EUROPE-WEST4")]
    Netherlands,
    /// Store the files in Zurich.
    #[serde(rename = "EUROPE-WEST6")]
    Zurich,
}

/// ALl options in Asia.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AsiaLocation {
    /// Store the files in Taiwan.
    #[serde(rename = "ASIA-EAST1")]
    Taiwan,
    /// Store the files in Hong Kong.
    #[serde(rename = "ASIA-EAST2")]
    HongKong,
    /// Store the files in Tokyo.
    #[serde(rename = "ASIA-NORTHEAST1")]
    Tokyo,
    /// Store the files in Osaka.
    #[serde(rename = "ASIA-NORTHEAST2")]
    Osaka,
    /// Store the files in Mumbai.
    #[serde(rename = "ASIA-SOUTH1")]
    Mumbai,
    /// Store the files in Singapore.
    #[serde(rename = "ASIA-SOUTHEAST1")]
    Singapore,
}

/// All options in Australia.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AusLocation {
    /// Store the files in Sydney.
    #[serde(rename = "AUSTRALIA-SOUTHEAST1")]
    Sydney,
}

/// The possible options for multi-region storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MultiRegion {
    /// Data centers in Asia
    Asia,
    /// Data centers in the European Union
    ///
    /// Object data added to a bucket in the EU multi-region is not stored in the EUROPE-WEST2 or
    /// EUROPE-WEST6 data center.
    Eu,
    /// Data centers in the United States
    Us,
}

/// The possible options for dual-region storage
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DualRegion {
    /// EUROPE-NORTH1 and EUROPE-WEST4. Additionally, object metadata may be stored in EUROPE-WEST1.
    Eur4,
    /// US-CENTRAL1 and US-EAST1. Additionally, object metadata may be stored in Tulsa, Oklahoma.
    Nam4,
}
