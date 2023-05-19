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
    Australia(AustraliaLocation),
    /// All options in the Middle East
    MiddleEast(MiddleEastLocation),
}

/// All options in North America.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NALocation {
    /// Store the files in Montréal.
    #[serde(rename = "NORTHAMERICA-NORTHEAST1")]
    Montreal,
    /// Store the files in Toronto.
    #[serde(rename = "NORTHAMERICA-NORTHEAST2")]
    Toronto,
    /// Store the files in Iowa.
    #[serde(rename = "US-CENTRAL1")]
    Iowa,
    /// Store the files in South Carolina.
    #[serde(rename = "US-EAST1")]
    SouthCarolina,
    /// Store the files in Northern Virginia.
    #[serde(rename = "US-EAST4")]
    NorthernVirginia,
    /// Store the files in Columbus.
    #[serde(rename = "US-EAST5")]
    Columbus,
    /// Store the files in Dallas.
    #[serde(rename = "US-SOUTH1")]
    Dallas,
    /// Store the files in Oregon.
    #[serde(rename = "US-WEST1")]
    Oregon,
    /// Store the files in Los Angeles.
    #[serde(rename = "US-WEST2")]
    LosAngeles,
    /// Store the files in Salt Lake City.
    #[serde(rename = "US-WEST3")]
    SaltLakeCity,
    /// Store the files in Las Vegas.
    #[serde(rename = "US-WEST4")]
    LasVegas,
}

/// All options in South America.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SALocation {
    /// Store the files in Soa Paulo.
    #[serde(rename = "SOUTHAMERICA-EAST1")]
    SaoPaulo,
    /// Store the files in Santiago.
    #[serde(rename = "SOUTHAMERICA-EAST2")]
    Santiago,
}

/// All options in Middle East.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MiddleEastLocation {
    /// Store the files in Doha.
    #[serde(rename = "ME-CENTRAL1")]
    Doha,
    /// Store the files in Tel Aviv.
    #[serde(rename = "ME-WEST1")]
    TelAviv,
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
    /// Store the files in Milan.
    #[serde(rename = "EUROPE-WEST8")]
    Milan,
    /// Store the files in Paris.
    #[serde(rename = "EUROPE-WEST9")]
    Paris,
    /// Store the files in Turin.
    #[serde(rename = "EUROPE-WEST12")]
    Turin,
    /// Store the files in Warsaw.
    #[serde(rename = "EUROPE-CENTRAL2")]
    Warsaw,
    /// Store the files in Madrid.
    #[serde(rename = "EUROPE-SOUTHWEST1")]
    Madrid,
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
    /// Store the files in Seoul.
    #[serde(rename = "ASIA-NORTHEAST3")]
    Seoul,
    /// Store the files in Mumbai.
    #[serde(rename = "ASIA-SOUTH1")]
    Mumbai,
    /// Store the files in Delhi.
    #[serde(rename = "ASIA-SOUTH2")]
    Delhi,
    /// Store the files in Singapore.
    #[serde(rename = "ASIA-SOUTHEAST1")]
    Singapore,
    /// Store the files in Jakarta.
    #[serde(rename = "ASIA-SOUTHEAST2")]
    Jakarta,
}

/// All options in Australia.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AustraliaLocation {
    /// Store the files in Sydney.
    #[serde(rename = "AUSTRALIA-SOUTHEAST1")]
    Sydney,
    /// Store the files in Melbourne.
    #[serde(rename = "AUSTRALIA-SOUTHEAST2")]
    Melbourne,
}

/// The possible options for multi-region storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MultiRegion {
    /// Data centers in Asia
    Asia,
    /// Data centers within member states of the European Union:
    ///
    /// Object data added to a bucket in the `EU` multi-region is not stored in the EUROPE-WEST2 (London) or
    /// EUROPE-WEST6 (Zurich) data centers.
    Eu,
    /// Data centers in the United States
    Us,
}

/// The possible options for dual-region storage
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DualRegion {
    /// Tokyo and Osaka.
    Asia1,
    /// Finland and Netherlands.
    Eur4,
    /// Iowa and South Carolina.
    Nam4,
}
