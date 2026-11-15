//! [`fake_bus`] adapter for the [`Es8311Model`].

use crate::Es8311Model;
use core::cell::RefCell;
use fake_bus::I2cTarget;

/// An I²C bus with a single [`Es8311Model`] attached.
///
/// The model is shared through a [`RefCell`] so that tests can inspect it while a driver
/// owns the bus. For a bus shared with other models, use [`fake_bus::FakeI2c`] with an
/// array of targets.
pub type FakeI2c<'a> = fake_bus::FakeI2c<&'a RefCell<Es8311Model>>;

impl I2cTarget for Es8311Model {
    fn address(&self) -> u8 {
        Es8311Model::address(self)
    }

    fn start(&mut self, is_read: bool) -> bool {
        self.i2c_start(is_read)
    }

    fn write(&mut self, byte: u8) -> bool {
        self.i2c_write(byte)
    }

    fn read(&mut self) -> u8 {
        self.i2c_read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake_bus::embedded_hal::i2c::{ErrorKind, I2c, NoAcknowledgeSource};

    #[test]
    fn write_read() {
        let model = RefCell::new(Es8311Model::new());
        let mut bus = FakeI2c::new(&model);
        let mut id = [0; 2];
        bus.write_read(0x18, &[0xFD], &mut id).unwrap();
        assert_eq!(id, [0x83, 0x11]);
    }

    #[test]
    fn wrong_address_nacks() {
        let model = RefCell::new(Es8311Model::new());
        let mut bus = FakeI2c::new(&model);
        assert_eq!(
            bus.write(0x19, &[0xFD]),
            Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address))
        );
    }

    #[test]
    fn async_write_read() {
        let model = RefCell::new(Es8311Model::with_ce(true));
        let mut bus = FakeI2c::new(&model);
        let mut id = [0];
        embassy_futures::block_on(fake_bus::embedded_hal_async::i2c::I2c::write_read(
            &mut bus,
            0x19,
            &[0xFE],
            &mut id,
        ))
        .unwrap();
        assert_eq!(id, [0x11]);
    }
}
