//! Wokwi custom chip wrapping the `es8311-sim` behavioral model.
//!
//! Pins: `VCC`, `GND`, `SDA`, `SCL` and `CE`. `CE` is sampled once at start-up and selects
//! the I²C address: low (or unconnected) for `0x18`, high for `0x19`. The audio pins are
//! not part of the chip; the model has no audio signal path.
//!
//! The chip logs to the simulator console whenever the DAC output is switched on or off.

use es8311_sim::Es8311Model;
use std::cell::RefCell;
use std::ffi::{CString, c_void};
use wokwi_chip_ll::*;

struct Chip {
    model: Es8311Model,
    output_enabled: bool,
}

thread_local! {
    static CHIP: RefCell<Option<Chip>> = const { RefCell::new(None) };
}

fn with_chip<R>(f: impl FnOnce(&mut Chip) -> R) -> R {
    CHIP.with(|chip| f(chip.borrow_mut().as_mut().expect("chipInit not called")))
}

fn cstr(s: &str) -> CString {
    CString::new(s).expect("no interior NUL")
}

fn debug(message: &str) {
    let message = cstr(message);
    // SAFETY: `message` is a valid NUL-terminated string for the duration of the call.
    unsafe { debugPrint(message.as_ptr()) };
}

extern "C" fn on_connect(_: *mut c_void, _address: u32, is_read: bool) -> bool {
    with_chip(|chip| chip.model.i2c_start(is_read))
}

extern "C" fn on_read(_: *mut c_void) -> u8 {
    with_chip(|chip| chip.model.i2c_read())
}

extern "C" fn on_write(_: *mut c_void, data: u8) -> bool {
    with_chip(|chip| {
        let ack = chip.model.i2c_write(data);
        let enabled = chip.model.dac_output_enabled();

        if enabled != chip.output_enabled {
            chip.output_enabled = enabled;
            debug(if enabled {
                "es8311: DAC output on"
            } else {
                "es8311: DAC output off"
            });
        }

        ack
    })
}

extern "C" fn on_disconnect(_: *mut c_void) {}

/// Entry point called by the simulator.
#[unsafe(no_mangle)]
pub extern "C" fn chipInit() {
    // SAFETY: All pointers passed to the simulator are valid for the duration of each
    // call; configs are copied by the simulator. Init functions are only called here.
    unsafe {
        let ce = pinInit(cstr("CE").as_ptr(), INPUT_PULLDOWN);
        let sda = pinInit(cstr("SDA").as_ptr(), INPUT);
        let scl = pinInit(cstr("SCL").as_ptr(), INPUT);

        let model = Es8311Model::with_ce(pinRead(ce) == HIGH);
        i2cInit(&I2CConfig {
            user_data: std::ptr::null(),
            address: u32::from(model.address()),
            scl,
            sda,
            connect: on_connect,
            read: on_read,
            write: on_write,
            disconnect: on_disconnect,
        });

        let address = model.address();
        CHIP.with(|c| {
            *c.borrow_mut() = Some(Chip {
                model,
                output_enabled: false,
            })
        });
        debug(&format!("es8311: chip initialized at 0x{address:02X}"));
    }
}
