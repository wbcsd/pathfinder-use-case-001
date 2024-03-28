/*
 * Copyright (c) Martin Pompéry
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the crate's root directory of this source tree.
 */

//! Use Case 001 Data Model definitions
//!
//! See https://www.carbon-transparency.com for further details.
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use schemars::schema::{ArrayValidation, NumberValidation, Schema, StringValidation};
use schemars::JsonSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "ProductFootprint" of Tech Spec Version 2
pub struct ProductFootprint {
    pub id: PfId,
    pub spec_version: SpecVersionString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preceding_pf_ids: Option<NonEmptyPfIdVec>,
    pub version: VersionInteger,
    pub created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    pub status: PfStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period_end: Option<DateTime<Utc>>,
    pub company_name: NonEmptyString,
    pub company_ids: CompanyIdSet,
    pub product_description: String,
    pub product_ids: ProductIdSet,
    pub product_category_cpc: NonEmptyString,
    pub product_name_company: NonEmptyString,
    pub comment: String,

    pub pcf: CarbonFootprint,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "CarbonFootprint" of Spec Version 2
pub struct CarbonFootprint {
    pub declared_unit: DeclaredUnit,
    pub unitary_product_amount: StrictlyPositiveDecimal,
    pub p_cf_excluding_biogenic: PositiveDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_cf_including_biogenic: Option<WrappedDecimal>,
    pub fossil_ghg_emissions: PositiveDecimal,
    pub fossil_carbon_content: PositiveDecimal,
    pub biogenic_carbon_content: PositiveDecimal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_luc_ghg_emissions: Option<PositiveDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_management_ghg_emissions: Option<PositiveDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_biogenic_ghg_emissions: Option<PositiveDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_luc_ghg_emissions: Option<PositiveDecimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub biogenic_carbon_withdrawal: Option<PositiveDecimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aircraft_ghg_emissions: Option<PositiveDecimal>,

    pub characterization_factors: CharacterizationFactors,

    pub ipcc_characterization_factors_sources: IpccCharacterizationFactorsSources,

    pub cross_sectoral_standards_used: CrossSectoralStandardSet,
    pub product_or_sector_specific_rules: ProductOrSectorSpecificRuleSet,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub biogenic_accounting_methodology: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary_processes_description: Option<String>,

    pub reference_period_start: DateTime<Utc>,
    pub reference_period_end: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic_scope: Option<GeographicScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_emission_factor_sources: Option<EmissionFactorDSSet>,

    pub exempted_emissions_percent: ExemptedEmissionsPercent,

    pub exempted_emissions_description: String,

    pub packaging_emissions_included: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_ghg_emissions: Option<PositiveDecimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_rules_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncertainty_assessment_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_data_share: Option<Percent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dqi: Option<DataQualityIndicators>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assurance: Option<Assurance>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "PfId" of Spec Version 2
pub struct PfId(pub Uuid);

impl std::fmt::Display for PfId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum PfStatus {
    Active,
    Deprecated,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "DeclaredUnit" of Spec Version 2
pub enum DeclaredUnit {
    #[serde(rename = "liter")]
    Liter,
    #[serde(rename = "kilogram")]
    Kilogram,
    #[serde(rename = "cubic meter")]
    CubicMeter,
    #[serde(rename = "kilowatt hour")]
    KilowattHour,
    #[serde(rename = "megajoule")]
    Megajoule,
    #[serde(rename = "ton kilometer")]
    TonKilometer,
    #[serde(rename = "square meter")]
    SquareMeter,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "CrossSectoralStandard" of Spec Version 2
pub enum CrossSectoralStandard {
    #[serde(rename = "GHG Protocol Product standard")]
    Ghgp,
    #[serde(rename = "ISO Standard 14067")]
    ISO14067,
    #[serde(rename = "ISO Standard 14044")]
    ISO14044,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum CharacterizationFactors {
    #[serde(rename = "AR5")]
    Ar5,
    #[serde(rename = "AR6")]
    Ar6,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct IpccCharacterizationFactorsSource(String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct IpccCharacterizationFactorsSources(pub Vec<IpccCharacterizationFactorsSource>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct PositiveDecimal(Decimal);

/// a f64 in the 0..5 range
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ExemptedEmissionsPercent(pub f64);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct WrappedDecimal(Decimal);

/// a f64 in the 0..100 range
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "Percent" of Spec Version 2
pub struct Percent(f64);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct StrictlyPositiveDecimal(Decimal);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct NonEmptyString(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct NonEmptyStringVec(pub Vec<NonEmptyString>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct NonEmptyPfIdVec(pub Vec<PfId>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "CompanyIdSet" of Spec Version 2
pub struct CompanyIdSet(pub Vec<Urn>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "ProductIdSet" of Spec Version 2
pub struct ProductIdSet(pub Vec<Urn>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
/// Data Type "EmissionFactorDSSet" of Spec Version 2
pub struct EmissionFactorDSSet(pub Vec<EmissionFactorDS>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Urn(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct SpecVersionString(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "VersionInteger" of Spec Version 2
pub struct VersionInteger(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[serde(untagged)]
/// Encoded geographic scope rules of a Spec Version 2 `CarbonFootprint`
pub enum GeographicScope {
    Global,
    #[serde(rename_all = "camelCase")]
    Regional {
        geography_region_or_subregion: UNRegionOrSubregion,
    },
    #[serde(rename_all = "camelCase")]
    Country {
        geography_country: ISO3166CC,
    },
    #[serde(rename_all = "camelCase")]
    Subdivision {
        geography_country_subdivision: NonEmptyString,
    },
}

impl GeographicScope {
    pub fn geography_country(&self) -> Option<&str> {
        match self {
            GeographicScope::Country { geography_country } => Some(&geography_country.0),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
/// List of UN regions and subregions
pub enum UNRegionOrSubregion {
    Africa,
    Americas,
    Asia,
    Europe,
    Oceania,
    #[serde(rename = "Australia and New Zealand")]
    AustraliaAndNewZealand,
    #[serde(rename = "Central Asia")]
    CentralAsia,
    #[serde(rename = "Eastern Asia")]
    EasternAsia,
    #[serde(rename = "Eastern Europe")]
    EasternEurope,
    #[serde(rename = "Latin America and the Caribbean")]
    LatinAmericaAndTheCaribbean,
    Melanesia,
    Micronesia,
    #[serde(rename = "Northern Africa")]
    NorthernAfrica,
    #[serde(rename = "Northern America")]
    NorthernAmerica,
    #[serde(rename = "Northern Europe")]
    NorthernEurope,
    Polynesia,
    #[serde(rename = "South-eastern Asia")]
    SouthEasternAsia,
    #[serde(rename = "Southern Asia")]
    SouthernAsia,
    #[serde(rename = "Southern Europe")]
    SouthernEurope,
    #[serde(rename = "Sub-Saharan Africa")]
    SubSaharanAfrica,
    #[serde(rename = "Western Asia")]
    WesternAsia,
    #[serde(rename = "Western Europe")]
    WesternEurope,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct ProductOrSectorSpecificRuleSet(pub Vec<ProductOrSectorSpecificRule>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct CrossSectoralStandardSet(pub Vec<CrossSectoralStandard>);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
// TODO JSONSchema
pub struct ISO3166CC(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub enum ProductOrSectorSpecificRuleOperator {
    #[serde(rename = "PEF")]
    Pef,
    #[serde(rename = "EPD International")]
    EPDInternational,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
// TODO JsonSchema: add validation rule WRT operator == Other implying other_operator_name to be set; otherwise it must be empty.
pub struct ProductOrSectorSpecificRule {
    pub operator: ProductOrSectorSpecificRuleOperator,
    pub rule_names: NonEmptyStringVec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_operator_name: Option<NonEmptyString>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
// Data Type "EmissionFactorDS" of Spec Version 2
pub struct EmissionFactorDS {
    pub name: NonEmptyString,
    pub version: NonEmptyString,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "DataQualityIndicators" of Spec Version 2
pub struct DataQualityIndicators {
    pub coverage_percent: Percent,
    pub technological_d_q_r: StrictlyPositiveDecimal,
    pub temporal_d_q_r: StrictlyPositiveDecimal,
    pub geographical_d_q_r: StrictlyPositiveDecimal,
    pub completeness_d_q_r: StrictlyPositiveDecimal,
    pub reliability_d_q_r: StrictlyPositiveDecimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq, Default)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
/// Data Type "Assurance" of Spec Version 2
pub struct Assurance {
    pub assurance: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<AssuranceCoverage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<AssuranceLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<AssuranceBoundary>,

    pub provider_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum AssuranceCoverage {
    #[serde(rename = "corporate level")]
    CorporateLevel,
    #[serde(rename = "product line")]
    ProductLine,
    #[serde(rename = "PCF system")]
    PcfSystem,
    #[serde(rename = "product level")]
    ProductLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum AssuranceLevel {
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "reasonable")]
    Reasonable,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum AssuranceBoundary {
    #[serde(rename = "Gate-to-Gate")]
    GateToGate,
    #[serde(rename = "Cradle-to-Gate")]
    CradleToGate,
}

impl From<String> for IpccCharacterizationFactorsSource {
    fn from(s: String) -> IpccCharacterizationFactorsSource {
        IpccCharacterizationFactorsSource(s)
    }
}

impl From<Vec<IpccCharacterizationFactorsSource>> for IpccCharacterizationFactorsSources {
    fn from(v: Vec<IpccCharacterizationFactorsSource>) -> IpccCharacterizationFactorsSources {
        IpccCharacterizationFactorsSources(v)
    }
}

impl From<Decimal> for PositiveDecimal {
    fn from(f: Decimal) -> PositiveDecimal {
        PositiveDecimal(f)
    }
}

impl From<Decimal> for WrappedDecimal {
    fn from(f: Decimal) -> WrappedDecimal {
        WrappedDecimal(f)
    }
}

impl From<Decimal> for StrictlyPositiveDecimal {
    fn from(f: Decimal) -> StrictlyPositiveDecimal {
        StrictlyPositiveDecimal(f)
    }
}

impl From<f64> for ExemptedEmissionsPercent {
    fn from(f: f64) -> ExemptedEmissionsPercent {
        ExemptedEmissionsPercent(f)
    }
}

impl From<f64> for Percent {
    fn from(f: f64) -> Percent {
        Percent(f)
    }
}

impl From<String> for NonEmptyString {
    fn from(s: String) -> NonEmptyString {
        NonEmptyString(s)
    }
}

impl From<Vec<NonEmptyString>> for NonEmptyStringVec {
    fn from(v: Vec<NonEmptyString>) -> NonEmptyStringVec {
        NonEmptyStringVec(v)
    }
}

impl From<Vec<PfId>> for NonEmptyPfIdVec {
    fn from(v: Vec<PfId>) -> Self {
        NonEmptyPfIdVec(v)
    }
}

impl From<String> for Urn {
    fn from(s: String) -> Urn {
        Urn(s)
    }
}

impl From<String> for SpecVersionString {
    fn from(s: String) -> SpecVersionString {
        SpecVersionString(s)
    }
}

impl JsonSchema for IpccCharacterizationFactorsSource {
    fn schema_name() -> String {
        String::new()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            pattern: Some("^AR\\d+$".into()),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for IpccCharacterizationFactorsSources {
    fn schema_name() -> String {
        "IpccCharacterizationFactorsSources".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<IpccCharacterizationFactorsSource>(gen, Some(1))
    }
}

impl JsonSchema for NonEmptyString {
    fn schema_name() -> String {
        "NonEmptyString".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            max_length: None,
            min_length: Some(1),
            pattern: None,
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for NonEmptyStringVec {
    fn schema_name() -> String {
        "NonEmptyStringVec".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<NonEmptyString>(gen, Some(1))
    }
}

impl JsonSchema for NonEmptyPfIdVec {
    fn schema_name() -> String {
        "NonEmptyPfIdVec".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<PfId>(gen, Some(1))
    }
}

impl JsonSchema for ProductOrSectorSpecificRuleSet {
    fn schema_name() -> String {
        "ProductOrSectorSpecificRuleSet".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<ProductOrSectorSpecificRule>(gen, None)
    }
}

impl JsonSchema for Urn {
    fn schema_name() -> String {
        "GenericURN".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            pattern: Some("^([uU][rR][nN]):".into()),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for PositiveDecimal {
    fn schema_name() -> String {
        "PositiveDecimal".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            pattern: Some(String::from("^\\d+(\\.\\d+)?$")),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for WrappedDecimal {
    fn schema_name() -> String {
        "Decimal".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            pattern: Some(String::from("^-?\\d+(\\.\\d+)?$")),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for StrictlyPositiveDecimal {
    fn schema_name() -> String {
        "StrictlyPositiveDecimal".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::new(StringValidation {
            pattern: Some(String::from(
                "^(\\d*[1-9]\\d*([\\.]\\d+)?|\\d+(\\.\\d*[1-9]\\d*)?)$",
            )),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for ExemptedEmissionsPercent {
    fn schema_name() -> String {
        "ExemptedEmissionsPercent".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut s = match f64::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.number = Some(Box::new(NumberValidation {
            minimum: Some(0.00),
            maximum: Some(5.0),
            ..(NumberValidation::default())
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for Percent {
    fn schema_name() -> String {
        "Percent".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut s = match f64::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.number = Some(Box::new(NumberValidation {
            minimum: Some(0.00),
            maximum: Some(100.0),
            ..(NumberValidation::default())
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for VersionInteger {
    fn schema_name() -> String {
        "VersionInteger".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut s = match i32::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.number = Some(Box::new(NumberValidation {
            minimum: Some(0.00),
            maximum: Some(i32::MAX as f64),
            ..(NumberValidation::default())
        }));

        Schema::Object(s)
    }
}

impl JsonSchema for CompanyIdSet {
    fn schema_name() -> String {
        "CompanyIdSet".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<Urn>(gen, Some(1))
    }
}

impl JsonSchema for CrossSectoralStandardSet {
    fn schema_name() -> String {
        "CrossSectoralStandardSet".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<CrossSectoralStandard>(gen, Some(1))
    }
}

impl JsonSchema for ProductIdSet {
    fn schema_name() -> String {
        "ProductIdSet".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<Urn>(gen, Some(1))
    }
}

impl JsonSchema for EmissionFactorDSSet {
    fn schema_name() -> String {
        "EmissionFactorDSSet".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        json_set_schema::<EmissionFactorDS>(gen, Some(1))
    }
}

impl JsonSchema for SpecVersionString {
    fn schema_name() -> String {
        "VersionString".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut s = match String::json_schema(gen) {
            Schema::Object(s) => s,
            Schema::Bool(_) => panic!("Unexpected base schema"),
        };

        s.string = Some(Box::from(StringValidation {
            pattern: Some("^\\d+\\.\\d+\\.\\d+(-\\d{8})?$".into()),
            min_length: Some(5),
            ..Default::default()
        }));

        Schema::Object(s)
    }
}

#[derive(Debug)]
pub enum UuidError {
    ParseError(uuid::Error),
    VersionError,
}

impl<'a> rocket::request::FromParam<'a> for PfId {
    type Error = UuidError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(param).map_err(UuidError::ParseError)?;
        if uuid.get_version_num() != 4 {
            Err(UuidError::VersionError)
        } else {
            Ok(PfId(uuid))
        }
    }
}

impl JsonSchema for PfId {
    fn schema_name() -> String {
        "PfId".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        if let Schema::Object(mut o) = String::json_schema(gen) {
            o.format = Some(String::from("uuid"));
            Schema::Object(o)
        } else {
            panic!("Unrecognized String base schema");
        }
    }
}

fn json_set_schema<T: JsonSchema>(
    gen: &mut schemars::gen::SchemaGenerator,
    min_items: Option<u32>,
) -> Schema {
    let mut s = match Vec::<T>::json_schema(gen) {
        Schema::Object(s) => s,
        Schema::Bool(_) => panic!("Unexpected base schema"),
    };

    s.array = Some(Box::new(ArrayValidation {
        unique_items: Some(true),
        min_items,
        items: Some(T::json_schema(gen).into()),
        ..Default::default()
    }));

    Schema::Object(s)
}
