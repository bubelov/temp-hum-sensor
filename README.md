# Temperature and Humidity Sensor

## Hardware:

- [XIAO ESP32C6](https://wiki.seeedstudio.com/xiao_esp32c6_getting_started/#hardware-overview)
- [Expansion Board Base for XIAO](https://wiki.seeedstudio.com/Seeeduino-XIAO-Expansion-Board/)
- [SHT3X](https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf)

## Wiring

`ESP`:`VBUS` ----- `VIN`:`SHT3X`

`ESP`:`GND` ----- `GND`:`SHT3X`

`ESP`:`SDA` ----- `SDA`:`SHT3X`

`ESP`:`SCL` ----- `SCL`:`SHT3X`

## Flashing

```bash
cargo r --release
```

Consult [Rust on ESP Book](https://docs.espressif.com/projects/rust/book/preface.html) for more info on dev env setup, if needed.

## Current Behaviour

You are expected to see the readings in your console:

```
INFO - initializing led
INFO - led init complete
INFO - initializing i2c
INFO - i2c init complete
INFO - temp: 25.10147 | hum: 31.94934
INFO - temp: 25.074768 | hum: 31.961548
INFO - temp: 25.10147 | hum: 31.94934
INFO - temp: 25.08812 | hum: 31.976807
```

## Roadmap

- Design a 3D printable case