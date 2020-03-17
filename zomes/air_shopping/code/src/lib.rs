#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
// see https://developer.holochain.org/api/0.0.44-alpha3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FlightSegment {
    secure_flight: Option<bool>,
    segment_key: String,
    departure: Departure,
    arrival: Arrival,
    marketing_carrier: MarketingCarrier,
    operation_carrier: Option<OperatingCarrier>,
    equipement: Option<Equipment>,
    class_of_service: Option<ClassOfService>,
    flight_detail: Option<FlightDetail>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Departure {
    airport_code: String,
    timestamp: String,
    airport_name: Option<String>,
    terminal_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Arrival {
    airport_code: String,
    timestamp: Option<String>,
    change_of_day: Option<String>,
    airport_name: Option<String>,
    terminal_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MarketingCarrier {
    airline_id: String,
    name: Option<String>,
    flight_number: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OperatingCarrier {
    airline_id: Option<String>,
    name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Equipment {
    aircraft_code: String,
    name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ClassOfService {
    code: Option<String>,
    markting_name: Option<MarketingName>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MarketingName {
    cabin_designator: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct StopLocation {
    airport_code: String,
    arrival_timestamp: String,
    departure_timestamp: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FlightDetail {
    flight_segment_type: Option<String>,
    flight_duration: String,
    stops: Option<String>,
    stop_location: Vec<StopLocation>,
}
impl FlightSegment {
    fn entry(self) -> Entry {
        Entry::App("flight_segment".into(), self.into())
    }
}
impl Fare {
    fn entry(self) -> Entry {
        Entry::App("fare".into(), self.into())
    }
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Fare {
    refs: String,
    list_key: String,
    fare_code: String,
    fare_basis_code: String,
}

#[zome]
mod air_shopping {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }
    #[entry_def]
    fn flight_segment_def() -> ValidatingEntryType {
        entry!(
            name: "flight_segment",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<FlightSegment>| {
                Ok(())
            },
            links: [
                from!(
                    holochain_anchors::ANCHOR_TYPE,
                    link_type: "anchor->flight_segment",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                       Ok(())
                    }
                )
            ]
        )
    }
    #[entry_def]
    fn fare_def() -> ValidatingEntryType {
        entry!(
            name: "fare",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Fare>| {
                Ok(())
            },
            links: [
                from!(
                    holochain_anchors::ANCHOR_TYPE,
                    link_type: "anchor->fare",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                       Ok(())
                    }
                )
            ]
        )
    }
    #[zome_fn("hc_public")]
    fn create_flight_segment(flight_segment: FlightSegment) -> ZomeApiResult<Address> {
        let anchor_address = holochain_anchors::anchor(
            "flight_segment".to_string(),
            flight_segment.segment_key.clone(),
        )?;

        let flight_segment_entry = flight_segment.entry();
        let flight_segment_address = hdk::commit_entry(&flight_segment_entry)?;
        hdk::link_entries(
            &anchor_address,
            &flight_segment_address.clone(),
            "anchor->flight_segment",
            "",
        )?;
        Ok(flight_segment_address)
    }
    #[zome_fn("hc_public")]
    fn get_entry(r#type: String, key: String) -> ZomeApiResult<JsonString> {
        let anchor_address = holochain_anchors::anchor(r#type.clone(), key.clone())?;
        hdk::debug::<JsonString>(anchor_address.clone().into())?;

        let option_address = hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly(&("anchor->".to_string() + &r#type)),
            LinkMatch::Any,
        )?;
        if let Some(address) = option_address.addresses().last() {
            match hdk::get_entry(&address)? {
                Some(Entry::App(_tupe, json_string)) => Ok(json_string),
                _ => Err(ZomeApiError::Internal("This page no exist".to_string())),
            }
        } else {
            Err(ZomeApiError::Internal("This page no exist".to_string()))
        }
    }
    #[zome_fn("hc_public")]
    fn create_fare(fare: Fare) -> ZomeApiResult<Address> {
        let anchor_address = holochain_anchors::anchor("fare".to_string(), fare.list_key.clone())?;
        let fare_entry = fare.entry();
        let fare_address = hdk::commit_entry(&fare_entry)?;
        hdk::link_entries(&anchor_address, &fare_address.clone(), "anchor->fare", "")?;
        Ok(fare_address)
    }
}
