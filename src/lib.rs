#![deny(clippy::all)]

#[cfg(target_os = "windows")]
mod windows;

use std::collections::HashMap;

use napi_derive::napi;

/// Reads MDM (Mobile Device Management) configuration settings for the current application.
///
/// On Windows, this function retrieves configuration settings written by Microsoft Intune
/// from the "Managed.App.Settings" container in the application's local settings.
///
/// On macOS and Linux, this function returns an empty object (no-op), as MDM reading
/// is not yet implemented for these platforms.
///
/// # Returns
///
/// A JavaScript object (Record<string, string>) containing key-value pairs of MDM settings.
/// All values are converted to strings regardless of their original type in the MDM store.
///
/// Returns an empty object if:
/// - The application is not running in a package context (Windows)
/// - The Managed.App.Settings container doesn't exist (no MDM configuration)
/// - Any Windows API calls fail
/// - Running on a non-Windows platform
///
/// This function never throws errors; it gracefully returns an empty object in all failure cases.
///
/// # Example
///
/// ```typescript
/// import { getMdmSettings } from '@gian-reto/mdm-config-reader';
///
/// const settings = getMdmSettings();
/// console.log(settings); // { "settingKey": "settingValue", ... } or {}
/// ```
#[napi]
pub fn get_mdm_settings() -> HashMap<String, String> {
  #[cfg(target_os = "windows")]
  {
    // Return empty HashMap if any Windows API call fails.
    windows::get_mdm_settings_impl().unwrap_or_default()
  }

  #[cfg(not(target_os = "windows"))]
  {
    // Return empty HashMap on non-Windows platforms (macOS, Linux, etc.).
    HashMap::new()
  }
}
