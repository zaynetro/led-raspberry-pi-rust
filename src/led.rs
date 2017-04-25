use sysfs_gpio::{Direction, Pin, Error};

pub trait LED {
    fn on(&mut self) -> Result<(), Error>;
    fn off(&mut self) -> Result<(), Error>;
}

pub struct LEDImpl {
    pin: Pin,
}

impl LEDImpl {
    pub fn new(pin_num: u64) -> Self {
        let pin = Pin::new(pin_num);
        let _ = pin.export();
        let _ = pin.set_direction(Direction::Out);
        LEDImpl { pin: pin }
    }
}

impl LED for LEDImpl {
    fn on(&mut self) -> Result<(), Error> {
        self.pin.set_value(1)
    }

    fn off(&mut self) -> Result<(), Error> {
        self.pin.set_value(0)
    }
}

impl Drop for LEDImpl {
    fn drop(&mut self) -> () {
        let _ = self.pin.unexport();
    }
}
