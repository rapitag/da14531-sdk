/// Incomplete list of 16-bit Service IDs
#[allow(dead_code)]
pub const ADV_TYPE_INCOMPLETE_LIST_16BIT_SERVICE_IDS: u8 = 0x02;

/// Complete list of 16-bit Service IDs
#[allow(dead_code)]
pub const ADV_TYPE_COMPLETE_LIST_16BIT_SERVICE_IDS: u8 = 0x03;

/// Incomplete list of 32-bit Service IDs (not relevant for Bluetooth 4.0)
#[allow(dead_code)]
pub const ADV_TYPE_INCOMPLETE_LIST_32BIT_SERVICE_IDS: u8 = 0x04;

/// Complete list of 32-bit Service IDs (not relevant for Bluetooth 4.0)
#[allow(dead_code)]
pub const ADV_TYPE_COMPLETE_LIST_32BIT_SERVICE_IDS: u8 = 0x05;

/// Incomplete list of 128-bit Service IDs
#[allow(dead_code)]
pub const ADV_TYPE_INCOMPLETE_LIST_128BIT_SERVICE_IDS: u8 = 0x06;

/// Complete list of 128-bit Service IDs
#[allow(dead_code)]
pub const ADV_TYPE_COMPLETE_LIST_128BIT_SERVICE_IDS: u8 = 0x07;

/// Shortened Local Name
#[allow(dead_code)]
pub const ADV_TYPE_SHORTENED_LOCAL_NAME: u8 = 0x08;

/// Complete Local Name
#[allow(dead_code)]
pub const ADV_TYPE_COMPLETE_LOCAL_NAME: u8 = 0x09;

/// TX Power Level (in dBm)
#[allow(dead_code)]
pub const ADV_TYPE_TX_POWER_LEVEL: u8 = 0x0A;

/// Class of Device
#[allow(dead_code)]
pub const ADV_TYPE_CLASS_OF_DEVICE: u8 = 0x0D;

/// Simple Pairing Hash C
#[allow(dead_code)]
pub const ADV_TYPE_SIMPLE_PAIRING_HASH_C: u8 = 0x0E;

/// Simple Pairing Randomizer R
#[allow(dead_code)]
pub const ADV_TYPE_SIMPLE_PAIRING_RANDOM_R: u8 = 0x0F;

/// Device ID
#[allow(dead_code)]
pub const ADV_TYPE_DEVICE_ID: u8 = 0x10;

/// Security Manager Out of Band Flags
#[allow(dead_code)]
pub const ADV_TYPE_SECURITY_MAN_OUT_OF_BAND_FLAGS: u8 = 0x11;

/// Slave Connection Interval Range
#[allow(dead_code)]
pub const ADV_TYPE_SLAVE_CONNECTION_INTERVAL_RANGE: u8 = 0x12;

/// List of 16-bit Service Solicitation UUIDs
#[allow(dead_code)]
pub const ADV_TYPE_LIST_16BIT_SOLICITATION_UUIDS: u8 = 0x14;

/// List of 32-bit Service Solicitation UUIDs
#[allow(dead_code)]
pub const ADV_TYPE_LIST_32BIT_SOLICITATION_UUIDS: u8 = 0x1F;

/// List of 128-bit Service Solicitation UUIDs
#[allow(dead_code)]
pub const ADV_TYPE_LIST_128BIT_SOLICITATION_UUIDS: u8 = 0x15;

/// Service Data - 16-bit UUID
#[allow(dead_code)]
pub const ADV_TYPE_SERVICE_DATA_16BIT_UUID: u8 = 0x16;

/// Service Data - 32-bit UUID
#[allow(dead_code)]
pub const ADV_TYPE_SERVICE_DATA_32BIT_UUID: u8 = 0x20;

/// Service Data - 128-bit UUID
#[allow(dead_code)]
pub const ADV_TYPE_SERVICE_DATA_128BIT_UUID: u8 = 0x21;

/// LE Secure Connections Confirmation Value
#[allow(dead_code)]
pub const ADV_TYPE_LE_SECURE_CON_CONFIRM_VAL: u8 = 0x22;

/// LE Secure Connections Random Value
#[allow(dead_code)]
pub const ADV_TYPE_LE_SECURE_CON_RANDOM_VAL: u8 = 0x23;

/// Public Target Address
#[allow(dead_code)]
pub const ADV_TYPE_PUBLIC_TARGET_ADDRESS: u8 = 0x17;

/// Random Target Address
#[allow(dead_code)]
pub const ADV_TYPE_RANDOM_TARGET_ADDRESS: u8 = 0x18;

/// Appearance
#[allow(dead_code)]
pub const ADV_TYPE_APPEARANCE: u8 = 0x19;

/// Advertising Interval
#[allow(dead_code)]
pub const ADV_TYPE_ADVERTISING_INTERVAL: u8 = 0x1A;

/// LE Bluetooth Device Address
#[allow(dead_code)]
pub const ADV_TYPE_LE_BLUETOOTH_DEVICE_ADDRESS: u8 = 0x1B;

/// LE Role
#[allow(dead_code)]
pub const ADV_TYPE_LE_ROLE: u8 = 0x1C;

/// Simple Pairing Hash C-256
#[allow(dead_code)]
pub const ADV_TYPE_SIMPLE_PAIRING_HASH_C256: u8 = 0x1D;

/// Simple Pairing Randomizer R-256
#[allow(dead_code)]
pub const ADV_TYPE_SIMPLE_PAIRING_RANDOM_C256: u8 = 0x1E;

/// Uniform Resource Identifier
#[allow(dead_code)]
pub const ADV_TYPE_URI: u8 = 0x24;

/// 3D Information Data
#[allow(dead_code)]
pub const ADV_TYPE_INFO_DATA_3D: u8 = 0x3D;

/// Manufacturer Specific Data
#[allow(dead_code)]
pub const ADV_TYPE_MANUFACTURER_SPECIFIC_DATA: u8 = 0xFF;

macro_rules! user_adv_data_entry_size {
    ($byte:literal $($tail:literal)*) => {
        1 + user_adv_data_entry_size!($($tail )*)
    };
    () => {
        1
    }
}

macro_rules! configure_user_adv_data {
    ($({$adv_type: ident, $($byte:literal),*}),*) => {
        pub static mut USER_ADVERTISE_DATA: [u8; 0 $(
            + user_adv_data_entry_size!($($byte )*) + 1
        )*] = [
            $(
                user_adv_data_entry_size!($($byte )*),
                $adv_type,
                $(
                    $byte,
                )*
            )*
        ];
    };
}

macro_rules! configure_user_scan_response_data {
    ($({$adv_type: ident, $($byte:literal),*}),*) => {
        const USER_ADVERTISE_SCAN_RESPONSE_DATA: [u8; 0 $(
            + user_adv_data_entry_size!($($byte )*) + 1
        )*] = [
            $(
                user_adv_data_entry_size!($($byte )*),
                $adv_type,
                $(
                    $byte,
                )*
            )*
        ];
    };
}

pub(crate) use configure_user_adv_data;
pub(crate) use configure_user_scan_response_data;
pub(crate) use user_adv_data_entry_size;
