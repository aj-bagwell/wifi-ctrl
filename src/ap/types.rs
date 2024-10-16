use super::{error, Result};
use serde::{de, Deserialize, Serialize};

/// Status of the WiFi Station
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub state: String,
    pub phy: String,
    pub freq: String,
    pub num_sta_non_erp: String,
    pub num_sta_no_short_slot_time: String,
    pub num_sta_no_short_preamble: String,
    pub olbc: String,
    pub num_sta_ht_no_gf: String,
    pub num_sta_no_ht: String,
    pub num_sta_ht_20_mhz: String,
    pub num_sta_ht40_intolerant: String,
    pub olbc_ht: String,
    pub ht_op_mode: String,
    pub cac_time_seconds: String,
    pub cac_time_left_seconds: String,
    pub channel: String,
    pub secondary_channel: String,
    pub ieee80211n: String,
    pub ieee80211ac: String,
    pub ieee80211ax: String,
    pub beacon_int: String,
    pub dtim_period: String,
    pub ht_caps_info: String,
    pub ht_mcs_bitmask: String,
    pub supported_rates: String,
    pub max_txpower: String,
    pub bss: Vec<String>,
    pub bssid: Vec<String>,
    pub ssid: Vec<String>,
    pub num_sta: Vec<String>,
}

impl Status {
    pub fn from_response(response: &str) -> Result<Status> {
        crate::config::deserialize_str(response).map_err(|e| error::Error::ParsingWifiStatus {
            e,
            s: response.into(),
        })
    }
}

/// Configuration of the WiFi station
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bssid: String,
    pub ssid: String,
    #[serde(deserialize_with = "deserialize_enabled_bool")]
    pub wps_state: bool,
    #[serde(deserialize_with = "deserialize_i32")]
    pub wpa: i32,
    pub key_mgmt: String,
    pub group_cipher: String,
    pub rsn_pairwise_cipher: String,
    pub wpa_pairwise_cipher: String,
}

impl Config {
    /// Decode from the response sent from the supplicant
    /// ```
    /// # use wifi_ctrl::ap::Config;
    /// let resp = r#"
    ///bssid=e0:91:f5:7d:11:c0
    ///ssid=\xc2\xaf\\_(\xe3\x83\x84)_/\xc2\xaf
    ///wps_state=enabled
    ///wpa=12
    ///group_cipher=CCMP
    ///key_mgmt=WPA2-PSK
    ///wpa_state=COMPLETED
    ///rsn_pairwise_cipher=foo
    ///wpa_pairwise_cipher=bar
    ///"#;
    /// let config = Config::from_response(resp).unwrap();
    /// assert_eq!(config.wps_state, true);
    /// assert_eq!(config.wpa, 12);
    /// assert_eq!(config.ssid, r#"¯\_(ツ)_/¯"#);
    /// ```
    pub fn from_response(response: &str) -> Result<Config> {
        crate::config::deserialize_str(response).map_err(|e| error::Error::ParsingWifiConfig {
            e,
            s: response.into(),
        })
    }
}

fn deserialize_enabled_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "enabled" => Ok(true),
        "disabled" => Ok(false),
        _ => Err(de::Error::unknown_variant(&s, &["enabled", "disabled"])),
    }
}

fn deserialize_i32<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;

    match s.parse() {
        Ok(n) => Ok(n),
        _ => Err(de::Error::custom("invalid int")),
    }
}
