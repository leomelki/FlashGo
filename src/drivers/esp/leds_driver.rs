// Un-stdified from https://github.com/cat-in-136/ws2812-esp32-rmt-driver/blob/ae9f8b72e77e05cf86bb2c2a9634b8db28e4924c/src/driver/esp32_rmt.rs

use anyhow::Result;
use core::time::Duration;
use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::rmt::config::TransmitConfig;
use esp_idf_svc::hal::rmt::{PinState, Pulse, RmtChannel, Symbol, TxRmtDriver};
use esp_idf_svc::hal::units::Hertz;

/// T0H duration time (0 code, high voltage time)
const WS2812_T0H_NS: Duration = Duration::from_nanos(400);
/// T0L duration time (0 code, low voltage time)
const WS2812_T0L_NS: Duration = Duration::from_nanos(850);
/// T1H duration time (1 code, high voltage time)
const WS2812_T1H_NS: Duration = Duration::from_nanos(800);
/// T1L duration time (1 code, low voltage time)
const WS2812_T1L_NS: Duration = Duration::from_nanos(450);

/// Converter to a sequence of RMT items.
#[repr(C)]
struct Ws2812Esp32RmtItemEncoder {
    /// The RMT item that represents a 0 code.
    bit0: Symbol,
    /// The RMT item that represents a 1 code.
    bit1: Symbol,
}

impl Ws2812Esp32RmtItemEncoder {
    /// Creates a new encoder with the given clock frequency.
    ///
    /// # Arguments
    ///
    /// * `clock_hz` - The clock frequency.
    ///
    /// # Errors
    ///
    /// Returns an error if the clock frequency is invalid or if the RMT item encoder cannot be created.
    fn new(clock_hz: Hertz) -> Result<Self> {
        let (bit0, bit1) = (
            Symbol::new(
                Pulse::new_with_duration(clock_hz, PinState::High, &WS2812_T0H_NS)?,
                Pulse::new_with_duration(clock_hz, PinState::Low, &WS2812_T0L_NS)?,
            ),
            Symbol::new(
                Pulse::new_with_duration(clock_hz, PinState::High, &WS2812_T1H_NS)?,
                Pulse::new_with_duration(clock_hz, PinState::Low, &WS2812_T1L_NS)?,
            ),
        );

        Ok(Self { bit0, bit1 })
    }

    /// Encodes a block of data as a sequence of RMT items.
    ///
    /// # Arguments
    ///
    /// * `src` - The block of data to encode.
    ///
    /// # Returns
    ///
    /// An iterator over the RMT items that represent the encoded data.
    #[inline]
    fn encode_iter<'a, 'b, T>(&'a self, src: T) -> impl Iterator<Item = Symbol> + Send + 'a
    where
        'b: 'a,
        T: Iterator<Item = u8> + Send + 'b,
    {
        src.flat_map(move |v| {
            (0..(u8::BITS as usize)).map(move |i| {
                if v & (1 << (7 - i)) != 0 {
                    self.bit1
                } else {
                    self.bit0
                }
            })
        })
    }
}

/// WS2812 ESP32 RMT driver wrapper.
pub struct Ws2812Esp32RmtDriver<'d> {
    /// TxRMT driver.
    tx: TxRmtDriver<'d>,
    /// `u8`-to-`rmt_item32_t` Encoder
    encoder: Ws2812Esp32RmtItemEncoder,
}

impl<'d> Ws2812Esp32RmtDriver<'d> {
    /// Creates a WS2812 ESP32 RMT driver wrapper.
    ///
    /// RMT driver of `channel` shall be initialized and installed for `pin`.
    /// `channel` shall be different between different `pin`.
    ///
    /// # Errors
    ///
    /// Returns an error if the RMT driver initialization failed.
    pub fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'd,
        pin: impl Peripheral<P = impl OutputPin> + 'd,
    ) -> Result<Self> {
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;

        let clock_hz = tx.counter_clock()?;
        let encoder = Ws2812Esp32RmtItemEncoder::new(clock_hz)?;

        Ok(Self { tx, encoder })
    }

    /// Writes pixel data from a pixel-byte sequence to the IO pin.
    ///
    /// Byte count per LED pixel and channel order is not handled by this method.
    /// The pixel data sequence has to be correctly laid out depending on the LED strip model.
    ///
    /// # Errors
    ///
    /// Returns an error if an RMT driver error occurred.
    ///
    /// # Warning
    ///
    /// Iteration of `pixel_sequence` happens inside an interrupt handler so beware of side-effects
    /// that don't work in interrupt handlers.
    /// See [esp_idf_hal::rmt::TxRmtDriver#start_iter_blocking()] for details.
    pub fn write_blocking<'a, 'b, T>(&'a mut self, pixel_sequence: T) -> Result<()>
    where
        'b: 'a,
        T: Iterator<Item = u8> + Send + 'b,
    {
        let signal = self.encoder.encode_iter(pixel_sequence);
        self.tx.start_iter_blocking(signal)?;
        Ok(())
    }

    /// Writes pixel data from a pixel-byte sequence to the IO pin.
    ///
    /// Byte count per LED pixel and channel order is not handled by this method.
    /// The pixel data sequence has to be correctly laid out depending on the LED strip model.
    ///
    /// Note that this requires `pixel_sequence` to be [`Box`]ed for an allocation free version see [`Self::write_blocking`].
    ///
    /// # Errors
    ///
    /// Returns an error if an RMT driver error occurred.
    ///
    /// # Warning
    ///
    /// Iteration of `pixel_sequence` happens inside an interrupt handler so beware of side-effects
    /// that don't work in interrupt handlers.
    /// See [esp_idf_hal::rmt::TxRmtDriver#start_iter()] for details.
    pub fn write<T>(&'static mut self, pixel_sequence: T) -> Result<()>
    where
        T: Iterator<Item = u8> + Send + 'static,
    {
        let signal = self.encoder.encode_iter(pixel_sequence);
        self.tx.start_iter(signal)?;
        Ok(())
    }
}
