#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(special_module_name)]

#[doc = include_str!("./lib/lcd_doc.md")]


mod lib;
use lib::lcd_utils;

use arduino_hal::{hal::{self, port}, port::{mode, Pin}, Peripherals, Pins};

// user defined panic handler
use panic_halt as _;

/// Test function that grabs a mutable pointer and implicitly returns it back
/// Along with mutable pointers to gpio pin array
/// and a immutable borrow of a str type!
fn borrow_uart( serial:  &mut hal::usart::Usart0<arduino_hal::DefaultClock>,
                gpio: &mut [Pin<mode::Output>],
                s: &str,
              ) -> u8  // retval is a unsigned 8-bit value
{

    let mut err_num: u8 = 0;

    // IMPORTANT: reason why this loop does not trigger double barrowing error is because within the iterator scope "for char in s.chars"
    // it is only being borrowed once per iteration, which is good!
    for character in s.chars() {

        // since I am passing a mutable reference into the function, I only have to pass the name.
        match ufmt::uwriteln!(serial,  "{}", character) {
            Err(e) => err_num += 1 ,
            Ok(t) => err_num += 0 ,
        }
    
    }
    
    return err_num;

}

#[arduino_hal::entry]
fn main() -> ! {
    
    // configure peripheral library and pin layout for ATMEGA328P
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // by default_serial: serial = Usart<Atmega, USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>
    // pub struct Usart<H, USART, RX, TX, CLOCK>
    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _serial = arduino_hal::Usart::new(
        dp.USART0, // UART0 peripheral selected 
        pins.d0, // RX pin
        pins.d1.into_output(), // TX pin
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600), // 57600 baud rate
    );
    // adc initialization
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // declare data bits for LCD display
    let mut lcd_pins: [Pin<mode::Output>; 12] = [
        pins.d13.into_output().downgrade(), // LED
        pins.d12.into_output().downgrade(), // DB7
        pins.d11.into_output().downgrade(), // DB6
        pins.d10.into_output().downgrade(), // DB5
        pins.d9.into_output().downgrade(),  // DB4
        pins.d8.into_output().downgrade(),  // DB3
        pins.d7.into_output().downgrade(),  // DB2
        pins.d6.into_output().downgrade(),  // DB1
        pins.d5.into_output().downgrade(),  // DB0
        pins.d4.into_output().downgrade(),  // EN
        pins.d3.into_output().downgrade(),  // R/W
        pins.d2.into_output().downgrade(),  // RS
    ];
    
    // declaration of adc analog pin 0 
    let a0 = pins.a0.into_analog_input(&mut adc); 
    
    // declaration of static string and char array
    let test_str: &str = "ADC0: ";
    let mut char_arr: [char; 5] = ['0', '.', '0', '0', 'V']; 

    // return any new error_values (if any) and add it to the master error counter
    // also since I am passing it as a &mut, ownership will return back to the original serial in main()
    // WARNING: Must pass mutable references if you want to toggle led or send UART msgs!
    //error_counter += borrow_uart(&mut serial, &mut leds, test_str); 

    // this test spams the letter 'H' on both LCD rows
    // lcd_utils::test_write_char(&mut lcd_pins); 

    // start of forever loop
    loop {
        arduino_hal::delay_ms(2000);
        lcd_utils::reset_lcd(&mut lcd_pins);
        let adc_0_val: f32 = ( a0.analog_read(&mut adc) as f32 ) * 5.0 / 1023.0; // convert from hex value to human readable voltage
        lcd_utils::float_to_array(adc_0_val, &mut char_arr); // convert float into LCD display char array
        lcd_utils::write_static_string(&mut lcd_pins, test_str); // start LCD with standard "ADC0: " static string
        lcd_utils::write_char_array(&mut lcd_pins, &mut char_arr); // end LCD row 1 with the actual ADC voltage value!
    }

}
