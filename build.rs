use std::env;
use std::path::{Path, PathBuf};

const INCLUDE_PATHS: &[&str] = &[
    "/sdk/app_modules/api",
    "/sdk/ble_stack/controller/em",
    "/sdk/ble_stack/controller/llc",
    "/sdk/ble_stack/controller/lld",
    "/sdk/ble_stack/controller/llm",
    "/sdk/ble_stack/ea/api",
    "/sdk/ble_stack/em/api",
    "/sdk/ble_stack/hci/api",
    "/sdk/ble_stack/hci/src",
    "/sdk/ble_stack/host/att",
    "/sdk/ble_stack/host/att/attc",
    "/sdk/ble_stack/host/att/attm",
    "/sdk/ble_stack/host/att/atts",
    "/sdk/ble_stack/host/gap",
    "/sdk/ble_stack/host/gap/gapc",
    "/sdk/ble_stack/host/gap/gapm",
    "/sdk/ble_stack/host/gatt",
    "/sdk/ble_stack/host/gatt/gattc",
    "/sdk/ble_stack/host/gatt/gattm",
    "/sdk/ble_stack/host/l2c/l2cc",
    "/sdk/ble_stack/host/l2c/l2cm",
    "/sdk/ble_stack/host/smp",
    "/sdk/ble_stack/host/smp/smpc",
    "/sdk/ble_stack/host/smp/smpm",
    "/sdk/ble_stack/profiles",
    "/sdk/ble_stack/profiles/custom",
    "/sdk/ble_stack/profiles/custom/custs/api",
    "/sdk/ble_stack/profiles/dis/diss/api",
    "/sdk/ble_stack/rwble_hl",
    "/sdk/ble_stack/rwble",
    "/sdk/common_project_files",
    "/sdk/platform/arch",
    "/sdk/platform/arch/boot",
    "/sdk/platform/arch/boot/GCC",
    "/sdk/platform/arch/compiler",
    "/sdk/platform/arch/compiler/GCC",
    "/sdk/platform/arch/ll",
    "/sdk/platform/arch/main",
    "/sdk/platform/core_modules/arch_console",
    "/sdk/platform/core_modules/common/api",
    "/sdk/platform/core_modules/crypto",
    "/sdk/platform/core_modules/dbg/api",
    "/sdk/platform/core_modules/gtl/api",
    "/sdk/platform/core_modules/gtl/src",
    "/sdk/platform/core_modules/h4tl/api",
    "/sdk/platform/core_modules/ke/api",
    "/sdk/platform/core_modules/ke/src",
    "/sdk/platform/core_modules/nvds/api",
    "/sdk/platform/core_modules/rf/api",
    "/sdk/platform/core_modules/rwip/api",
    "/sdk/platform/core_modules/rwip/api",
    "/sdk/platform/driver/adc",
    "/sdk/platform/driver/ble",
    "/sdk/platform/driver/dma",
    "/sdk/platform/driver/gpio",
    "/sdk/platform/driver/hw_otpc",
    "/sdk/platform/driver/i2c_eeprom",
    "/sdk/platform/driver/i2c",
    "/sdk/platform/driver/reg",
    "/sdk/platform/driver/spi_flash",
    "/sdk/platform/driver/spi",
    "/sdk/platform/driver/syscntl",
    "/sdk/platform/driver/trng",
    "/sdk/platform/driver/uart",
    "/sdk/platform/include",
    "/sdk/platform/include/CMSIS/5.6.0/Include",
    "/sdk/platform/system_library/include",
    "/sdk/platform/utilities/otp_cs",
    "/sdk/platform/utilities/otp_hdr",
    "/third_party/hash",
    "/third_party/irng",
];

const CONFIG_HEADERS: &[&str] = &[
    "da1458x_config_basic.h",
    "da1458x_config_advanced.h",
    "user_config.h",
];

const SDK_BASE_C_SOURCES: &[&str] = &[
    "/sdk/platform/arch/boot/system_DA14531.c",
    "/sdk/platform/arch/main/arch_main.c",
    "/sdk/platform/arch/main/arch_system.c",
    "/sdk/platform/arch/main/hardfault_handler.c",
    "/sdk/platform/arch/main/jump_table.c",
    "/sdk/platform/arch/main/nmi_handler.c",
    "/sdk/platform/core_modules/nvds/src/nvds.c",
    "/sdk/platform/driver/adc/adc_531.c",
    "/sdk/platform/driver/gpio/gpio.c",
    "/sdk/platform/driver/hw_otpc/hw_otpc_531.c",
    "/sdk/platform/driver/syscntl/syscntl.c",
    "/sdk/platform/driver/trng/trng.c",
    "/sdk/platform/driver/uart/uart_utils.c",
    "/sdk/platform/driver/uart/uart.c",
    "/sdk/platform/driver/spi/spi_531.c",
    "/sdk/platform/driver/spi_flash/spi_flash.c",
    "/sdk/platform/utilities/otp_cs/otp_cs.c",
    "/sdk/platform/utilities/otp_hdr/otp_hdr.c",
];

const SDK_BLE_C_SOURCES: &[&str] = &[
    "/sdk/platform/arch/main/arch_rom.c",
    "/sdk/platform/arch/main/arch_sleep.c",
    "/sdk/platform/core_modules/crypto/aes.c",
    "/sdk/platform/core_modules/crypto/aes_api.c",
    "/sdk/platform/core_modules/crypto/aes_cbc.c",
    "/sdk/platform/core_modules/crypto/sw_aes.c",
    "/sdk/platform/core_modules/crypto/aes_task.c",
    "/sdk/platform/core_modules/rf/src/ble_arp.c",
    "/sdk/platform/core_modules/rf/src/rf_531.c",
    "/sdk/platform/core_modules/rwip/src/rwip.c",
    "/sdk/app_modules/src/app_common/app_msg_utils.c",
    "/sdk/app_modules/src/app_common/app_task.c",
    "/sdk/app_modules/src/app_custs/app_customs_task.c",
    "/sdk/app_modules/src/app_default_hnd/app_default_handlers.c",
    "/sdk/app_modules/src/app_entry/app_entry_point.c",
    "/sdk/ble_stack/profiles/custom/custs/src/custs1_task.c",
    "/sdk/ble_stack/profiles/prf.c",
    "/sdk/ble_stack/rwble/rwble.c",
];

const SDK_ASM_SOURCES: &[&str] = &[
    //"/sdk/platform/arch/boot/GCC/ivtable_DA14531.S",
    //"/sdk/platform/arch/boot/GCC/startup_DA14531.S",
];

fn is_feature_enabled(feature: &str) -> bool {
    // Convert the feature name to Cargo's expected environment variable format
    let cargo_feature = format!("CARGO_FEATURE_{}", feature.to_uppercase());
    std::env::var(cargo_feature).is_ok()
}

fn generate_config_file(file_name: &str, content: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::write(out_path.join(file_name), content).unwrap();
}

fn generate_user_modules_config() {
    let excludes = [
        ("profile_custom_server1", "EXCLUDE_DLG_CUSTS1"),
        ("profile_custom_server2", "EXCLUDE_DLG_CUSTS2"),
        ("profile_dis_server", "EXCLUDE_DLG_DISS"),
        ("profile_prox_reporter", "EXCLUDE_DLG_PROXR"),
        ("profile_batt_server", "EXCLUDE_DLG_BASS"),
        ("profile_suota_receiver", "EXCLUDE_DLG_SUOTAR"),
        ("profile_findme_target", "EXCLUDE_DLG_FINDT"),
        ("profile_findme_locator", "EXCLUDE_DLG_FINDL"),
    ];

    let mut header = String::from(
        "#pragma once
#define EXCLUDE_DLG_GAP (0)
#define EXCLUDE_DLG_TIMER (0)
#define EXCLUDE_DLG_MSG (1)
#define EXCLUDE_DLG_SEC (1)\n",
    );

    for (feature, define) in &excludes {
        let value = if is_feature_enabled(feature) { 0 } else { 1 };
        header += &format!("#define {} ({})\n", define, value);
    }

    generate_config_file("user_modules_config.h", &header);
}

fn generate_user_profiles_config() {
    let profiles = [
        ("profile_custom_server1", "CFG_PRF_CUST1"),
        ("profile_custom_server2", "CFG_PRF_CUST2"),
        ("profile_dis_server", "CFG_PRF_DISS"),
        ("profile_prox_reporter", "CFG_PRF_PXPR"),
        ("profile_batt_server", "CFG_PRF_BASS"),
        ("profile_suota_receiver", "CFG_PRF_SUOTAR"),
        ("profile_findme_target", "CFG_PRF_FMPT"),
        ("profile_findme_locator", "CFG_PRF_FMPL"),
    ];

    let mut header = String::from("#pragma once\n");

    for (feature, define) in &profiles {
        if is_feature_enabled(feature) {
            header += &format!("#define {}\n", define);
        }
    }

    generate_config_file("user_profiles_config.h", &header);
}

fn generate_user_config() {
    #[cfg(all(feature = "address_mode_public", feature = "address_mode_static"))]
    compile_error!("Only one address mode feature flag can be set!");

    let address_mode = if cfg!(feature = "address_mode_public") {
        "APP_CFG_ADDR_PUB"
    } else {
        "APP_CFG_ADDR_STATIC"
    };

    #[cfg(any(
        all(feature = "sleep_mode_off", feature = "sleep_mode_ext_on"),
        all(feature = "sleep_mode_off", feature = "sleep_mode_ext_otp_copy_on"),
        all(feature = "sleep_mode_ext_on", feature = "sleep_mode_ext_otp_copy_on")
    ))]
    compile_error!("Only one sleep mode feature flag can be set!");

    let sleep_mode = if cfg!(feature = "sleep_mode_off") {
        "ARCH_SLEEP_OFF"
    } else if cfg!(feature = "sleep_mode_ext_on") {
        "ARCH_EXT_SLEEP_ON"
    } else if cfg!(feature = "sleep_mode_ext_otp_copy_on") {
        "ARCH_EXT_SLEEP_OTP_COPY_ON"
    } else {
        panic!("One sleep mode feature flag has to be set!");
    };

    let header = format!(
        "#pragma once
#include \"app_user_config.h\"
#include \"arch_api.h\"
#include \"app_default_handlers.h\"
#include \"app_adv_data.h\"
#include \"co_bt.h\"
#define USER_CFG_ADDRESS_MODE {}
#define USER_CFG_CNTL_PRIV_MODE APP_CFG_CNTL_PRIV_MODE_NETWORK
static const sleep_state_t app_default_sleep_mode = {};
extern const struct default_handlers_configuration user_default_hnd_conf;",
        address_mode, sleep_mode
    );

    generate_config_file("user_config.h", &header);
}

fn setup_build() -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<(String, Option<String>)>,
) {
    let sdk_path = env::var("SDK_PATH")
        .map(|path| Path::new(&path).to_path_buf())
        .unwrap_or(PathBuf::new().join("..").join("sdk"))
        .to_str()
        .unwrap()
        .to_string();

    #[allow(unused_mut)]
    let mut include_dirs: Vec<_> = INCLUDE_PATHS.into();

    #[allow(unused_mut)]
    let mut sdk_c_sources: Vec<_> = SDK_BASE_C_SOURCES.into();

    if cfg!(not(feature = "no_ble")) {
        sdk_c_sources.extend_from_slice(SDK_BLE_C_SOURCES);
    }

    let mut defines: Vec<(&str, Option<&str>)> = vec![("__DA14531__", None)];

    #[allow(unused_mut)]
    let mut include_files: Vec<&str> = Vec::new();

    // Enable TRNG
    defines.push(("CFG_USE_CHACHA20_RAND", None));

    #[cfg(feature = "no_ble")]
    {
        defines.push(("__NON_BLE_EXAMPLE__", None));
    }

    #[cfg(feature = "address_mode_public")]
    {
        defines.push(("USER_CFG_ADDRESS_MODE", Some("APP_CFG_ADDR_PUB")));
    }

    #[cfg(feature = "address_mode_static")]
    {
        defines.push(("USER_CFG_ADDRESS_MODE", Some("APP_CFG_ADDR_STATIC")));
    }

    #[cfg(feature = "ble_server_profiles")]
    {
        defines.push(("BLE_SERVER_PRF", None));
    }

    #[cfg(feature = "ble_client_profiles")]
    {
        defines.push(("BLE_CLIENT_PRF", None));
    }

    #[cfg(feature = "profile_gatt_client")]
    {
        defines.push(("CFG_PRF_GATTC", None));
        include_dirs.push(&translate_path(
            "/sdk/ble_stack/profiles/gatt/gatt_client/api",
        ));
        sdk_c_sources.push(&translate_path(
            "/sdk/app_modules/src/app_gattc/app_gattc.c",
        ));
    }

    #[cfg(feature = "profile_prox_reporter")]
    {
        defines.push(("CFG_PRF_PXPR", None));
        include_dirs.push(&translate_path("/sdk/ble_stack/profiles/prox/proxr/api"));
        include_files.push(&translate_path("/sdk/app_modules/api/app_proxr.h"));
    }

    #[cfg(feature = "profile_batt_server")]
    {
        defines.push(("CFG_PRF_PXPR", None));
        include_dirs.push(&translate_path("/sdk/ble_stack/profiles/prox/proxr/api"));
        include_files.push(&translate_path("/sdk/app_modules/api/app_proxr.h"));
    }

    #[cfg(feature = "profile_findme_target")]
    {
        defines.push(("CFG_PRF_FMPT", None));
        include_dirs.push(&translate_path("/sdk/ble_stack/profiles/find/findt/api"));
        include_dirs.push(&translate_path("/sdk/ble_stack/profiles/find"));
        include_files.push(&translate_path("/sdk/app_modules/api/app_findme.h"));
    }

    let mut include_dirs: Vec<_> = include_dirs
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    include_dirs.push(
        env::current_dir()
            .unwrap()
            .to_path_buf()
            .join("include")
            .to_str()
            .unwrap()
            .to_string(),
    );
    include_dirs.push(env::var("OUT_DIR").unwrap());

    #[allow(unused_mut)]
    let mut include_files: Vec<_> = include_files
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    // #[cfg(not(feature = "no_ble"))]
    // {
    let mut config_headers: Vec<_> = CONFIG_HEADERS.iter().map(|s| s.to_string()).collect();
    include_files.append(&mut config_headers);
    // }

    let sdk_c_sources: Vec<_> = sdk_c_sources
        .iter()
        .filter_map(|path| {
            if cfg!(feature = "no_main") && path.ends_with("arch_main.c") {
                return None;
            }
            if cfg!(not(feature = "driver_spi")) && path.ends_with("spi_531.c") {
                return None;
            }
            if cfg!(not(feature = "driver_spi_flash")) && path.ends_with("spi_flash.c") {
                return None;
            }
            Some(format!("{}{}", sdk_path, path))
        })
        .collect();

    let sdk_asm_sources: Vec<_> = SDK_ASM_SOURCES
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    let defines: Vec<_> = defines
        .iter()
        .map(|(key, value)| (key.to_string(), value.map(|value| value.to_string())))
        .collect();

    (
        include_dirs,
        include_files,
        sdk_c_sources,
        sdk_asm_sources,
        defines,
    )
}

fn generate_bindings(
    include_dirs: &Vec<String>,
    include_files: &Vec<String>,
    defines: &Vec<(String, Option<String>)>,
    rustify_enums: &Vec<&str>,
) {
    let bindings_header = if cfg!(feature = "no_ble") {
        "bindings_no_ble.h"
    } else {
        "bindings.h"
    };

    let mut builder = bindgen::Builder::default()
        .header(bindings_header)
        .ctypes_prefix("cty")
        .use_core()
        .size_t_is_usize(true)
        .clang_arg("-D__SOFTFP__")
        .clang_arg("-DUSER_DEVICE_NAME_LEN=0")
        .clang_arg(&translate_path("-I/Applications/ARM/arm-none-eabi/include"))
        .clang_arg(&translate_path(
            "-I/usr/lib/gcc/arm-none-eabi/12.2.1/include",
        ))
        .clang_arg(&translate_path("-I/usr/include/newlib"))
        .clang_arg("-Wno-expansion-to-defined");

    for (key, value) in defines {
        if let Some(value) = value {
            builder = builder.clang_arg(format!("-D{}={}", key, value));
        } else {
            builder = builder.clang_arg(format!("-D{}", key));
        }
    }

    for inc_dir in include_dirs {
        builder = builder.clang_arg(format!("-I{}", translate_path(inc_dir)));
    }

    for inc_file in include_files {
        builder = builder.clang_args(vec!["-include", &translate_path(inc_file)]);
    }

    for re in rustify_enums {
        builder = builder.rustified_enum(re)
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_sdk(
    include_dirs: &Vec<String>,
    include_files: &Vec<String>,
    defines: &Vec<(String, Option<String>)>,
    sdk_c_sources: &Vec<String>,
    sdk_asm_sources: &Vec<String>,
) {
    let mut cc_builder = cc::Build::new();

    let mut cc_builder = cc_builder
        .debug(true)
        .target("thumbv6m-none-eabi")
        .flag("-march=armv6-m")
        .flag("-Wno-expansion-to-defined")
        .flag("-Wno-unused-parameter")
        .flag("-fstack-usage")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .opt_level_str("z")
        .define("USER_DEVICE_NAME_LEN", Some("0"));

    for inc_dir in include_dirs {
        cc_builder = cc_builder.include(translate_path(inc_dir));
    }

    for inc_file in include_files {
        cc_builder = cc_builder.flag(&format!("-include{}", translate_path(inc_file)));
    }

    for (key, value) in defines {
        cc_builder = cc_builder.define(key, value.as_ref().map(|v| v.as_str()));
    }

    for file in sdk_c_sources {
        cc_builder = cc_builder.file(translate_path(file));
    }

    cc_builder.compile("sdk");

    // cc::Build::new()
    //     .files(sdk_asm_sources)
    //     .define("__DA14531__", None)
    //     .compile("boot");
}

fn main() {
    let (include_dirs, include_files, sdk_c_sources, sdk_asm_sources, defines) = setup_build();

    if cfg!(not(feature = "no_ble")) {
        generate_user_config();
        generate_user_modules_config();
        generate_user_profiles_config();
    }

    std::fs::write(
        "/home/harry/Development/rapitag/firmware-bundle/blinky/build/include_dirs",
        include_dirs.join("\n"),
    )
    .unwrap();
    std::fs::write(
        "/home/harry/Development/rapitag/firmware-bundle/blinky/build/include_files",
        include_files.join("\n"),
    )
    .unwrap();
    std::fs::write(
        "/home/harry/Development/rapitag/firmware-bundle/blinky/build/sdk_c_sources",
        sdk_c_sources.join("\n"),
    )
    .unwrap();
    std::fs::write(
        "/home/harry/Development/rapitag/firmware-bundle/blinky/build/defines",
        defines
            .iter()
            .map(|(s, _)| s.clone())
            .fold(String::new(), |mut acc, s| {
                acc += &s;
                acc += "\n";
                acc
            }),
    )
    .unwrap();

    generate_bindings(
        &include_dirs,
        &include_files,
        &defines,
        &vec![
            "hl_err",
            "process_event_response",
            "syscntl_dcdc_level_t",
            "APP_MSG",
        ],
    );

    compile_sdk(
        &include_dirs,
        &include_files,
        &defines,
        &sdk_c_sources,
        &sdk_asm_sources,
    );

    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=build.rs");

    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/da1458x_config_basic.h")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/da1458x_config_advanced.h")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_callback_config.h")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_custs_config.h")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_periph_setup.h")
    );
}

fn translate_path(path: &str) -> String {
    if cfg!(windows) {
        path.replace("/", "\\")
    } else {
        path.to_string()
    }
}
