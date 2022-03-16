use nrf52840_pac as pac;
use self::pac::RADIO;

pub struct Radio {
    periph : RADIO,
}
