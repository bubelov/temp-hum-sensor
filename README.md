# Temperature and Humidity Sensor

## Hardware:

- [XIAO ESP32C6](https://wiki.seeedstudio.com/xiao_esp32c6_getting_started/#hardware-overview)
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
INFO - temp: 25.424583 | hum: 73.21278
INFO - temp: 25.437935 | hum: 73.153275
INFO - temp: 25.437935 | hum: 73.112076
INFO - temp: 25.437935 | hum: 73.153275
```

The built-in LED should blink every second to indicate healty activity.

## Roadmap

- Add Zigbee support
- Add optional E-Ink display support
- Add a LiFePo4 battery
- Design a 3D printable case