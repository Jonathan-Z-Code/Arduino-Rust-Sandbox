JHD162A LCD Driver [By: Jonathan Zurita]

# Detailed Introduction

Welcome! This is the **JHD162A LCD Driver**
documentation! There are three key components
to this build:
- [x] Initializing LCD display
- [x] Writing `static &str` and `char` arrays to LCD
- [x] Converting non-char types into characters
 
Each component will be explained in-depth,
including design and implementation choices by the author
 
## Question 1) Why convert float to a char array?
I personally believed that displaying certain float values such as ADC readings or other fine-tuned data could be proven useful. Therefore, I included a float-to-char function out of the box!
However, the function `float_to_array` only supports up to 2 decimal places in accuracy.
## Question 2) Why is it solely in 8-bit mode?
Originally, I started on developing a JHD162A LCD Driver for the STM32F4 series. I started development in 4-bit mode, saving precious gpio pins. However, I decided to take a different route and try out 8-bit mode for the LCD driver.Thankfully the 8-bit mode technically can transmit new characters at a faster rate. Depends mostly your situation (desired rate of data transmission).
## Question 3) Are there next steps in development?
Possible next steps would be to implement a 4-bit mode. Another thing would be to create a example folder? main.rs should contain an example of analog pin 0 being read and displayed on LCD screen.