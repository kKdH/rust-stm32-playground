#![no_main]
#![no_std]
#![allow(dead_code)]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::gpio::Edge;
use stm32f4xx_hal::prelude::*;

#[allow(unused_imports)]
use panic_halt as _;
use stm32f4xx_hal::pac::TIM1;
use stm32f4xx_hal::timer::DelayUs;

mod ms8607 {
    use rtt_target::rprintln;
    use stm32f4xx_hal::i2c::I2c;
    use stm32f4xx_hal::pac::{I2C1, TIM1};
    use stm32f4xx_hal::prelude::*;
    use stm32f4xx_hal::timer::DelayUs;

    pub const PT_ADDRESS: u8 = 0x76;

    pub const PT_COMMAND_RESET: u8 = 0x1E;
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_256: u8 = 0x40;       // ADC time 0.56ms
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_512: u8 = 0x42;       // ADC time 1.10ms
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_1024: u8 = 0x44;      // ADC time 2.17ms
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_2048: u8 = 0x46;      // ADC time 4.30ms
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_4096: u8 = 0x48;      // ADC time 8.61ms
    pub const PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_8192: u8 = 0x4A;      // ADC time 17.2ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_256: u8 = 0x50;    // ADC time 0.56ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_512: u8 = 0x52;    // ADC time 1.10ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_1024: u8 = 0x54;   // ADC time 2.17ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_2048: u8 = 0x56;   // ADC time 4.30ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_4096: u8 = 0x58;   // ADC time 8.61ms
    pub const PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_8192: u8 = 0x5A;   // ADC time 17.2ms
    pub const PT_COMMAND_READ_VALUE: u8 = 0x00;
    pub const PT_COMMAND_PROM_READ_CRC: u8 = 0xA0;
    pub const PT_COMMAND_PROM_READ_PRESSURE_SENSITIVITY: u8 = 0xA2;
    pub const PT_COMMAND_PROM_READ_PRESSURE_OFFSET: u8 = 0xA4;
    pub const PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_PRESSURE_SENSITIVITY: u8 = 0xA6;
    pub const PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_PRESSURE_OFFSET: u8 = 0xA8;
    pub const PT_COMMAND_PROM_READ_REFERENCE_TEMPERATURE: u8 = 0xAA;
    pub const PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_TEMPERATURE: u8 = 0xAC;

    pub const RH_ADDRESS: u8 = 0x40;

    pub const RH_COMMAND_RESET: u8 = 0xFE;
    pub const RH_COMMAND_WRITE_USER_REGISTER: u8 = 0xE6;
    pub const RH_COMMAND_READ_USER_REGISTER: u8 = 0xE7;
    pub const RH_COMMAND_MEASURE_HUMIDITY_HOLD: u8 = 0xE5;
    pub const RH_COMMAND_MEASURE_HUMIDITY_NO_HOLD: u8 = 0xF5;
    pub const RH_COMMAND_PROM_READ_0: u8 = 0xA0;
    pub const RH_COMMAND_PROM_READ_1: u8 = 0xA2;
    pub const RH_COMMAND_PROM_READ_2: u8 = 0xA4;
    pub const RH_COMMAND_PROM_READ_3: u8 = 0xA6;
    pub const RH_COMMAND_PROM_READ_4: u8 = 0xA8;
    pub const RH_COMMAND_PROM_READ_5: u8 = 0xAA;
    pub const RH_COMMAND_PROM_READ_6: u8 = 0xAC;

    #[derive(Debug, Default)]
    pub struct CalibrationData {
        pub pressure_sensitivity: i64,
        pub pressure_offset: i64,
        pub temperature_coefficient_pressure_sensitivity: i64,
        pub temperature_coefficient_pressure_offset: i64,
        pub reference_temperature: i64,
        pub temperature_coefficient_temperature: i64,
    }


    pub fn reset_pt(i2c: &mut I2c<I2C1>) {
        let result = i2c.write(PT_ADDRESS, &[PT_COMMAND_RESET]);
        rprintln!("Reset PT: {:?}", result);
    }

    pub fn reset_rh(i2c: &mut I2c<I2C1>) {
        let result = i2c.write(RH_ADDRESS, &[RH_COMMAND_RESET]);
        rprintln!("Reset RH: {:?}", result);
    }

    pub fn read_pt_calibration_data(i2c: &mut I2c<I2C1>) -> CalibrationData {

        let mut calibration_data = CalibrationData::default();
        let mut buffer = [0u8; 2];

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_PRESSURE_SENSITIVITY], &mut buffer);
        calibration_data.pressure_sensitivity = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM pressure sensitivity: {:?}", result);

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_PRESSURE_OFFSET], &mut buffer);
        calibration_data.pressure_offset = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM pressure offset: {:?}", result);

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_PRESSURE_SENSITIVITY], &mut buffer);
        calibration_data.temperature_coefficient_pressure_sensitivity = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM temperature coefficient pressure sensitivity: {:?}", result);

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_PRESSURE_OFFSET], &mut buffer);
        calibration_data.temperature_coefficient_pressure_offset = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM temperature coefficient pressure offset: {:?}", result);

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_REFERENCE_TEMPERATURE], &mut buffer);
        calibration_data.reference_temperature = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM reference temperature: {:?}", result);

        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_PROM_READ_TEMPERATURE_COEFFICIENT_TEMPERATURE], &mut buffer);
        calibration_data.temperature_coefficient_temperature = u16::from_be_bytes(buffer) as i64;
        rprintln!("Read PT PROM temperature coefficient temperature: {:?}", result);

        calibration_data
    }

    pub fn initiate_pressure_conversion(i2c: &mut I2c<I2C1>) {
        let result = i2c.write(PT_ADDRESS, &[PT_COMMAND_INITIATE_PRESSURE_CONVERSION_OSR_8192]);
        rprintln!("Initiate pressure conversion: {:?}", result);
    }

    pub fn initiate_temperature_conversion(i2c: &mut I2c<I2C1>) {
        let result = i2c.write(PT_ADDRESS, &[PT_COMMAND_INITIATE_TEMPERATURE_CONVERSION_OSR_8192]);
        rprintln!("Initiate temperature conversion: {:?}", result);
    }

    pub fn read_pt_value(i2c: &mut I2c<I2C1>) -> u32 {
        let mut buffer = [0u8; 3];
        let result = i2c.write_read(PT_ADDRESS, &[PT_COMMAND_READ_VALUE], &mut buffer);
        rprintln!("Read PT value: {:?}", result);

        ((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32)
    }

    pub fn compensate_temperature(raw_temperature: u32, calibration_data: &CalibrationData) -> f32 {
        let delta = raw_temperature as i64 - (calibration_data.reference_temperature << 8);
        let temperature_1 = 2000 + (delta * calibration_data.temperature_coefficient_temperature >> 23);
        let temperature_2 = if temperature_1 < 2000 {
            (3 * delta * delta) >> 33
        }
        else {
            (5 * delta * delta) >> 38
        };
        (temperature_1 - temperature_2) as f32 / 100_f32
    }

    pub fn compensate_pressure(raw_pressure: u32, raw_temperature: u32, calibration_data: &CalibrationData) -> f32 {
        let temperature_delta = raw_temperature as i64 - (calibration_data.reference_temperature << 8);

        let offset_1 = (calibration_data.pressure_offset << 17) + ((calibration_data.temperature_coefficient_pressure_offset * temperature_delta) >> 6);
        let sensitivity_1 = (calibration_data.pressure_sensitivity << 16) + ((calibration_data.temperature_coefficient_pressure_sensitivity * temperature_delta) >> 7);
        let pressure = (((raw_pressure as i64) * sensitivity_1) >> 21 - offset_1) >> 15;

        // rprintln!(r#"{{"raw_pressure": {}, "raw_temperature": {}, "c2": {}, "offset": {}, "c1": {}, "sensitivity": {}, "pressure": {}}}"#,
        //     raw_pressure, raw_temperature, calibration_data.pressure_offset, offset_1, calibration_data.pressure_sensitivity, sensitivity_1, pressure);

        pressure as f32 / 100_f32
    }

    pub fn measure_humidity(i2c: &mut I2c<I2C1>, delay: &mut DelayUs<TIM1>) -> f32 {
        let mut buffer = [0_u8; 3];
        let result = i2c.write(RH_ADDRESS, &[RH_COMMAND_MEASURE_HUMIDITY_NO_HOLD]);
        rprintln!("Starting humidity measurement: {:?}", result);

        while let Err(_) = i2c.read(RH_ADDRESS, &mut buffer) {
            delay.delay_ms(10_u16)
        }

        rprintln!("Stopped humidity measurement.");

        let raw_humidity = (((buffer[0] as u32) << 8) | (buffer[1] as u32)) as i64;
        let humidity = raw_humidity * 125 / (1 << 16) - 6;

        humidity as f32
    }

    pub fn compensate_humidity(temperature: f32, humidity: f32) -> f32 {
        humidity + (25_f32 - temperature) * -0.15_f32
    }
}

#[entry]
fn main() -> ! {

    let _core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut device_peripherals = pac::Peripherals::take().unwrap();

    rtt_init_print!();

    let mut sys_cfg = device_peripherals.SYSCFG.constrain();
    let rcc = device_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let gpio_b = device_peripherals.GPIOB.split();
    let gpio_c = device_peripherals.GPIOC.split();

    let mut i2c = {
        let scl = gpio_b.pb8.into_alternate_open_drain::<4>();
        let sda = gpio_b.pb9.into_alternate_open_drain::<4>();
        device_peripherals.I2C1.i2c((scl, sda), 400.kHz(), &clocks)
    };

    let mut button = gpio_c.pc13.into_pull_up_input();
    button.make_interrupt_source(&mut sys_cfg);
    button.enable_interrupt(&mut device_peripherals.EXTI);
    button.trigger_on_edge(&mut device_peripherals.EXTI, Edge::Rising);

    rprintln!("Initialized");

    let mut delay: DelayUs<TIM1> = device_peripherals.TIM1.delay_us(&clocks);

    ms8607::reset_pt(&mut i2c);
    ms8607::reset_rh(&mut i2c);

    let calibration_data = ms8607::read_pt_calibration_data(&mut i2c);

    rprintln!("Calibration data: {:?}", calibration_data);

    loop {
        ms8607::initiate_temperature_conversion(&mut i2c);

        delay.delay_ms(20_u16); // await ADC

        let raw_temperature = ms8607::read_pt_value(&mut i2c);

        ms8607::initiate_pressure_conversion(&mut i2c);

        delay.delay_ms(20_u16); // await ADC

        let raw_pressure = ms8607::read_pt_value(&mut i2c);

        let temperature = ms8607::compensate_temperature(raw_temperature, &calibration_data);
        let pressure = ms8607::compensate_pressure(raw_pressure, raw_temperature, &calibration_data);

        let humidity = ms8607::compensate_humidity(temperature, ms8607::measure_humidity(&mut i2c, &mut delay));

        rprintln!("Temperature: {:.2} °C, Pressure: {:.2} mbar, Humidity: {:.2} %RH", temperature, pressure, humidity);

        delay.delay_ms(5000_u16);
    }
}
