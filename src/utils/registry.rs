use windows::core::HRESULT;
use windows_registry::*;

/// Represents the different registry hives available on a Windows system.
#[derive(Debug, Copy, Clone)]
pub enum RegistryHive {
    ClassesRoot,
    CurrentConfig,
    CurrentUser,
    DynData,
    LocalMachine,
    PerformanceData,
    Users,
}

/// Represents the different registry view types (64-bit or 32-bit).
#[derive(Debug, Copy, Clone)]
pub enum RegistryHiveType {
    X64,
    X86,
}

/// Opens the base registry key for the given hive and registry view (x64 or x86).
///
/// # Arguments
///
/// * `hive` - The registry hive to open.
/// * `hive_type` - The registry view type (x64 or x86).
///
/// # Returns
///
/// * `Ok(Some(Key))` if the key was successfully opened.
/// * `Ok(None)` if the key was not found.
/// * `Err(e)` if there was an error opening the key.
pub fn open_base_key(hive: RegistryHive, hive_type: RegistryHiveType) -> Result<Option<Key>> {
    let base = match hive {
        RegistryHive::ClassesRoot => CLASSES_ROOT,
        RegistryHive::CurrentConfig => CURRENT_CONFIG,
        RegistryHive::CurrentUser => CURRENT_USER,
        // RegistryHive::DynData => DYN_DATA,
        RegistryHive::LocalMachine => LOCAL_MACHINE,
        // RegistryHive::PerformanceData => PERFORMANCE_DATA,
        RegistryHive::Users => USERS,
        _ => panic!("todo registry kind {:?}", hive),
    };

    // Attempt to open the base key (empty string means the root key)
    match base.options().read().open("") {
        Ok(key) => Ok(Some(key)),
        Err(e) => {
            // If the underlying error’s raw OS error is 2 (ERROR_FILE_NOT_FOUND),
            // we return None rather than propagating the error.
            if e.code() == HRESULT(2) {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

/// Opens a subkey for a given registry hive and path.
///
/// # Arguments
///
/// * `hive` - The registry hive to query.
/// * `path` - The path within the hive to query.
///
/// # Returns
///
/// * `Ok(Key)` if the subkey was successfully opened.
/// * `Err(e)` if there was an error opening the subkey.
pub fn open_sub_key(hive: RegistryHive, path: &str) -> Result<Key> {
    let base_maybe = open_base_key(hive, RegistryHiveType::X64)?;

    match base_maybe {
        Some(base) => base.open(path),
        None => Err(HRESULT::from_nt(0).into()),
    }
}

fn get_value(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

/// Retrieves a string value from a given registry hive, path, and value name.
///
/// # Arguments
///
/// * `hive` - The registry hive to query.
/// * `path` - The path within the hive to query.
/// * `name` - The name of the value to retrieve.
///
/// # Returns
///
/// * `Ok(String)` containing the value.
/// * `Err(e)` if there was an error retrieving the value.
pub fn get_string_value(hive: RegistryHive, path: &str, name: &str) -> Result<String> {
    let key = open_sub_key(hive, path)?;
    return key.get_value(name)?.try_into();
}

fn get_multi_string_value(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

fn get_expanded_string_value(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

fn get_dword_value(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

fn get_qword_value(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

/// Retrieves a string value from a given registry hive, path, and value name.
///
/// # Arguments
///
/// * `hive` - The registry hive to query.
/// * `path` - The path within the hive to query.
/// * `name` - The name of the value to retrieve.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` containing the binary values.
/// * `Err(e)` if there was an error retrieving the value.
pub fn get_binary_value(hive: RegistryHive, path: &str, name: &str) -> Result<Vec<u8>> {
    let value = open_sub_key(hive, path)?.get_value(name)?;

    match value.get(0..value.len()) {
        Some(value) => return Ok(value.to_vec()),
        None => Err(HRESULT::from_nt(0).into()),
    }
}

fn get_values(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

/// Retrieves the names of the subkeys for a given registry hive and path.
///
/// # Arguments
///
/// * `hive` - The registry hive to query.
/// * `path` - The path within the hive to query.
///
/// # Returns
///
/// * `Ok(Vec<String>)` containing the names of the subkeys.
/// * `Err(e)` if there was an error querying the subkeys.
pub fn get_sub_key_names(hive: RegistryHive, path: &str) -> Result<Vec<String>> {
    let base_maybe = open_base_key(hive, RegistryHiveType::X64)?;

    match base_maybe {
        Some(base) => Ok(base.open(path)?.keys()?.map(|key| key).collect()),
        None => Ok(vec![]),
    }
}

fn get_user_sids(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

fn get_hive(hive: RegistryHive, path: &str, value: &str) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests opening the CURRENT_USER hive using the 64-bit view.
    #[test]
    fn test_open_current_user_x64() {
        // Attempt to open the CURRENT_USER hive using the 64-bit view.
        let key = open_base_key(RegistryHive::CurrentUser, RegistryHiveType::X64)
            .expect("Failed to open key");
        // When run on a normal Windows system, CURRENT_USER should always exist.
        assert!(key.is_some());
    }

    /// Tests retrieving the names of subkeys in the SOFTWARE key of the LOCAL_MACHINE hive.
    #[test]
    fn test_get_sub_key_names_basic() {
        let strings =
            get_sub_key_names(RegistryHive::LocalMachine, "SOFTWARE").expect("Failed to open key");
        println!("{:?}", strings);
        assert!(strings.len() > 0)
    }
}
