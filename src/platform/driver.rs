pub mod syscntl {
    pub use crate::bindings::{
        syscntl_dcdc_level_t as SyscntlDcdcLevel, syscntl_dcdc_turn_on_in_boost,
    };

    // ToDo: Port to HAL crate
    #[inline]
    pub fn dcdc_turn_on_in_boost(dcdc_level: SyscntlDcdcLevel) {
        unsafe {
            syscntl_dcdc_turn_on_in_boost(dcdc_level);
        }
    }
}

// ToDo: Port to HAL crate
pub mod gpio {
    #[inline]
    pub fn GPIO_EnableIRQ(pin: u8, irq: u8, low_input: bool, release_wait: bool, debounce: u8) {
        unsafe {
            crate::bindings::GPIO_EnableIRQ(
                0,
                pin as u32,
                irq as i32,
                low_input,
                release_wait,
                debounce,
            );
        }
    }

    #[inline]
    pub fn GPIO_RegisterCallback(irq: u8, callback: unsafe extern "C" fn()) {
        unsafe { crate::bindings::GPIO_RegisterCallback(irq as i32, Some(callback)) }
    }

    #[inline]
    pub fn GPIO_ConfigurePin(pin: u32, mode: u32, function: u32, high: bool) {
        unsafe { crate::bindings::GPIO_ConfigurePin(0, pin, mode, function, high) }
    }
}

#[cfg(feature = "driver_spi")]
pub mod spi {
    pub use crate::bindings::{spi_cfg_t as SpiConfig, SPI_Pad_t as SpiPad};

    /// Defines the SPI master/slave mode.
    #[repr(u8)]
    pub enum MsModeCfg {
        /// SPI Master mode
        Master = 0,
    }

    /// Defines the SPI mode (CPOL, CPHA).
    #[repr(u8)]
    pub enum CpModeCfg {
        /// New data on falling, capture on rising, Clk low in idle state.
        Mode0 = 0,

        /// New data on rising, capture on falling, Clk low in idle state.
        Mode1 = 1,

        /// New data on rising, capture on falling, Clk high in idle state.
        Mode2 = 2,

        /// New data on falling, capture on rising, Clk high in idle state.
        Mode3 = 3,
    }

    /// Defines the SPI speed mode.
    #[repr(u16)]
    pub enum SpeedModeCfg {
        /// SPI master clock frequency 2MHz.
        Speed2MHz = 2000,

        /// SPI master clock frequency 4MHz.
        Speed4MHz = 4000,

        /// SPI master clock frequency 8MHz.
        Speed8MHz = 8000,

        /// SPI master clock frequency 16MHz.
        Speed16MHz = 16000,

        /// SPI master clock frequency 32MHz.
        Speed32MHz = 32000,
    }

    /// Word Size Configuration
    #[repr(u8)]
    pub enum WszModeCfg {
        /// Word Size 8 bits
        Mode8Bit = 0,

        /// Word Size 16 bits
        Mode16Bit = 1,

        /// Word Size 32 bits
        Mode32Bit = 2,
    }

    /// Control the CS output in master mode.
    #[repr(u8)]
    pub enum CsModeCfg {
        /// None slave device selected.
        None = 0,

        /// Selected slave device connected to GPIO with FUNC_MODE = SPI_CS0.
        Cs0 = 1,

        /// Selected slave device connected to GPIO with FUNC_MODE = SPI_CS1.
        Cs1 = 2,

        /// Selected slave device connected to GPIO with FUNC_MODE = GPIO.
        CsGpio = 4,
    }

    /// Define the SPI IRQ masked/enabled mode.
    #[repr(u8)]
    pub enum IrqCfg {
        /// IRQ is masked.
        Disabled = 0,

        /// IRQ is enabled.
        Enabled = 1,
    }

    /// Define the SPI master edge capture type.
    #[repr(u8)]
    pub enum MasterEdgeCaptureCfg {
        /// SPI master captures data at current clock edge
        Capture = 0,

        /// SPI master captures data at next clock edge. (only for high clock configurations)
        CaptureNext = 1,
    }

    pub struct SpiConfigBuilder {
        raw: [u8; 8],
    }

    impl SpiConfigBuilder {
        pub fn new() -> Self {
            Self {
                raw: [0xa0, 0x8b, 0x40, 0x1f, 0x88, 0x0, 0x1, 0x7],
            }
        }

        pub fn new_from_raw(raw: [u8; 8]) -> Self {
            Self { raw }
        }

        pub fn build(self) -> SpiConfig {
            let cfg = unsafe { *(self.raw.as_ptr() as *const SpiConfig) };

            cfg
        }

        pub fn set_port(mut self, port: u8) -> Self {
            self.raw[5] = port;
            self
        }

        pub fn set_pin(mut self, pin: u8) -> Self {
            self.raw[6] = pin;
            self
        }

        pub fn set_ms_mode(mut self, mode: MsModeCfg) -> Self {
            self.raw[0] |= (mode as u8) & 0b1;
            self
        }

        pub fn set_spi_mode(mut self, mode: CpModeCfg) -> Self {
            self.raw[0] |= (((mode as u8) & 0b111) << 1) as u8;
            self
        }

        pub fn set_speed_mode(mut self, mode: SpeedModeCfg) -> Self {
            let mode = mode as u16;
            self.raw[2] = (mode & 0xff) as u8;
            self.raw[3] = ((mode >> 8) & 0xff) as u8;
            self
        }

        pub fn set_wsz_mode(mut self, mode: WszModeCfg) -> Self {
            self.raw[4] |= ((mode as u8) & 0b111) as u8;
            self
        }

        pub fn set_cs_mode(mut self, mode: CsModeCfg) -> Self {
            self.raw[4] |= (((mode as u8) & 0b111) << 3) as u8;
            self
        }

        pub fn set_irq_enabled(mut self, mode: IrqCfg) -> Self {
            self.raw[4] |= (((mode as u8) as u8) << 6) as u8;
            self
        }

        pub fn set_spi_capture(mut self, mode: MasterEdgeCaptureCfg) -> Self {
            self.raw[7] |= ((mode as u8) & 0b1) as u8;
            self
        }
    }

    #[inline]
    pub fn spi_initialize(spi_cfg: &SpiConfig) -> i8 {
        unsafe { crate::bindings::spi_initialize(spi_cfg) }
    }

    #[inline]
    pub fn spi_send(data: &[u8]) {
        unsafe {
            crate::bindings::spi_send(data.as_ptr() as *const _, data.len() as u16, 0);
        }
    }
}

#[cfg(feature = "driver_spi_flash")]
pub mod spi_flash {
    pub use crate::bindings::{
        spi_flash_cfg_t as SpiFlashConfig, spi_flash_op_t as SpiFlashOp,
        spi_flash_op_t_SPI_FLASH_OP_SE as SPI_FLASH_OP_SE,
    };

    #[derive(Debug)]
    pub enum SpiFlashError {
        Timeout,
        NotErased,
        Protected,
        Invalid,
        Align,
        FlashVendor,
        FlashType,
        ProgError,
        ReadError,
        NotDetected,
        AutodetectError,
        WelError,
        EraseError,
        Busy,
    }

    pub type SpiFlashResult<T = ()> = Result<T, SpiFlashError>;

    fn ret_to_result<T>(ret: i8, value: T) -> SpiFlashResult<T> {
        match ret {
            0 => Ok(value),
            -1 => Err(SpiFlashError::Timeout),
            -2 => Err(SpiFlashError::NotErased),
            -3 => Err(SpiFlashError::Protected),
            -4 => Err(SpiFlashError::Invalid),
            -5 => Err(SpiFlashError::Align),
            -6 => Err(SpiFlashError::FlashVendor),
            -7 => Err(SpiFlashError::FlashType),
            -8 => Err(SpiFlashError::ProgError),
            -9 => Err(SpiFlashError::ReadError),
            -10 => Err(SpiFlashError::NotDetected),
            -11 => Err(SpiFlashError::AutodetectError),
            -12 => Err(SpiFlashError::WelError),
            -13 => Err(SpiFlashError::EraseError),
            -14 => Err(SpiFlashError::Busy),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn spi_flash_configure_env(spi_flash_cfg: &SpiFlashConfig) {
        unsafe { crate::bindings::spi_flash_configure_env(spi_flash_cfg) }
    }

    #[inline]
    pub fn spi_flash_auto_detect() -> SpiFlashResult<u8> {
        let mut dev_id = 0;
        ret_to_result(
            unsafe { crate::bindings::spi_flash_auto_detect(&mut dev_id) },
            dev_id,
        )
    }

    #[inline]
    pub fn spi_flash_block_erase(address: u32, erase_op: SpiFlashOp) -> SpiFlashResult {
        ret_to_result(
            unsafe { crate::bindings::spi_flash_block_erase(address, erase_op) },
            (),
        )
    }

    #[inline]
    pub fn spi_flash_page_erase(address: u32) -> SpiFlashResult {
        ret_to_result(
            unsafe { crate::bindings::spi_flash_page_erase(address) },
            (),
        )
    }

    #[inline]
    pub fn spi_flash_chip_erase() -> SpiFlashResult {
        ret_to_result(unsafe { crate::bindings::spi_flash_chip_erase() }, ())
    }

    #[inline]
    pub fn spi_flash_write_data(data: &[u8], address: u32) -> SpiFlashResult<usize> {
        let mut bytes_written = 0;
        ret_to_result(
            unsafe {
                crate::bindings::spi_flash_write_data(
                    data.as_ptr() as *mut _,
                    address,
                    data.len() as u32,
                    &mut bytes_written,
                )
            },
            bytes_written as usize,
        )
    }

    #[inline]
    pub fn spi_flash_read_data(data: &mut [u8], address: u32) -> SpiFlashResult<usize> {
        let mut bytes_read = 0;
        ret_to_result(
            unsafe {
                crate::bindings::spi_flash_read_data(
                    data.as_mut_ptr(),
                    address,
                    data.len() as u32,
                    &mut bytes_read,
                )
            },
            bytes_read as usize,
        )
    }

    #[inline]
    pub fn spi_flash_read_data_buffer(data: &mut [u8], address: u32) -> SpiFlashResult<usize> {
        let mut bytes_read = 0;
        ret_to_result(
            unsafe {
                crate::bindings::spi_flash_read_data_buffer(
                    data.as_mut_ptr(),
                    address,
                    data.len() as u32,
                    &mut bytes_read,
                )
            },
            bytes_read as usize,
        )
    }

    #[inline]
    pub fn spi_flash_enable_with_autodetect(spi_cfg: &super::spi::SpiConfig) -> SpiFlashResult<u8> {
        let mut dev_id = 0;
        ret_to_result(
            unsafe { crate::bindings::spi_flash_enable_with_autodetect(spi_cfg, &mut dev_id) },
            dev_id,
        )
    }

    #[inline]
    pub fn spi_flash_read_jedec_id() -> SpiFlashResult<u32> {
        let mut jedec_id = 0;
        ret_to_result(
            unsafe { crate::bindings::spi_flash_read_jedec_id(&mut jedec_id) },
            jedec_id,
        )
    }

    #[inline]
    pub fn spi_flash_release_from_power_down() -> SpiFlashResult {
        ret_to_result(
            unsafe { crate::bindings::spi_flash_release_from_power_down() },
            (),
        )
    }

    #[inline]
    pub fn spi_flash_power_down() -> SpiFlashResult {
        ret_to_result(unsafe { crate::bindings::spi_flash_power_down() }, ())
    }

    #[inline]
    pub fn spi_flash_enable(
        spi_cfg: &super::spi::SpiConfig,
        spi_flash_cfg: &SpiFlashConfig,
    ) -> SpiFlashResult {
        ret_to_result(
            unsafe { crate::bindings::spi_flash_enable(spi_cfg, spi_flash_cfg) },
            (),
        )
    }

    #[inline]
    pub fn spi_flash_wait_till_ready() -> SpiFlashResult {
        ret_to_result(unsafe { crate::bindings::spi_flash_wait_till_ready() }, ())
    }
}
