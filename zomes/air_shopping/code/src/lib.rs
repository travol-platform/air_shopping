#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
// see https://developer.holochain.org/api/0.0.44-alpha3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FlightSegment {
    anchor_address: Address,
    secure_flight: bool,
    segment_key: String,
    departure: Departure,
    arrival: Arrival,
    marketing_carrier: MarketingCarrier,
    operation_carrier: OperatingCarrier,
    equipement: Equipment,
    class_of_service: ClassOfService,
    flight_detail: FlightDetail,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Departure {
    airport_code: String,
    timestamp: String,
    airport_name: String,
    terminal_name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Arrival {
    airport_code: String,
    timestamp: String,
    change_of_day: String,
    airport_name: String,
    terminal_name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MarketingCarrier {
    airline_id: String,
    name: String,
    flight_number: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OperatingCarrier {
    airline_id: String,
    name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Equipment {
    aircraft_code: i32,
    name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ClassOfService {
    code: i32,
    markting_name: MarketingName,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MarketingName {
    cabin_designator: i32,
    name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FlightDetail {
    flight_segment_type: String,
    flight_duration: String,
    stops: i32,
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
    anchor_address: Address,
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
    fn create_flight_segment(
        secure_flight: bool,
        segment_key: String,
        departure: Departure,
        arrival: Arrival,
        marketing_carrier: MarketingCarrier,
        operation_carrier: OperatingCarrier,
        equipement: Equipment,
        class_of_service: ClassOfService,
        flight_detail: FlightDetail,
    ) -> ZomeApiResult<Address> {
        let anchor_address =
            holochain_anchors::anchor("fligth_segment".to_string(), segment_key.clone())?;

        let flight_segment_entry = FlightSegment {
            anchor_address: anchor_address.clone(),
            secure_flight,
            segment_key,
            departure,
            arrival,
            marketing_carrier,
            operation_carrier,
            equipement,
            class_of_service,
            flight_detail,
        }
        .entry();
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
        let option_address = hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly(&("anchor->".to_owned() + &r#type)),
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
    fn create_fare(
        refs: String,
        list_key: String,
        fare_code: String,
        fare_basis_code: String,
    ) -> ZomeApiResult<Address> {
        let anchor_address = holochain_anchors::anchor("fare".to_string(), list_key.clone())?;

        let fare_entry = Fare {
            anchor_address: anchor_address.clone(),
            refs,
            list_key,
            fare_code,
            fare_basis_code,
        }
        .entry();
        let fare_address = hdk::commit_entry(&fare_entry)?;
        hdk::link_entries(&anchor_address, &fare_address.clone(), "anchor->fare", "")?;
        Ok(fare_address)
    }
}
