use std::collections::HashMap;

use windows::{core::*,  Win32::System::Variant::*, Win32::System::Wmi::*};

pub struct WbemIterator<'a> {
    results: &'a IEnumWbemClassObject,
    fields: Vec<String>
}

impl<'a> WbemIterator<'a> {
    pub fn from(
        enumerator: &'a IEnumWbemClassObject,
        fields: Vec<String>
    )-> WbemIterator<'a> {
        return WbemIterator { 
            results: enumerator,
            fields: fields
        }
    }
}

impl<'a> Iterator for WbemIterator<'a> {
    // Assuming that each item you yield is a Result containing the antivirus product data.
    type Item = Result<HashMap<String, VARIANT>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut row = [None; 1];
        let mut returned = 0;
        
        let hr = unsafe { self.results.Next(WBEM_INFINITE, &mut row, &mut returned) };

        if let Err(e) = hr.ok() {
            return Some(Err(e));
        }

        let mut columns: HashMap<String, VARIANT> = HashMap::new();

        if let Some(instance) = row[0].as_ref() {
            for field in &self.fields {
                let mut value = VARIANT::default();

                unsafe { 
                    let res =  instance.Get(&HSTRING::from(field), 0, &mut value, None, None);
                    // all columns must be Ok
                    if let Some(e) = res.err() {
                        return Some(Err(e));
                    } 
                };
                columns.insert(field.to_string(), value);
            }
            Some(Ok(columns))
        } else {
            None
        }
    }
}