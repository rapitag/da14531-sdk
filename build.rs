use std::env;
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref INCLUDE_PATHS: Vec<&'static str> = vec![
        "sdk/app_modules/api",
        "sdk/ble_stack/controller/em",
        "sdk/ble_stack/controller/llc",
        "sdk/ble_stack/controller/lld",
        "sdk/ble_stack/controller/llm",
        "sdk/ble_stack/ea/api",
        "sdk/ble_stack/em/api",
        "sdk/ble_stack/hci/api",
        "sdk/ble_stack/hci/src",
        "sdk/ble_stack/host/att",
        "sdk/ble_stack/host/att/attc",
        "sdk/ble_stack/host/att/attm",
        "sdk/ble_stack/host/att/atts",
        "sdk/ble_stack/host/gap",
        "sdk/ble_stack/host/gap/gapc",
        "sdk/ble_stack/host/gap/gapm",
        "sdk/ble_stack/host/gatt",
        "sdk/ble_stack/host/gatt/gattc",
        "sdk/ble_stack/host/gatt/gattm",
        "sdk/ble_stack/host/l2c/l2cc",
        "sdk/ble_stack/host/l2c/l2cm",
        "sdk/ble_stack/host/smp",
        "sdk/ble_stack/host/smp/smpc",
        "sdk/ble_stack/host/smp/smpm",
        "sdk/ble_stack/profiles",
        "sdk/ble_stack/profiles/custom",
        "sdk/ble_stack/profiles/custom/custs/api",
        "sdk/ble_stack/profiles/dis/diss/api",
        "sdk/ble_stack/rwble_hl",
        "sdk/ble_stack/rwble",
        "sdk/common_project_files",
        "sdk/platform/arch",
        "sdk/platform/arch/boot",
        "sdk/platform/arch/boot/GCC",
        "sdk/platform/arch/compiler",
        "sdk/platform/arch/compiler/GCC",
        "sdk/platform/arch/ll",
        "sdk/platform/arch/main",
        "sdk/platform/core_modules/arch_console",
        "sdk/platform/core_modules/common/api",
        "sdk/platform/core_modules/dbg/api",
        "sdk/platform/core_modules/gtl/api",
        "sdk/platform/core_modules/gtl/src",
        "sdk/platform/core_modules/h4tl/api",
        "sdk/platform/core_modules/ke/api",
        "sdk/platform/core_modules/ke/src",
        "sdk/platform/core_modules/nvds/api",
        "sdk/platform/core_modules/rf/api",
        "sdk/platform/core_modules/rwip/api",
        "sdk/platform/core_modules/rwip/api",
        "sdk/platform/driver/adc",
        "sdk/platform/driver/ble",
        "sdk/platform/driver/dma",
        "sdk/platform/driver/gpio",
        "sdk/platform/driver/hw_otpc",
        "sdk/platform/driver/i2c_eeprom",
        "sdk/platform/driver/i2c",
        "sdk/platform/driver/reg",
        "sdk/platform/driver/spi_flash",
        "sdk/platform/driver/spi",
        "sdk/platform/driver/syscntl",
        "sdk/platform/driver/trng",
        "sdk/platform/driver/uart",
        "sdk/platform/include",
        "sdk/platform/include/CMSIS/5.6.0/Include",
        "sdk/platform/system_library/include",
        "sdk/platform/utilities/otp_cs",
        "sdk/platform/utilities/otp_hdr",
    ];
}

#[derive(Debug)]
struct ParseCallbacksImpl;

fn generate_bindings(rustify_enums: Vec<&str>) {
    let sdk_path = env::var("SDK_PATH").expect("SDK_PATH not set!");
    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH not set!");

    let mut builder = bindgen::Builder::default()
        .header("bindings.h")
        .clang_arg("-D__DA14531__")
        .ctypes_prefix("cty")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}", config_path))
        .clang_arg("-I/usr/lib/newlib-nano/arm-none-eabi/include")
        .clang_arg("-Wno-expansion-to-defined");

    for inc_path in INCLUDE_PATHS.iter() {
        builder = builder.clang_arg(format!("-I{}/{}", sdk_path, inc_path));
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

fn main() {
    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH not set!");

    generate_bindings(vec![
        "syscntl_dcdc_level_t",
        "hl_err",
        "gapc_msg_id",
        "gap_ad_type",
        "KE_API_ID"
    ]);

    println!("cargo:rerun-if-changed={}/da1458x_config_basic.h", config_path);
    println!("cargo:rerun-if-changed={}/da1458x_config_advanced.h", config_path);
    println!("cargo:rerun-if-changed={}/user_config.h", config_path);
    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=build.rs");
}
