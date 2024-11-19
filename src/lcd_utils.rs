
use arduino_hal::port::{mode, Pin};

/// Maximum length for one row of characters
const MAX_LENGTH: u8 = 16; 

/// Arbitrary value, user-defined
const MS_DELAY: u16 = 3_000; 

/// All pin macro numbers correspond
/// to their respective index the in the gpio array in main()
const LED_PIN: usize = 0;
const D7_PIN: usize = 1;
const D6_PIN: usize = 2;
const D5_PIN: usize = 3;
const D4_PIN: usize = 4;
const D3_PIN: usize = 5;
const D2_PIN: usize = 6;
const D1_PIN: usize = 7;
const D0_PIN: usize = 8;
const ENABLE_PIN: usize = 9;
const READ_WRITE_PIN: usize = 10;
const REGISTER_SEL: usize = 11;

/// BIT_0 through BIT_7 are bit masks in order to
/// process the ASCII char parameter
const BIT_0: usize = 0x01; 
const BIT_1: usize = 0x02;
const BIT_2: usize = 0x04;
const BIT_3: usize = 0x08;
const BIT_4: usize = 0x10;
const BIT_5: usize = 0x20;
const BIT_6: usize = 0x40;
const BIT_7: usize = 0x80; 


/// Provided in order to convert u8 into proper ASCII char
const ASCII_OFFSET: u8 = 0x30; 

/// Function converts a `f32` into a `[char]` array
/// in order to be processed by the LCD display
pub fn float_to_array(float: f32, arr: &mut [char]) {
    
    // this method is scalable up to X amount of decimal places
    // although i think multipling very large float numbers
    // would be significant overhead! :)
    let float_hundredths: u8 = ( ( (float) * 100.0) as u8 ) % 10;
    let float_tenths: u8 = ( ( (float) * 10.0) as u8 ) % 10;
    let float_ones: u8 = float as u8;
    
    arr[0] = (float_ones + ASCII_OFFSET) as char;
    arr[2] = (float_tenths + ASCII_OFFSET) as char;
    arr[3] = (float_hundredths + ASCII_OFFSET) as char;
    
}

/// Clears all data pins (D0-D7)
pub fn clear_data_bits( gpio: &mut [Pin<mode::Output>]) {    
    
    // clear all data bits!
    gpio[D7_PIN].set_low(); 
    gpio[D6_PIN].set_low();
    gpio[D5_PIN].set_low();
    gpio[D4_PIN].set_low();   
    gpio[D3_PIN].set_low(); 
    gpio[D2_PIN].set_low();
    gpio[D1_PIN].set_low();
    gpio[D0_PIN].set_low();

}

/// Writes a static `&str` to the LCD display
pub fn write_static_string(gpio: &mut [Pin<mode::Output>], s: &str) {
    
    let mut str_length: u8 = 0;

    for _c in s.chars() {
        str_length += 1; // iterate through each char
    }
    if str_length > MAX_LENGTH { // if &str is too long, exit function!
        return;
    }

    gpio[LED_PIN].set_high(); // start of data transmission 
    gpio[REGISTER_SEL].set_high(); // enable RS pin
 
    for c in s.chars() {

        // clear all pins
        clear_data_bits(gpio);

        // parse each bit in char
        let d7: u8 = (c as u8) & (BIT_7 as u8); 
        let d6: u8 = (c as u8) & (BIT_6 as u8); 
        let d5: u8 = (c as u8) & (BIT_5 as u8); 
        let d4: u8 = (c as u8) & (BIT_4 as u8); 
        let d3: u8 = (c as u8) & (BIT_3 as u8); 
        let d2: u8 = (c as u8) & (BIT_2 as u8);
        let d1: u8 = (c as u8) & (BIT_1 as u8); 
        let d0: u8 = (c as u8) & (BIT_0 as u8);  

        // set each data bit high depending on bit mask
        if d7 > 0 {
            gpio[D7_PIN].set_high(); 
        }
        if d6 > 0  {
            gpio[D6_PIN].set_high(); 
        }
        if d5 > 0 {
            gpio[D5_PIN].set_high(); 
        }
        if d4 > 0 { 
            gpio[D4_PIN].set_high(); 
        }
        if d3 > 0 {
            gpio[D3_PIN].set_high(); 
        }
        if d2 > 0 {
            gpio[D2_PIN].set_high(); 
        }
        if d1 > 0 { 
            gpio[D1_PIN].set_high(); 
        }
        if d0 > 0 {
            gpio[D0_PIN].set_high(); 
        }
        
        gpio[ENABLE_PIN].set_high(); // start enable
        arduino_hal::delay_ms(10);
        gpio[ENABLE_PIN].set_low(); // sample data bits
        arduino_hal::delay_ms(20);

    }
    
    gpio[REGISTER_SEL].set_low(); // disable RS pin  
    gpio[LED_PIN].set_low(); // end of data transmission

}

/// Writes a `[char]` array to the LCD display
pub fn write_char_array(gpio: &mut [Pin<mode::Output>], arr: &mut [char]) {
    
    let arr_length: usize = arr.len();

    gpio[LED_PIN].set_high(); // start of data transmission 
    gpio[REGISTER_SEL].set_high(); // enable RS pin

    for i in 0..arr_length {
    
        let c: char = arr[i];
        
        // clear all pins
        clear_data_bits(gpio);
    
        // parse each bit in char
        let d7: u8 = (c as u8) & (BIT_7 as u8); 
        let d6: u8 = (c as u8) & (BIT_6 as u8); 
        let d5: u8 = (c as u8) & (BIT_5 as u8); 
        let d4: u8 = (c as u8) & (BIT_4 as u8); 
        let d3: u8 = (c as u8) & (BIT_3 as u8); 
        let d2: u8 = (c as u8) & (BIT_2 as u8);
        let d1: u8 = (c as u8) & (BIT_1 as u8); 
        let d0: u8 = (c as u8) & (BIT_0 as u8);  
    
        // set each data bit high depending on bit mask
        if d7 > 0 {
            gpio[D7_PIN].set_high(); 
        }
        if d6 > 0  {
            gpio[D6_PIN].set_high(); 
        }
        if d5 > 0 {
            gpio[D5_PIN].set_high(); 
        }
        if d4 > 0 { 
            gpio[D4_PIN].set_high(); 
        }
        if d3 > 0 {
            gpio[D3_PIN].set_high(); 
        }
        if d2 > 0 {
            gpio[D2_PIN].set_high(); 
        }
        if d1 > 0 { 
            gpio[D1_PIN].set_high(); 
        }
        if d0 > 0 {
            gpio[D0_PIN].set_high(); 
        }
            
        gpio[ENABLE_PIN].set_high(); // start enable
        arduino_hal::delay_ms(10);
        gpio[ENABLE_PIN].set_low(); // sample data bits
        arduino_hal::delay_ms(20);
    
        }
        
        gpio[REGISTER_SEL].set_low(); // disable RS pin
        gpio[LED_PIN].set_low(); // end of data transmission 

}

/// Fills whole LCD display with characters as a test
pub fn test_write_char( gpio: &mut [Pin<mode::Output>] ) {

    gpio[REGISTER_SEL].set_high(); // enable RS pin

    for _i in 0..16 {
        
        gpio[D7_PIN].set_low(); // character 'H'
        gpio[D6_PIN].set_high();
        gpio[D5_PIN].set_low();
        gpio[D4_PIN].set_low();   
        gpio[D3_PIN].set_high(); 
        gpio[D2_PIN].set_low();
        gpio[D1_PIN].set_low();
        gpio[D0_PIN].set_low();

        gpio[ENABLE_PIN].set_high(); // start enable
        arduino_hal::delay_ms(10);
        gpio[ENABLE_PIN].set_low(); // sample data bits
        arduino_hal::delay_ms(20);

    }

    gpio[REGISTER_SEL].set_low(); // disable RS pin

    // move cursor to second line!
    gpio[D7_PIN].set_high();
    gpio[D6_PIN].set_high();
    gpio[D5_PIN].set_low();
    gpio[D4_PIN].set_low(); 
    gpio[D3_PIN].set_low(); 
    gpio[D2_PIN].set_low();
    gpio[D1_PIN].set_low();
    gpio[D0_PIN].set_low();

    gpio[ENABLE_PIN].set_high(); // start enable 
    arduino_hal::delay_ms(10);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(20);


    gpio[REGISTER_SEL].set_high(); // enable RS pin

    for _i in 0..16 {
        
        gpio[D7_PIN].set_low(); // character 'H'
        gpio[D6_PIN].set_high();
        gpio[D5_PIN].set_low();
        gpio[D4_PIN].set_low();   
        gpio[D3_PIN].set_high(); 
        gpio[D2_PIN].set_low();
        gpio[D1_PIN].set_low();
        gpio[D0_PIN].set_low();

        gpio[ENABLE_PIN].set_high(); // start enable
        arduino_hal::delay_ms(10);
        gpio[ENABLE_PIN].set_low(); // sample data bits
        arduino_hal::delay_ms(20);

    }
    
    gpio[REGISTER_SEL].set_low(); // disable RS pin

    gpio[D7_PIN].set_low();
    gpio[D6_PIN].set_low();
    gpio[D5_PIN].set_low();
    gpio[D4_PIN].set_low(); 
    gpio[D3_PIN].set_low(); // set cursor to home position
    gpio[D2_PIN].set_low();
    gpio[D1_PIN].set_high();
    gpio[D0_PIN].set_low();

    gpio[ENABLE_PIN].set_high(); // start enable
    arduino_hal::delay_ms(10);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(20);

}

/// Clears the LCD display, and sets the cursor back to the beginning
pub fn reset_lcd( gpio: &mut [Pin<mode::Output>] ) {

    // allow gpio pins to settle if 
    // this function is called on boot
    arduino_hal::delay_ms(1);
    gpio[REGISTER_SEL].set_low(); 
    gpio[READ_WRITE_PIN].set_low();

    gpio[LED_PIN].set_high(); // start reset process
    
    gpio[D7_PIN].set_low();
    gpio[D6_PIN].set_low();
    gpio[D5_PIN].set_high();
    gpio[D4_PIN].set_high(); // 8-bit data mode
    gpio[D3_PIN].set_high(); // 2-row mode 
    gpio[D2_PIN].set_high(); // 5X10 font style 
    gpio[D1_PIN].set_low();
    gpio[D0_PIN].set_low();

    gpio[ENABLE_PIN].set_high(); // start enable
    arduino_hal::delay_ms(1);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(2);

    gpio[D7_PIN].set_low();
    gpio[D6_PIN].set_low();
    gpio[D5_PIN].set_low();
    gpio[D4_PIN].set_low(); // clear these pins

    gpio[D3_PIN].set_low(); // reset LCD screen
    gpio[D2_PIN].set_low();
    gpio[D1_PIN].set_low();
    gpio[D0_PIN].set_high();    

    gpio[ENABLE_PIN].set_high(); // start enable
    arduino_hal::delay_ms(1);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(2);

    gpio[D3_PIN].set_low(); // set cursor to home position
    gpio[D2_PIN].set_low();
    gpio[D1_PIN].set_high();
    gpio[D0_PIN].set_low();

    gpio[ENABLE_PIN].set_high(); // start enable
    arduino_hal::delay_ms(1);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(2);

    gpio[D3_PIN].set_high(); // set blinking cursor
    gpio[D2_PIN].set_high();
    gpio[D1_PIN].set_high();
    gpio[D0_PIN].set_high();

    gpio[ENABLE_PIN].set_high(); // start enable 
    arduino_hal::delay_ms(1);
    gpio[ENABLE_PIN].set_low(); // sample data bits
    arduino_hal::delay_ms(2);

    gpio[LED_PIN].set_low(); // reset process finished! :)

}

