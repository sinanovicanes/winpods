use serde::Serialize;
use windows::{
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Media::Audio::{
            DEVICE_STATE_ACTIVE, ERole, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
            eCommunications, eConsole, eMultimedia, eRender,
        },
        System::{
            Com::{
                CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
                CoTaskMemFree, CoUninitialize, STGM_READ,
                StructuredStorage::{PROPVARIANT, PropVariantClear},
            },
            Variant::{VT_BSTR, VT_LPWSTR},
        },
    },
    core::{GUID, Interface, PCWSTR, PWSTR},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEndpoint {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub is_airpods_match: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioRouteSummary {
    pub matched_endpoint: Option<AudioEndpoint>,
    pub endpoints: Vec<AudioEndpoint>,
    pub roles_attempted: u32,
    pub roles_succeeded: u32,
    pub roles_failed: u32,
}

windows::core::imp::define_interface!(
    IPolicyConfig,
    IPolicyConfig_Vtbl,
    0xf8679f50_850a_41cf_9c72_430f290290c8
);
windows::core::imp::interface_hierarchy!(IPolicyConfig, windows::core::IUnknown);

#[repr(C)]
#[allow(non_snake_case)]
pub struct IPolicyConfig_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,
    pub GetMixFormat: usize,
    pub GetDeviceFormat: usize,
    pub ResetDeviceFormat: usize,
    pub SetDeviceFormat: usize,
    pub GetProcessingPeriod: usize,
    pub SetProcessingPeriod: usize,
    pub GetShareMode: usize,
    pub SetShareMode: usize,
    pub GetPropertyValue: usize,
    pub SetPropertyValue: usize,
    pub SetDefaultEndpoint:
        unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR, ERole) -> windows::core::HRESULT,
    pub SetEndpointVisibility: usize,
}

impl IPolicyConfig {
    unsafe fn set_default_endpoint(&self, device_id: &str, role: ERole) -> windows::core::Result<()> {
        let wide_id: Vec<u16> = device_id.encode_utf16().chain(Some(0)).collect();

        unsafe {
            (Interface::vtable(self).SetDefaultEndpoint)(
                Interface::as_raw(self),
                PCWSTR(wide_id.as_ptr()),
                role,
            )
            .ok()
        }
    }
}

pub fn list_render_endpoints(selected_device_name: Option<&str>) -> Result<Vec<AudioEndpoint>, String> {
    with_com_initialized(|| {
        let enumerator = device_enumerator()?;
        let default_id = default_render_endpoint_id(&enumerator).ok();
        let collection = unsafe {
            enumerator
                .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
                .map_err(|e| format!("Failed to enumerate audio outputs: {e}"))?
        };
        let count = unsafe {
            collection
                .GetCount()
                .map_err(|e| format!("Failed to count audio outputs: {e}"))?
        };
        let mut endpoints = Vec::with_capacity(count as usize);

        for index in 0..count {
            let device = unsafe {
                collection
                    .Item(index)
                    .map_err(|e| format!("Failed to read audio output {index}: {e}"))?
            };
            let id = device_id(&device)?;
            let name = endpoint_name(&device).unwrap_or_else(|_| id.clone());
            let is_default = default_id.as_deref() == Some(id.as_str());
            let is_airpods_match = selected_device_name
                .map(|selected_name| endpoint_matches_device(&name, selected_name))
                .unwrap_or_else(|| endpoint_name_looks_like_airpods(&name));

            endpoints.push(AudioEndpoint {
                id,
                name,
                is_default,
                is_airpods_match,
            });
        }

        Ok(endpoints)
    })
}

pub fn route_render_audio_to_device(selected_device_name: &str) -> Result<AudioRouteSummary, String> {
    with_com_initialized(|| {
        let endpoints = list_render_endpoints(Some(selected_device_name))?;
        let matched_endpoint = endpoints
            .iter()
            .find(|endpoint| endpoint.is_airpods_match)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "Windows is not exposing an active audio output endpoint for {selected_device_name}"
                )
            })?;

        let policy_config = policy_config()?;
        let mut summary = AudioRouteSummary {
            matched_endpoint: Some(matched_endpoint.clone()),
            endpoints,
            roles_attempted: 3,
            roles_succeeded: 0,
            roles_failed: 0,
        };

        for role in [eConsole, eMultimedia, eCommunications] {
            let result = unsafe { policy_config.set_default_endpoint(&matched_endpoint.id, role) };

            if result.is_ok() {
                summary.roles_succeeded += 1;
            } else {
                summary.roles_failed += 1;
            }
        }

        Ok(summary)
    })
}

fn with_com_initialized<T>(callback: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| format!("Failed to initialize Windows audio COM: {e}"))?;
    }

    let result = callback();

    unsafe {
        CoUninitialize();
    }

    result
}

fn device_enumerator() -> Result<IMMDeviceEnumerator, String> {
    unsafe {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create Windows audio enumerator: {e}"))
    }
}

fn policy_config() -> Result<IPolicyConfig, String> {
    const CLSID_POLICY_CONFIG_CLIENT: GUID =
        GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

    unsafe {
        CoCreateInstance(&CLSID_POLICY_CONFIG_CLIENT, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create Windows audio policy client: {e}"))
    }
}

fn default_render_endpoint_id(enumerator: &IMMDeviceEnumerator) -> Result<String, String> {
    let device = unsafe {
        enumerator
            .GetDefaultAudioEndpoint(eRender, eMultimedia)
            .map_err(|e| format!("Failed to read current default audio output: {e}"))?
    };

    device_id(&device)
}

fn device_id(device: &IMMDevice) -> Result<String, String> {
    let id = unsafe {
        device
            .GetId()
            .map_err(|e| format!("Failed to read audio output id: {e}"))?
    };
    let id_string = pwstr_to_string(id);

    unsafe {
        CoTaskMemFree(Some(id.0 as _));
    }

    Ok(id_string)
}

fn endpoint_name(device: &IMMDevice) -> Result<String, String> {
    let property_store = unsafe {
        device
            .OpenPropertyStore(STGM_READ)
            .map_err(|e| format!("Failed to open audio output properties: {e}"))?
    };
    let mut value = unsafe {
        property_store
            .GetValue(&PKEY_Device_FriendlyName)
            .map_err(|e| format!("Failed to read audio output name: {e}"))?
    };
    let name = propvariant_to_string(&value);

    unsafe {
        let _ = PropVariantClear(&mut value);
    }

    name.ok_or_else(|| "Audio output has no friendly name".to_string())
}

fn propvariant_to_string(value: &PROPVARIANT) -> Option<String> {
    let inner = unsafe { &value.Anonymous.Anonymous };

    if inner.vt == VT_LPWSTR {
        return Some(pwstr_to_string(unsafe { inner.Anonymous.pwszVal }));
    }

    if inner.vt == VT_BSTR {
        return Some(unsafe { inner.Anonymous.bstrVal.to_string() });
    }

    None
}

fn pwstr_to_string(value: PWSTR) -> String {
    unsafe { value.to_string().unwrap_or_default() }
}

fn endpoint_matches_device(endpoint_name: &str, selected_device_name: &str) -> bool {
    let endpoint_name = normalize_name(endpoint_name);
    let selected_device_name = normalize_name(selected_device_name);

    endpoint_name_looks_like_airpods(&endpoint_name)
        || endpoint_name.contains(&selected_device_name)
        || selected_device_name.contains(&endpoint_name)
}

fn endpoint_name_looks_like_airpods(name: &str) -> bool {
    let name = normalize_name(name);

    name.contains("airpods") || name.contains("beats")
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(['’', '\'', '`'], "")
        .replace(['-', '_', '(', ')'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
