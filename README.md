# SSD1306 driver

Currently only implemented for the 128x64 4-wire SPI protocol OLED module commonly found on eBay and the like.

This is a huge WIP. The example just dumps a test pattern to the screen at time of writing, just to prove it's working. More to follow...

## TODO

- [x] 8BPP image drawing
- [x] 1BPP image drawing
- [x] 8px high fixed width pixel font
- [x] Line drawing
- [x] Rect drawing
- [x] Circle drawing
- [ ] Other fonts
- [ ] Refactor font drawing API to take a font struct as input
- [ ] Investigate having one generic `draw()` function that takes any drawable object. Opens doors to custom impls.