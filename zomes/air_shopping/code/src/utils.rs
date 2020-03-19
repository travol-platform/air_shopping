use crate::*;
pub fn validation_ref_price_class(entry: PriceClass) -> Result<(), String> {
    if let Some(class_of_service) = entry.class_of_service {
        if let Some(r#ref) = class_of_service.r#ref {
            let mut iter = r#ref.split_ascii_whitespace();
            if let Some(s) = iter.next() {
                let anchor_address =
                    match holochain_anchors::anchor("flight_segment".to_string(), s.to_string()) {
                        Ok(r) => r,
                        _ => return Err("hola".to_string()),
                    };
                let option_address = match hdk::get_links(
                    &anchor_address,
                    LinkMatch::Exactly("anchor->flight_segment"),
                    LinkMatch::Any,
                ) {
                    Ok(r) => r.addresses(),
                    _ => return Err("hola".to_string()),
                };
                if let None = option_address.last() {
                    return Err("no existe el vuelo".to_string());
                }
            } else {
                return Err("se necesita la referencia del vuelo ".to_string());
            }
            if let Some(s) = iter.next() {
                let anchor_address =
                    match holochain_anchors::anchor("fare".to_string(), s.to_string()) {
                        Ok(r) => r,
                        _ => return Err("hola".to_string()),
                    };
                let option_address = match hdk::get_links(
                    &anchor_address,
                    LinkMatch::Exactly("anchor->fare"),
                    LinkMatch::Any,
                ) {
                    Ok(r) => r.addresses(),
                    _ => return Err("hola".to_string()),
                };
                if let Some(_) = option_address.last() {
                    return Ok(());
                } else {
                    return Err("no existe el fare".to_string());
                }
            } else {
                return Err("se necesita la referencia del fare ".to_string());
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
