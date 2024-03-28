/*
 * Copyright (c) Martin Pompéry
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the crate's root directory of this source tree.
 */
use crate::datamodel::*;
use chrono::prelude::*;
use rust_decimal_macros::dec;
use uuid::uuid;

lazy_static! {
    static ref TIME_PERIOD_START: DateTime<Utc> = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
    static ref TIME_PERIOD_END: DateTime<Utc> = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
    static ref CREATED_AT: DateTime<Utc> = Utc.ymd(2022, 5, 22).and_hms(21, 47, 32);
    static ref UPDATED_AT: DateTime<Utc> = Utc.ymd(2022, 5, 22).and_hms(21, 47, 35);
    static ref PCF: CarbonFootprint = CarbonFootprint {
        cross_sectoral_standards_used: CrossSectoralStandardSet(vec![CrossSectoralStandard::Ghgp]),
        fossil_ghg_emissions: dec!(0.123).into(),
        primary_data_share: Some(56.12.into()),
        boundary_processes_description: Some(String::from("End-of-life included")),
        secondary_emission_factor_sources: Some(EmissionFactorDSSet(vec![EmissionFactorDS {
            name: String::from("Ecoinvent").into(),
            version: String::from("1.2.3").into(),
        }])),
        reference_period_start: *TIME_PERIOD_START,
        reference_period_end: *TIME_PERIOD_END,
        /*geography_region_or_subregion: None,
        geography_country: Some(ISO3166CC(String::from("FR"))),
        geography_country_subdivision: None, */
        geographic_scope: Some(GeographicScope::Country { geography_country: ISO3166CC(String::from("FR")) }),
        i_luc_ghg_emissions: None,
        d_luc_ghg_emissions: None,
        land_management_ghg_emissions: Some(dec!(0.001).into()),
        other_biogenic_ghg_emissions: Some(dec!(0).into()),
        biogenic_carbon_content: dec!(0.0).into(),
        product_or_sector_specific_rules: ProductOrSectorSpecificRuleSet(vec![
            ProductOrSectorSpecificRule {
                operator: ProductOrSectorSpecificRuleOperator::EPDInternational,
                rule_names: vec![String::from("ABC 2021").into()].into(),
                other_operator_name: None,
            }
        ]),
        allocation_rules_description: None,
        declared_unit: DeclaredUnit::Liter,
        unitary_product_amount: dec!(12.0).into(),
        aircraft_ghg_emissions: None,
        assurance: Some(Assurance {
            assurance: true,
            coverage: Some(AssuranceCoverage::ProductLevel),
            ..Default::default()
        }),
        biogenic_accounting_methodology: None,
        biogenic_carbon_withdrawal: None,
        characterization_factors: CharacterizationFactors::Ar6,
        ipcc_characterization_factors_sources: IpccCharacterizationFactorsSources::from(vec![String::from("AR6").into()]),
        dqi: None,
        exempted_emissions_percent: ExemptedEmissionsPercent(0.0),
        exempted_emissions_description: "".to_string(),
        fossil_carbon_content: dec!(0.0).into(),
        packaging_emissions_included: false,
        packaging_ghg_emissions: None,
        p_cf_excluding_biogenic: dec!(0.0).into(),
        p_cf_including_biogenic: Some(dec!(0.0).into()),
        uncertainty_assessment_description: None,
    };
}

fn base() -> ProductFootprint {
    ProductFootprint {
        spec_version: String::from("2.0.0").into(),
        id: PfId(uuid!("d9be4477-e351-45b3-acd9-e1da05e6f633")),
        preceding_pf_ids: Some(vec![PfId(uuid!("c3028ee9-d595-4779-a73a-290bfa7505d6"))].into()),
        version: VersionInteger(0),
        created: *CREATED_AT,
        updated: None,
        status: PfStatus::Active,
        status_comment: None,
        validity_period_start: None,
        validity_period_end: None,
        company_name: String::from("My Corp").into(),
        company_ids: CompanyIdSet(vec![
            String::from("urn:uuid:51131FB5-42A2-4267-A402-0ECFEFAD1619").into(),
            String::from("urn:epc:id:sgln:4063973.00000.8").into(),
        ]),
        product_ids: ProductIdSet(vec![String::from("urn:gtin:4712345060507").into()]),
        product_name_company: String::from("Green Ethanol").into(),
        product_category_cpc: String::from("3342").into(),
        pcf: PCF.clone(),
        comment: "".into(),
        product_description: "Cote'd Or Ethanol".into(),
    }
}

lazy_static! {
    pub(crate) static ref PCF_DEMO_DATA: Vec<ProductFootprint> = vec![
        ProductFootprint {
            id: PfId(uuid!("d9be4477-e351-45b3-acd9-e1da05e6f633")),
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("c3028ee9-d595-4779-a73a-290bfa7505d6")),
            product_name_company: String::from("Green Ethanol Nuits-Saint-Georges").into(),
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("9faa3200-8b65-4116-bf57-4ec6cff7aed2")),
            product_name_company: String::from("Green Ethanol Pontigny").into(),
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("02a090d6-5c91-4290-855b-7ad4627903ef")),
            product_name_company: String::from("Green Ethanol Meursault").into(),
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("18d88391-f4c7-4b79-a302-01c092154177")),
            updated: Some(*UPDATED_AT),
            product_name_company: String::from("Green Ethanol Puligny-Montrachet").into(),
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("c20ac3d2-3209-4454-b7bc-073344824d73")),
            product_name_company: String::from("Green Ethanol Chassagne-Montrachet").into(),
            version: VersionInteger(1),
            updated: Some(*UPDATED_AT),
            pcf: CarbonFootprint {
                geographic_scope: Some(GeographicScope::Regional {
                    geography_region_or_subregion: UNRegionOrSubregion::WesternEurope
                }),
                ..PCF.clone()
            },
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("3893bb5d-da16-4dc1-9185-11d97476c254")),
            product_name_company: String::from("Green Ethanol Volnay").into(),
            version: VersionInteger(42),
            updated: Some(*UPDATED_AT),
            pcf: CarbonFootprint {
                geographic_scope: None, // i.e. global
                ..PCF.clone()
            },
            ..base()
        },
        ProductFootprint {
            id: PfId(uuid!("3392ff32-421e-44b5-966f-d02df90d91b2")),
            product_name_company: String::from("Green Ethanol Mont-Saint-Sulpice").into(),
            pcf: CarbonFootprint {
                geographic_scope: Some(GeographicScope::Subdivision {
                    geography_country_subdivision: String::from("FR-89").into(),
                }),
                ..PCF.clone()
            },
            ..base()
        },
    ];
}
