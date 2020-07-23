#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.50-alpha4/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Sum {
    value: i32,
}

impl Sum {
    pub fn new(value: i32) -> Self {
        Sum {
            value
        }
    }
    pub fn entry(self) -> Entry {
        Entry::App("sum".into(), self.into())
    }
}

#[zome]
mod callee_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn sum_entry_def() -> ValidatingEntryType {
        entry!(
            name: "sum",
            description: "this is the sum entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Sum>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn handle_sum_and_commit(num1: i32, num2: i32) -> ZomeApiResult<Address> {
        let sum_value = num1 + num2;
        let new_sum: Sum = Sum::new(sum_value);
        let sum_entry: Entry = new_sum.entry();
        let sum_entry_address = hdk::commit_entry(&sum_entry)?;
        Ok(sum_entry_address)
    }
}
