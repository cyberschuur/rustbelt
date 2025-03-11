pub mod formatter;
pub mod writer;

use inventory::iter;

use windows::{core::*, Win32::System::Com::*, Win32::System::Wmi::*};

use crate::{
    commands::base::registry::CommandRegistration,
    utils::{self, registry::RegistryHive},
};

pub struct Runtime {
    computer_name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    // TODO: add the following features that\
    // filter_results: bool,
    // delay_commands: String,
    // randomize_order: bool,
}

impl Runtime {
    pub fn new(
        username: Option<String>,
        password: Option<String>,
        computer_name: Option<String>,
    ) -> Result<Self> {
        unsafe {
            // Initialize COM and set up security
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
            CoInitializeSecurity(
                None,
                -1,
                None,
                None,
                RPC_C_AUTHN_LEVEL_DEFAULT,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
                None,
            )?;
        };

        return Ok(Self {
            username: username,
            password: password,
            computer_name: computer_name,
        });
    }

    pub fn wmi_query(&self, namespace: &str, query: &str) -> Result<IEnumWbemClassObject> {
        let username_bstr = self
            .username
            .as_ref()
            .map(BSTR::from)
            .unwrap_or_else(BSTR::new);
        let password_bstr = self
            .password
            .as_ref()
            .map(BSTR::from)
            .unwrap_or_else(BSTR::new);

        unsafe {
            let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
            let server = locator.ConnectServer(
                &BSTR::from(namespace),
                &username_bstr,
                &password_bstr,
                &BSTR::new(),
                0,
                &BSTR::new(),
                None,
            )?;

            // Execute the query to get antivirus products
            let enumerator = server.ExecQuery(
                &BSTR::from("WQL"),
                &BSTR::from(query),
                WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                None,
            )?;

            return Ok(enumerator);
        };
    }
}
