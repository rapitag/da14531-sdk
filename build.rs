use std::env;
use std::path::PathBuf;

use lazy_static::lazy_static;

const SDK_PATH: &str = "/home/harry/Development/rapitag/dialog/6.0.16.1144";

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
    static ref CONFIG_INCLUDES: Vec<&'static str> = vec![
        "config/da1458x_config_basic.h",
        "config/da1458x_config_advanced.h",
        "config/user_config.h"
    ];
}

fn generate_bindings(header: &str, rustify_enums: Option<Vec<&str>>) {
    let mut builder = bindgen::Builder::default()
        .header(format!("{}/{}", SDK_PATH, header))
        .clang_arg("-D__DA14531__")
        .ctypes_prefix("cty")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-Iconfig")
        .clang_arg("-I/usr/lib/newlib-nano/arm-none-eabi/include")
        .clang_arg("-Wno-expansion-to-defined");

    for inc_path in INCLUDE_PATHS.iter() {
        builder = builder.clang_arg(format!("-I{}/{}", SDK_PATH, inc_path));
    }

    for config_include in CONFIG_INCLUDES.iter() {
        builder = builder.clang_args(vec!["-include", config_include]);
    }

    if let Some(rustify_enums) = rustify_enums {
        for re in rustify_enums {
            builder = builder.rustified_enum(re)
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let header_path = PathBuf::from(header);
    let binding = format!("{}.rs", header_path.file_name().unwrap().to_str().unwrap());
    bindings
        .write_to_file(out_path.join(binding))
        .expect("Couldn't write bindings!");
}

fn main() {
    generate_bindings(
        "sdk/platform/driver/syscntl/syscntl.h",
        Some(vec!["syscntl_dcdc_level_t"]),
    );
    generate_bindings("sdk/platform/system_library/include/system_library.h", None);
    generate_bindings("sdk/platform/core_modules/ke/api/ke_msg.h", None);
    generate_bindings(
        "sdk/ble_stack/profiles/custom/custs/api/custs1_task.h",
        None,
    );

    println!("cargo:rerun-if-changed=build.rs");
}
