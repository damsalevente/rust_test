#![no_std]
#![no_main]

extern crate panic_halt; 
pub extern crate stm32f4;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use stm32f4::stm32f446::TIM3;

#[inline(never)]
fn delay(tim3:&TIM3, ms: u32) {

  tim3.arr.write(|w| w.arr().bits(ms));
  tim3.cr1.modify(|_, w| w.cen().set_bit());
  while !tim3.sr.read().uif().bit_is_set() {}

  // clear event flag 
  tim3.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
  let p = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = p.RCC;


  // give em clock 
  // refer to https://stm32.agg.io/rs/STM32F446.html#TIM3
  rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit().gpioben().set_bit());
  rcc.apb1enr.modify(|_, w| w.tim3en().set_bit().spi2en().set_bit());

  let gpioa = &p.GPIOA;
  let gpiob = &p.GPIOB;

  let tim3 = p.TIM3;

  //timer init pulse mode and disable en signal
  tim3.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

  // configure prescaler 
  tim3.psc.modify(|_, w| unsafe{ w.bits(15_000)});

  // set for output 
  gpioa.moder.write(|w| w.moder5().output().moder1().output());

  // set spi2 pins to alternate function
  gpiob.moder.write(| w| w.moder15().alternate()
      .moder14().alternate()
      .moder10().alternate()
      .moder9().alternate());
  // check if it set correctly  
  let alternatefunc = gpiob.moder.read().moder15().bits() as u8;
  hprintln!("Mode register alternate function? : {}",alternatefunc).unwrap();
  // set high speed pins
  gpiob.ospeedr.modify(|_,w| w.ospeedr15().very_high_speed()
      .ospeedr14().very_high_speed()
      .ospeedr10().very_high_speed()
      .ospeedr9().very_high_speed());

  let moderegister_val = gpiob.moder.read().moder15().bits() as u8;
  hprintln!("Register's value : {}",moderegister_val).unwrap();
  //pullup 
  //gpiob.pupdr.modify(|_, w|unsafe{ w.pupdr15().bits(0x01).
  //                        pupdr14().bits(0x01).
  //                        pupdr10().bits(0x01).
  //                        pupdr9().bits(0x01)});
  //alternate fucntion 5, spi2
  gpiob.afrh.modify(|_, w| w.afrh9().bits(0x05)
      .afrh10().bits(0x05)
      .afrh15().bits(0x05)
      .afrh14().bits(0x05));

  // spi config 
  let spi2 = &p.SPI2;

  // spi2 enable, and master mode
  // spi2 config written 
  spi2.cr1.write(|w| w.mstr().set_bit()
      .cpha().set_bit()
      .cpol().set_bit()
      .lsbfirst().msbfirst()
      .ssm().clear_bit()
      .br().bits(0x02)
      );

  // spi cr2 config, everything zero except ssoe 
  spi2.cr2.write(|w| w.ssoe().enabled());


  // check if works 
  let cr2_val = spi2.cr2.read().bits() as u32;
  hprintln!("Register's value : {}",cr2_val).unwrap();

  // broadcast message
  // write data register
  spi2.dr.modify(|_, w| unsafe{w.bits(0x0f)});

  //check gpiob9
  let mut  nsscheck = gpiob.odr.read().odr9().bit();
  hprintln!("Nss 's value : {}",nsscheck).unwrap();

  

  // disable spi oe 
  spi2.cr1.modify(|_, w| w.spe().bit(false));

  //check gpiob9 again
  nsscheck = gpiob.odr.read().odr9().bit();
  // print state 
  
  hprintln!("Disabled nss's value : {}",nsscheck).unwrap();

  // enable spi oe 
  spi2.cr1.modify(|_, w| w.spe().set_bit());

  //check gpiob9 again
  nsscheck = gpiob.odr.read().odr9().bit();

  hprintln!("nss's value : {}",nsscheck).unwrap();

      // turn led on
  gpioa.odr.write(|w| w.odr5().set_bit());
  let ms = 5;
  loop {
    let _word = spi2.dr.read().bits() as u16;
    hprintln!("Hello, world!{}",_word).unwrap();



    //delay(&tim3, ms); 
    //gpioa.odr.write(|w| w.odr5().clear_bit());
    ////gpioa.odr.write(|w| w.odr1().set_bit());
    //delay(&tim3, ms); 
    ////gpioa.odr.write(|w| w.odr1().clear_bit());
    //gpioa.odr.write(|w| w.odr5().set_bit());
  }   
}

