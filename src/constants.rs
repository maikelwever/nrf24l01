pub struct Memory {
}

impl Memory {
	pub const CONFIG: u8      = 0x00;
	pub const EN_AA: u8       = 0x01;
	pub const EN_RXADDR: u8   = 0x02;
	pub const SETUP_AW: u8    = 0x03;
	pub const SETUP_RETR: u8  = 0x04;
	pub const RF_CH: u8       = 0x05;
	pub const RF_SETUP: u8    = 0x06;
	pub const STATUS: u8      = 0x07;
	pub const OBSERVE_TX: u8  = 0x08;
	pub const CD: u8          = 0x09;
	pub const RX_ADDR_P0: u8  = 0x0A;
	pub const RX_ADDR_P1: u8  = 0x0B;
	pub const RX_ADDR_P2: u8  = 0x0C;
	pub const RX_ADDR_P3: u8  = 0x0D;
	pub const RX_ADDR_P4: u8  = 0x0E;
	pub const RX_ADDR_P5: u8  = 0x0F;
	pub const TX_ADDR: u8     = 0x10;
	pub const RX_PW_P0: u8    = 0x11;
	pub const RX_PW_P1: u8    = 0x12;
	pub const RX_PW_P2: u8    = 0x13;
	pub const RX_PW_P3: u8    = 0x14;
	pub const RX_PW_P4: u8    = 0x15;
	pub const RX_PW_P5: u8    = 0x16;
	pub const FIFO_STATUS: u8 = 0x17;
}

pub struct BitMnemonic {
}

impl BitMnemonic {
	pub const MASK_RX_DR: u8 =  6;
	pub const MASK_TX_DS: u8 =  5;
	pub const MASK_MAX_RT: u8=  4;
	pub const EN_CRC: u8     =  3;
	pub const CRCO: u8       =  2;
	pub const PWR_UP: u8     =  1;
	pub const PRIM_RX: u8    =  0;
	pub const ENAA_P5: u8    =  5;
	pub const ENAA_P4: u8    =  4;
	pub const ENAA_P3: u8    =  3;
	pub const ENAA_P2: u8    =  2;
	pub const ENAA_P1: u8    =  1;
	pub const ENAA_P0: u8    =  0;
	pub const ERX_P5: u8     =  5;
	pub const ERX_P4: u8     =  4;
	pub const ERX_P3: u8     =  3;
	pub const ERX_P2: u8     =  2;
	pub const ERX_P1: u8     =  1;
	pub const ERX_P0: u8     =  0;
	pub const AW: u8         =  0;
	pub const ARD: u8        =  4;
	pub const ARC: u8        =  0;
	pub const PLL_LOCK: u8   =  4;
	pub const RF_DR: u8      =  3;
	pub const RF_PWR: u8     =  1;
	pub const LNA_HCURR: u8  =  0;
	pub const RX_DR: u8      =  6;
	pub const TX_DS: u8      =  5;
	pub const MAX_RT: u8     =  4;
	pub const RX_P_NO: u8    =  1;
	pub const TX_FULL: u8    =  0;
	pub const PLOS_CNT: u8   =  4;
	pub const ARC_CNT: u8    =  0;
	pub const TX_REUSE: u8   =  6;
	pub const FIFO_FULL: u8  =  5;
	pub const TX_EMPTY: u8   =  4;
	pub const RX_FULL: u8    =  1;
	pub const RX_EMPTY: u8   =  0;

}

pub struct Instruction {
}

impl Instruction {
	pub const R_REGISTER: u8   =  0x00;
	pub const W_REGISTER: u8   =  0x20;
	pub const REGISTER_MASK: u8=  0x1F;
	pub const R_RX_PAYLOAD: u8 =  0x61;
	pub const W_TX_PAYLOAD: u8 =  0xA0;
	pub const FLUSH_TX: u8     =  0xE1;
	pub const FLUSH_RX: u8     =  0xE2;
	pub const REUSE_TX_PL: u8  =  0xE3;
	pub const NOP: u8          =  0xFF;
}

pub const MIRF_CONFIG: u8  = ((1<<3) | (0<<2) );
pub const MIRF_ADDR_LEN: u8 = 5;
