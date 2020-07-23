#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.50-alpha4/hdk/ for info on using the hdk library

#[zome]
mod caller_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn sum_and_get_address(num1: i32, num2: i32) -> ZomeApiResult<Address> {
        #[derive(Serialize, Deserialize, Debug, DefaultJson)]
        struct ZomeInput {
            num1: i32,
            num2: i32,
        };
        let call_input = ZomeInput {
            num1,
            num2,
        };

        let sum_address_string = hdk::call(
            hdk::THIS_INSTANCE, 
            "callee", 
            Address::from(hdk::PUBLIC_TOKEN.to_owned()), 
            "handle_sum_and_commit", 
            call_input.into()
        )?;
    
        match serde_json::from_str(&sum_address_string.to_string()) {
            Ok(result) => result,
            Err(_e) => Err(ZomeApiError::from("parsing failed".to_owned()))
        }
    }
}
