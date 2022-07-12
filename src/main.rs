#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use libm::{floorf, powf};
#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    hal::prelude::*,
    display::blocking::Display,
    hal::Timer,
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

const DEC_0: [[u8; 3]; 5] = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [1, 0, 0],
    ];

const DEC_1: [[u8; 3]; 5] = [
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ];
    
const DEC_2: [[u8; 3]; 5] = [
        [1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
        [1, 0, 0],
        [1, 1, 0],
    ];
    
const DEC_3: [[u8; 3]; 5] = [
        [1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
    ];
    
const DEC_4: [[u8; 3]; 5] = [
        [1, 0, 0],
        [1, 1, 0],
        [1, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ];
    
const DEC_5: [[u8; 3]; 5] = [
        [1, 1, 0],
        [1, 0, 0],
        [1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
    ];
    
const DEC_6: [[u8; 3]; 5] = [
        [1, 1, 0],
        [1, 0, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
    ];
    
const DEC_7: [[u8; 3]; 5] = [
        [1, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ];
    
const DEC_8: [[u8; 3]; 5] = [
        [1, 1, 0],
        [1, 1, 0],
        [0, 0, 0],
        [1, 1, 0],
        [1, 1, 0],
    ];
    
const DEC_9: [[u8; 3]; 5] = [
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
    ];

const UNI_0: [[u8; 2]; 5] = [
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [1, 0],
    ];

const UNI_1: [[u8; 2]; 5] = [
        [0, 1],
        [0, 1],
        [0, 1],
        [0, 1],
        [0, 1],
    ];
    
const UNI_2: [[u8; 2]; 5] = [
        [1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, 1],
    ];
    
const UNI_3: [[u8; 2]; 5] = [
        [1, 1],
        [0, 1],
        [1, 1],
        [0, 1],
        [1, 1],
    ];
    
const UNI_4: [[u8; 2]; 5] = [
        [1, 0],
        [1, 1],
        [1, 1],
        [0, 1],
        [0, 1],
    ];
    
const UNI_5: [[u8; 2]; 5] = [
        [1, 1],
        [1, 0],
        [1, 1],
        [0, 1],
        [1, 1],
    ];
    
const UNI_6: [[u8; 2]; 5] = [
        [1, 1],
        [1, 0],
        [1, 1],
        [1, 1],
        [1, 1],
    ];
    
const UNI_7: [[u8; 2]; 5] = [
        [1, 1],
        [0, 1],
        [0, 1],
        [0, 1],
        [0, 1],
    ];
    
const UNI_8: [[u8; 2]; 5] = [
        [1, 1],
        [1, 1],
        [0, 0],
        [1, 1],
        [1, 1],
    ];
    
const UNI_9: [[u8; 2]; 5] = [
        [1, 1],
        [1, 1],
        [1, 1],
        [0, 1],
        [0, 1],
    ];   
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = microbit::Board::take().unwrap();
		let mut full_lights: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]; 
    
		let mut unit_mat: [[u8; 2]; 5] = [
        [1, 2],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
    ];
    
    let mut dec_mat: [[u8; 3]; 5] = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ]; 
		let mut i = 0;
		let mut j = 0;
		let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
		
					
					
    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
		
    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz1).unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data_unscaled().unwrap();
            // RTT instead of normal print
            //let angle_board = ((data.z.abs()-275).abs()as f32 * (90.0/275.0))as u16;
            let angle_board = (-0.00000408117*powf(data.z.abs() as f32, 3 as f32))+ (0.0008967073*powf(data.z.abs() as f32, 2 as f32)) - (0.2611016383*data.z.abs() as f32)+ 91.215141336;
            match (angle_board%10.0) as u16 {
								0 => unit_mat = UNI_0,
								1 => unit_mat = UNI_1,
								2 => unit_mat = UNI_2,
								3 => unit_mat = UNI_3,
								4 => unit_mat = UNI_4,
								5 => unit_mat = UNI_5,
								6 => unit_mat = UNI_6,
								7 => unit_mat = UNI_7,
								8 => unit_mat = UNI_8,
								9 => unit_mat = UNI_9,
								_ => unit_mat = UNI_0,
					}
					
					match floorf(angle_board as f32/10.0) as u16 {
								0 => dec_mat = DEC_0,
								1 => dec_mat = DEC_1,
								2 => dec_mat = DEC_2,
								3 => dec_mat = DEC_3,
								4 => dec_mat = DEC_4,
								5 => dec_mat = DEC_5,
								6 => dec_mat = DEC_6,
								7 => dec_mat = DEC_7,
								8 => dec_mat = DEC_8,
								9 => dec_mat = DEC_9,
								_ => dec_mat = DEC_0,
					}
					
					i=0;
					j=3;
					for vali in unit_mat{
						for valj in vali{
								full_lights[i][j] = valj;
								j=j+1;
						}
						i=i+1;
						j=3;
					}
					i=0;
					j=0;
					for vali in dec_mat{
						for valj in vali{
								full_lights[i][j] = valj;
								j=j+1;
						}
						i=i+1;
						j=0;
					}
            display.show(&mut timer, full_lights, 1000);
        }
    }
}

