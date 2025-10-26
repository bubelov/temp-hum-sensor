pub struct Sht3x<I2C> {
    pub i2c: I2C,
    address: u8,
}

// https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf
impl<I2C> Sht3x<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub const CMD_MEASURE: [u8; 2] = [0x2C, 0x06];

    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn measure(&mut self) -> Sht3xMeasurement {
        self.i2c
            .write(self.address, &Self::CMD_MEASURE)
            .expect("failed to request measurement");
        let mut data = [0u8; 6];
        self.i2c
            .read(self.address, &mut data)
            .expect("failed to read measurement");
        let temp_raw = u16::from_be_bytes([data[0], data[1]]);
        let hum_raw = u16::from_be_bytes([data[3], data[4]]);
        let temp_celsius = -45.0 + (175.0 * temp_raw as f32 / 65535.0);
        let humidity_percent = 100.0 * hum_raw as f32 / 65535.0;
        Sht3xMeasurement {
            temp_celsius,
            humidity_percent,
        }
    }
}

pub struct Sht3xMeasurement {
    pub temp_celsius: f32,
    pub humidity_percent: f32,
}
