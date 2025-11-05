use std::collections::HashMap;

use windows::{
  core::{Interface, HSTRING},
  Foundation::{IPropertyValue, PropertyType},
};

/// Reads MDM configuration settings from the Managed.App.Settings container.
///
/// This function accesses the Windows ApplicationData API to retrieve
/// configuration settings written by Microsoft Intune for the current application.
///
/// # Returns
///
/// Returns a HashMap containing key-value pairs where all values are converted to strings,
/// or an error if Windows API calls fail.
pub fn get_mdm_settings_impl() -> windows::core::Result<HashMap<String, String>> {
  // Get current application data and navigate to `Managed.App.Settings` container.
  let app_data = windows::Storage::ApplicationData::Current()?;
  let local_settings = app_data.LocalSettings()?;
  let container = local_settings
    .Containers()?
    .Lookup(&HSTRING::from("Managed.App.Settings"))?;
  let values = container.Values()?;

  // Parse all key-value pairs into a HashMap.
  let mdm_settings: HashMap<String, String> = values
    .into_iter()
    .filter_map(|pair| {
      let key = pair.Key().ok()?.to_string_lossy();
      let value = pair.Value().ok()?;
      let property_value = value.cast::<IPropertyValue>().ok()?;

      let string_value = match property_value.Type().ok()? {
        // Accept any type of int, as we map it to a string anyway.
        PropertyType::Int16 => property_value.GetInt16().ok()?.to_string(),
        PropertyType::Int32 => property_value.GetInt32().ok()?.to_string(),
        PropertyType::Int64 => property_value.GetInt64().ok()?.to_string(),
        PropertyType::UInt16 => property_value.GetUInt16().ok()?.to_string(),
        PropertyType::UInt32 => property_value.GetUInt32().ok()?.to_string(),
        PropertyType::UInt64 => property_value.GetUInt64().ok()?.to_string(),
        PropertyType::Boolean => property_value.GetBoolean().ok()?.to_string(),
        PropertyType::String => property_value.GetString().ok()?.to_string_lossy(),
        _other => {
          // Silently skip unsupported property types.
          return None;
        }
      };

      Some((key, string_value))
    })
    .collect();

  Ok(mdm_settings)
}
