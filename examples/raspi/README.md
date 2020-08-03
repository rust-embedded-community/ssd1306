# raspi demo for oled(ssd1306)

in rust

## Quick start

The easiest way to build this sample is with [Cross](https://github.com/rust-embedded/cross). 

```
cross build --release --target armv7-unknown-linux-gnueabihf
```

> That assumes Raspberry Pi 2/3/4 running a 32 bit kernel

After the build finishes, copy it to your Raspberry Pi

```
scp target/armv7-unknown-linux-gnueabihf/release/raspi-oled user@ip:/home/user
```

Then SSH to your Pi and run it

```
sudo ./raspi-oled
```

## Example

![picture](./images/01.jpg)

![primitive](./images/02.jpg)
