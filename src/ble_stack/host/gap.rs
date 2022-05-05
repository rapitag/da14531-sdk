pub use crate::bindings::{
    gap_ad_type as GapAdType, gap_ad_type_GAP_AD_TYPE_COMPLETE_NAME as GAP_AD_TYPE_COMPLETE_NAME,
    gap_ad_type_GAP_AD_TYPE_MANU_SPECIFIC_DATA as GAP_AD_TYPE_MANU_SPECIFIC_DATA,
    gap_adv_mode_GAP_GEN_DISCOVERABLE as GAP_GEN_DISCOVERABLE,
    gap_adv_mode_GAP_NON_DISCOVERABLE as GAP_NON_DISCOVERABLE, gap_bdaddr as GapBDAddr,
    gap_role_GAP_ROLE_PERIPHERAL as GAP_ROLE_PERIPHERAL, GAP_INVALID_CONIDX, GAP_MAX_NAME_SIZE,
};

pub mod gapc;
pub mod gapm;
