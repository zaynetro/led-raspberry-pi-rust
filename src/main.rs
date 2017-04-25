extern crate sysfs_gpio;

mod led;

use std::thread::sleep;
use std::time::Duration;

use led::{LED, LEDImpl};

fn main() {
    let led = LEDImpl::new(147);
    blink(led, 50);
}

fn blink<L: LED>(mut led: L, times: i32) -> L {
    // Could be that we need to wait for a bit on Raspberry Pi
    sleep(Duration::from_millis(200));
    // TODO: Fix this method to make tests successfull
    led
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysfs_gpio::Error;

    #[derive(Debug)]
    #[derive(PartialEq)]
    enum LEDMockAction {
        On,
        Off,
    }

    struct LEDMock {
        pub actions: Vec<LEDMockAction>,
    }

    impl LEDMock {
        pub fn new() -> LEDMock {
            LEDMock { actions: Vec::new() }
        }
    }

    impl LED for LEDMock {
        fn on(&mut self) -> Result<(), Error> {
            self.actions.push(LEDMockAction::On);
            Ok(())
        }

        fn off(&mut self) -> Result<(), Error> {
            self.actions.push(LEDMockAction::Off);
            Ok(())
        }
    }

    #[test]
    fn blink_test() {
        let mock_led = LEDMock::new();
        let times = 2;
        let mock_led = blink(mock_led, times);
        let expected_actions = vec![LEDMockAction::On,
                                    LEDMockAction::Off,
                                    LEDMockAction::On,
                                    LEDMockAction::Off];
        assert_eq!(expected_actions.len(), mock_led.actions.len());
        for (actual, expected) in mock_led.actions.iter().zip(expected_actions) {
            assert_eq!(expected, *actual);
        }
    }
}
