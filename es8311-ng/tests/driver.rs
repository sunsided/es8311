//! Driver tests against the behavioral model.

use core::cell::RefCell;
use es8311_ng::registers::{DacControl, RegisterAddress as Reg};
use es8311_ng::{Config, Error, Es8311, RESET_DELAY_MS, SerialFormat, WordLength};
use es8311_sim::{Es8311Model, FakeI2c};

/// Records the total requested delay.
#[derive(Default)]
struct Delay(u64);

impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.0 += u64::from(ns);
    }
}

impl embedded_hal_async::delay::DelayNs for Delay {
    async fn delay_ns(&mut self, ns: u32) {
        self.0 += u64::from(ns);
    }
}

fn init(model: &RefCell<Es8311Model>) -> Es8311<FakeI2c<'_>> {
    let mut codec = Es8311::new(FakeI2c::new(model));
    codec
        .init(&mut Delay::default(), &Config::new(48_000))
        .unwrap();
    codec
}

#[test]
fn identifies_the_chip() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = Es8311::new(FakeI2c::new(&model));
    assert_eq!(codec.chip_id().unwrap(), es8311_ng::ES8311_CHIP_ID);
    assert_eq!(codec.chip_version().unwrap(), 0x00);
    assert!(codec.identify().unwrap());
}

#[test]
fn init_is_silent() {
    let model = RefCell::new(Es8311Model::new());
    let mut delay = Delay::default();
    let mut codec = Es8311::new(FakeI2c::new(&model));
    codec.init(&mut delay, &Config::new(48_000)).unwrap();

    assert!(delay.0 >= u64::from(RESET_DELAY_MS) * 1_000_000);
    let model = model.borrow();
    assert!(!model.dac_output_enabled());
    assert!(!model.loopback_enabled());
    assert_eq!(model.register(Reg::RESET), 0x80);
    assert_eq!(model.register(Reg::DAC_31), 0x60);
    assert_eq!(model.register(Reg::DAC_32), 0x00);
    assert_eq!(model.register(Reg::SYSTEM_0D), 0xFC);
    assert_eq!(model.register(Reg::SYSTEM_12), 0x02);
    assert_eq!(model.register(Reg::SDP_IN), 0x0C);
    assert_eq!(model.register(Reg::CLK_MANAGER_01), 0x3F);
}

/// Fails the test if the DAC output is enabled after any bus transfer.
struct SilentBus<'a> {
    bus: FakeI2c<'a>,
    model: &'a RefCell<Es8311Model>,
}

impl embedded_hal::i2c::ErrorType for SilentBus<'_> {
    type Error = embedded_hal::i2c::ErrorKind;
}

impl embedded_hal::i2c::I2c for SilentBus<'_> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus.transaction(address, operations)?;
        assert!(!self.model.borrow().dac_output_enabled());
        Ok(())
    }
}

#[test]
fn output_stays_off_during_init_and_standby() {
    let model = RefCell::new(Es8311Model::new());
    let bus = SilentBus {
        bus: FakeI2c::new(&model),
        model: &model,
    };
    let mut codec = Es8311::new(bus);
    codec
        .init(&mut Delay::default(), &Config::new(48_000))
        .unwrap();
    codec.dac_power_up().unwrap();
    codec.standby().unwrap();
    codec
        .init(&mut Delay::default(), &Config::new(48_000))
        .unwrap();
}

#[test]
fn playback_path() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = init(&model);

    codec.dac_power_up().unwrap();
    assert!(!model.borrow().dac_output_enabled(), "still muted");

    codec.set_mute(false).unwrap();
    assert!(model.borrow().dac_output_enabled());

    codec.set_mute(true).unwrap();
    assert!(!model.borrow().dac_output_enabled());

    codec.set_mute(false).unwrap();
    codec.dac_power_down().unwrap();
    assert!(!model.borrow().dac_output_enabled());
    assert_eq!(model.borrow().register(Reg::DAC_31), 0x60);
    assert_eq!(model.borrow().register(Reg::SYSTEM_0D), 0xFC);
}

#[test]
fn volume_never_exceeds_the_limit() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = init(&model);
    assert_eq!(codec.max_volume(), es8311_ng::DAC_VOLUME_MAX);

    codec.set_volume(0xFF).unwrap();
    assert_eq!(codec.volume().unwrap(), 0xFF);

    // Lowering the limit pulls the current volume down with it.
    codec.set_max_volume(0xA9).unwrap();
    assert_eq!(codec.volume().unwrap(), 0xA9);

    codec.set_volume(0xFF).unwrap();
    assert_eq!(model.borrow().register(Reg::DAC_32), 0xA9);

    codec.set_volume(0x80).unwrap();
    assert_eq!(codec.volume().unwrap(), 0x80);

    codec.set_max_volume(0xB0).unwrap();
    assert_eq!(codec.volume().unwrap(), 0x80);
}

#[test]
fn unsupported_clock_fails_before_bus_access() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = Es8311::new(FakeI2c::new(&model));
    let config = Config::new(48_000).with_mclk(12_345_678);
    assert_eq!(
        codec.init(&mut Delay::default(), &config),
        Err(Error::UnsupportedClock)
    );
    assert_eq!(model.borrow().register(Reg::GPIO_44), 0x00);
}

#[test]
fn other_clocks_and_formats() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = Es8311::new(FakeI2c::new(&model));
    let config = Config::new(16_000)
        .with_mclk(12_288_000)
        .with_format(SerialFormat::LeftJustified)
        .with_word_length(WordLength::Bits24);
    codec.init(&mut Delay::default(), &config).unwrap();

    let model = model.borrow();
    assert_eq!(model.register(Reg::CLK_MANAGER_02), 0x40);
    assert_eq!(model.register(Reg::CLK_MANAGER_04), 0x20);
    assert_eq!(model.register(Reg::SDP_IN), 0x01);
    assert_eq!(model.register(Reg::SDP_OUT), 0x41);
}

#[test]
fn standby_and_wake() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = init(&model);
    codec.set_volume(0xBF).unwrap();
    codec.dac_power_up().unwrap();
    codec.set_mute(false).unwrap();

    codec.standby().unwrap();
    {
        let model = model.borrow();
        assert!(!model.dac_output_enabled());
        assert_eq!(model.register(Reg::RESET), 0x1F);
        assert_eq!(model.register(Reg::CLK_MANAGER_01), 0x00);
        assert_eq!(model.register(Reg::SYSTEM_0D), 0xFC);
        assert_eq!(model.register(Reg::SYSTEM_0E), 0x7A);
        assert_eq!(model.register(Reg::DAC_32), 0x00);
    }

    codec
        .init(&mut Delay::default(), &Config::new(48_000))
        .unwrap();
    assert_eq!(model.borrow().register(Reg::RESET), 0x80);
    assert!(!model.borrow().dac_output_enabled());
}

#[test]
fn init_disables_loopback() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = Es8311::new(FakeI2c::new(&model));
    codec
        .write_register(es8311_ng::registers::GpioControl::from(0xD8))
        .unwrap();
    assert!(model.borrow().loopback_enabled());

    codec
        .init(&mut Delay::default(), &Config::new(48_000))
        .unwrap();
    assert!(!model.borrow().loopback_enabled());
}

#[test]
fn modify_register_preserves_other_bits() {
    let model = RefCell::new(Es8311Model::new());
    let mut codec = init(&model);
    codec
        .modify_register(|r: DacControl| r.with_dac_inv(true))
        .unwrap();
    codec.set_mute(false).unwrap();
    assert_eq!(model.borrow().register(Reg::DAC_31), 0x10);
}

#[test]
fn alternate_address() {
    let model = RefCell::new(Es8311Model::with_ce(true));
    let mut codec = Es8311::with_address(FakeI2c::new(&model), es8311_ng::ALTERNATE_DEVICE_ADDRESS);
    assert!(codec.identify().unwrap());

    let mut codec = Es8311::new(codec.release());
    assert!(matches!(
        codec.init(&mut Delay::default(), &Config::new(48_000)),
        Err(Error::I2c(_))
    ));
}

#[cfg(feature = "async")]
#[test]
fn async_driver() {
    use embassy_futures::block_on;
    let model = RefCell::new(Es8311Model::new());
    let mut codec = es8311_ng::asynch::Es8311::new(FakeI2c::new(&model));
    block_on(async {
        assert!(codec.identify().await.unwrap());
        codec
            .init(&mut Delay::default(), &Config::new(48_000))
            .await
            .unwrap();
        codec.set_max_volume(0xA9).await.unwrap();
        codec.set_volume(0xFF).await.unwrap();
        assert_eq!(codec.volume().await.unwrap(), 0xA9);
        codec.dac_power_up().await.unwrap();
        codec.set_mute(false).await.unwrap();
        assert!(model.borrow().dac_output_enabled());
        codec.standby().await.unwrap();
        assert!(!model.borrow().dac_output_enabled());
    });
}
