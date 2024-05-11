#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

struct LedMatrix {
    row1: *mut u32,
    row2: *mut u32,
    row3: *mut u32,
    row4: *mut u32,
    row5: *mut u32,
    row_locations: [u32;5],
    col1: *mut u32,
    col2: *mut u32,
    col3: *mut u32,
    col4: *mut u32,
    col5: *mut u32,
    gpio0_out: *mut u32,
}

impl LedMatrix{
    fn new() -> Self{
        let l = LedMatrix{//   This is the pin location
            row1: 0x5000_0754 as *mut u32,// P0 - 21
            row2: 0x5000_0758 as *mut u32,// P0 - 22
            row3: 0x5000_073C as *mut u32,// P0 - 15
            row4: 0x5000_0760 as *mut u32,// P0 - 24
            row5: 0x5000_074C as *mut u32,// P0 - 19
            row_locations: [21 ,22 ,15 ,24, 19 ],
            col1: 0x5000_0770 as *mut u32,
            col2: 0x5000_072C as *mut u32,//P0 - 11
            col3: 0x5000_077C as *mut u32,
            // P1 base - 0x5000_0300 , and offset for pin 5 is 714
            col4: (0x5000_0A14) as *mut u32,// P1 - 05
            col5: 0x5000_0778 as *mut u32,
            gpio0_out: 0x5000_0504 as *mut u32 // Out regiister of GPIO P0
        };
        // Initializing all the rows and colums to be activated
        unsafe{
            //row setup
            write_volatile(l.row1, 1);
            write_volatile(l.row2, 1);
            write_volatile(l.row3, 1);
            write_volatile(l.row4, 1);
            write_volatile(l.row5, 1);
            // column setup
            write_volatile(l.col1, 1);
            write_volatile(l.col2, 1);
            write_volatile(l.col3, 1);
            write_volatile(l.col4, 1);
            write_volatile(l.col5, 1);
        }

        return  l;
    }

    fn drive_all(&self, value: bool){
        // Setting the bits corresponding to each row to
        // the value of 0 or 1 by left shifting to the right positions
        let mut drive_value: u32 = 0;
        for i in 0..5{
            drive_value += (value as u32 )<< self.row_locations[i];
        }
        // Finally writing out the values
        unsafe{
            write_volatile(self.gpio0_out,drive_value);
        }
    }
}

#[entry]
fn main() -> ! {
    let l = LedMatrix::new();
   let mut is_on: bool = true;
   // This section turn the entire led matrix on and off.
   loop{
    l.drive_all(is_on);

    // simple wait sequence
    for _ in 0..400_000{
        nop();
    }

    is_on = !is_on;

   }
}