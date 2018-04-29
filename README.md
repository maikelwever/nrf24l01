# `nrf24l01`

> A platform agnostic driver to interface with the nRF24L01 (2.4GHz Wireless)

[![Build Status](https://travis-ci.org/maikelwever/nrf24l01.svg?branch=master)](https://travis-ci.org/maikelwever/nrf24l01)

## Overview

Using this crate you can send and receive data wirelessly over 2.4 GHz using a supported target of embedded-hal.
This is done using the nRF24L01 module. These modules can be found for cheap `(< 2$)` in various well known chinese stores.

This implementation is basically a port from https://github.com/aaronds/arduino-nrf24l01

This means you can communicate with nRF24L01 modules wired to an Arduino (for example).

## Example

Here is a simple example for using the nRF24L01 on a stm32f103xx device with stm32f103xx_hal.
It implements an echo server that expects a payload of 4 bytes.

This means it can be used as the server for the Mirf Arduino client example: https://github.com/aaronds/arduino-nrf24l01/blob/master/Mirf/examples/ping_client/ping_client.pde


```rust
extern crate panic_abort; // panicking behavior
extern crate stm32f103xx_hal as hal;
extern crate nrf24l01;

use hal::stm32f103xx;
use hal::spi::Spi;
use hal::prelude::*;

use nrf24l01::NRF24L01;


fn main() {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    led.set_high();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let ce = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut ncs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    ncs.set_high();
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        nrf24l01::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
        );

    // nRF24L01 library specific starts here.
    let mut nrf24l01 = NRF24L01::new(spi, ncs, ce, 1, 4).unwrap();
    nrf24l01.set_raddr("serv1".as_bytes()).unwrap();
    nrf24l01.config().unwrap();
    led.set_low();

    loop {
        if !nrf24l01.is_sending().unwrap() {
            if nrf24l01.data_ready().unwrap() {
                let mut buffer = [0; 4];
                nrf24l01.get_data(&mut buffer).unwrap();
                nrf24l01.set_taddr("clie1".as_bytes()).unwrap();
                nrf24l01.send(&buffer).unwrap();
                led.set_high();

            } else {
                led.set_low();
            }
        }
    }
}

```

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

