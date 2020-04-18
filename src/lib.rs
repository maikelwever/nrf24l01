//! A platform agnostic driver to interface with the nRF24L01 (2.4GHz Wireless)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1

#![deny(unsafe_code)]
#![no_std]

extern crate embedded_hal;

use embedded_hal::blocking;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::{Mode, Phase, Polarity};

mod constants;
pub use crate::constants::{BitMnemonic, Instruction, Memory, MIRF_ADDR_LEN, MIRF_CONFIG};

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// Error
#[derive(Debug)]
pub enum Error<E> {
    /// Late collision
    LateCollision,
    /// SPI error
    Spi(E),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Spi(e)
    }
}

pub struct NRF24L01<SPI, CSN, CE> {
    spi: SPI,
    csn: CSN,
    ce: CE,

    channel: u8,
    payload_size: u8,
    tx_power_status: bool,
}

impl<E, SPI, CSN, CE> NRF24L01<SPI, CSN, CE>
where
    SPI: blocking::spi::Transfer<u8, Error = E> + blocking::spi::Write<u8, Error = E>,
    CSN: OutputPin,
    CE: OutputPin,
{
    pub fn new(spi: SPI, csn: CSN, ce: CE, channel: u8, payload_size: u8) -> Result<Self, Error<E>> {
        let mut nrf24l01 = NRF24L01 {
            spi,
            csn,
            ce,

            channel,
            payload_size,
            tx_power_status: false,
        };

        let _ = nrf24l01.ce.set_low();
        let _ = nrf24l01.csn.set_high();

        Ok(nrf24l01)
    }

    pub fn config(&mut self) -> Result<(), Error<E>> {
        // This was done in the python version but not the C version.
        // Seems to work without it so leave this be commented.
        // nrf24l01.power_down()?;
        // self.config_register(Memory::SETUP_RETR, &0b11111)?;

        let channel = self.channel;
        let payload_size = self.payload_size;
        self.config_register(Memory::RF_CH, &channel)?;
        self.config_register(Memory::RX_PW_P0, &payload_size)?;
        self.config_register(Memory::RX_PW_P1, &payload_size)?;

        self.power_up_rx()?;
        self.flush_rx()?;
        Ok(())
    }

    fn config_register(&mut self, register: u8, value: &u8) -> Result<(), Error<E>> {
        let _ = self.csn.set_low();
        self.spi
            .write(&[Instruction::W_REGISTER | (Instruction::REGISTER_MASK & register)])?;
        self.spi.write(&[*value])?;
        let _ = self.csn.set_high();
        Ok(())
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let _ = self.csn.set_low();
        self.spi
            .write(&[Instruction::R_REGISTER | (Instruction::REGISTER_MASK & register)])?;
        let mut buffer = [0];
        self.spi.transfer(&mut buffer)?;
        let _ = self.csn.set_high();
        Ok(buffer[0])
    }

    fn write_register(&mut self, register: u8, value: &[u8]) -> Result<(), Error<E>> {
        let _ = self.csn.set_low();

        self.spi
            .write(&[Instruction::W_REGISTER | (Instruction::REGISTER_MASK & register)])?;
        self.spi.write(value)?;
        let _ = self.csn.set_high();
        Ok(())
    }

    pub fn power_down(&mut self) -> Result<(), Error<E>> {
        let _ = self.ce.set_low();
        self.config_register(Memory::CONFIG, &MIRF_CONFIG)?;
        Ok(())
    }

    fn power_up_rx(&mut self) -> Result<(), Error<E>> {
        self.tx_power_status = false;
        let _ = self.ce.set_low();
        self.config_register(
            Memory::CONFIG,
            &(MIRF_CONFIG | ((1 << BitMnemonic::PWR_UP) | (1 << BitMnemonic::PRIM_RX))),
        )?;
        let _ = self.ce.set_high();
        self.config_register(
            Memory::STATUS,
            &((1 << BitMnemonic::TX_DS) | (1 << BitMnemonic::MAX_RT)),
        )?;
        Ok(())
    }

    fn power_up_tx(&mut self) -> Result<(), Error<E>> {
        self.tx_power_status = true;
        self.config_register(
            Memory::CONFIG,
            &(MIRF_CONFIG | ((1 << BitMnemonic::PWR_UP) | (0 << BitMnemonic::PRIM_RX))),
        )?;
        Ok(())
    }

    fn flush_rx(&mut self) -> Result<(), Error<E>> {
        let _ = self.csn.set_low();
        self.spi.write(&[Instruction::FLUSH_RX])?;
        let _ = self.csn.set_high();
        Ok(())
    }

    pub fn free(self) -> (SPI, CSN, CE) {
        (self.spi, self.csn, self.ce)
    }

    pub fn set_raddr(&mut self, addr: &[u8]) -> Result<(), Error<E>> {
        let _ = self.ce.set_low();
        self.write_register(Memory::RX_ADDR_P1, addr)?;
        let _ = self.ce.set_high();
        Ok(())
    }

    pub fn set_taddr(&mut self, addr: &[u8]) -> Result<(), Error<E>> {
        self.write_register(Memory::RX_ADDR_P0, addr)?;
        self.write_register(Memory::TX_ADDR, addr)?;
        Ok(())
    }

    pub fn get_status(&mut self) -> Result<u8, Error<E>> {
        let response = self.read_register(Memory::STATUS)?;
        Ok(response)
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), Error<E>> {
        let _ = self.get_status()?; // I'm not entirely sure why, but Mirf does this, so we do as well.
        while self.tx_power_status {
            let status = self.get_status()?;
            if (status & ((1 << BitMnemonic::TX_DS) | (1 << BitMnemonic::MAX_RT))) != 0 {
                self.tx_power_status = false;
                break;
            }
        }

        let _ = self.ce.set_low();
        self.power_up_tx()?;

        let _ = self.csn.set_low();
        self.spi.write(&[Instruction::FLUSH_TX])?;
        let _ = self.csn.set_high();

        let _ = self.csn.set_low();
        self.spi.write(&[Instruction::W_TX_PAYLOAD])?;
        self.spi.write(data)?;
        let _ = self.csn.set_high();

        let _ = self.ce.set_high();
        Ok(())
    }

    pub fn is_sending(&mut self) -> Result<bool, Error<E>> {
        if self.tx_power_status {
            let status = self.get_status()?;
            if (status & ((1 << BitMnemonic::TX_DS) | (1 << BitMnemonic::MAX_RT))) != 0 {
                self.power_up_rx()?;
                return Ok(false);
            }

            return Ok(true);
        }
        Ok(false)
    }

    pub fn data_ready(&mut self) -> Result<bool, Error<E>> {
        let status = self.get_status()?;
        if (status & (1 << BitMnemonic::RX_DR)) != 0 {
            return Ok(true);
        }
        let fifo_empty = self.rx_fifo_empty()?;
        Ok(!fifo_empty)
    }

    fn rx_fifo_empty(&mut self) -> Result<bool, Error<E>> {
        let fifo_status = self.read_register(Memory::FIFO_STATUS)?;
        if fifo_status & (1 << BitMnemonic::RX_EMPTY) != 0 {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_data(&mut self, buf: &mut [u8]) -> Result<(), Error<E>> {
        let _ = self.csn.set_low();
        self.spi.write(&[Instruction::R_RX_PAYLOAD])?;
        self.spi.transfer(buf)?;
        let _ = self.csn.set_high();
        self.config_register(Memory::STATUS, &(1 << BitMnemonic::RX_DR))?;
        Ok(())
    }
}
