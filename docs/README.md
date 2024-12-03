# Documentation

Welcome to the documentation!

Take a look at the Navigation table down below 👇

## Navigation

| Category   | Href                      |
| ---------- | ------------------------- |
| Config     | [CONFIG](./CONFIG.md)     |
| Repository | [CONFIG](./REPOSITORY.md) |

## Type reference

| Type       | Description       |
| ---------- | ----------------- |
| string     | String (duh)      |
| string/url | Url/link          |
| [T]        | Array of T        |
| WpSize     | [WpSize](#wpsize) |
| Opt\<T\>     | Optional value    |

### WpSize

The aspect ratio of a wallpaper, take a look at [this for further reading](https://en.wikipedia.org/wiki/Display_aspect_ratio)

**Should be specified as a `string` in toml**

_This is stolen from the wikipedia article, credit to wikipedia!_

Possible values:

| Ratio/Value | Example sizes (px)                                 |
| ----------- | -------------------------------------------------- |
| 1:1         | 1920×1920                                          |
| 5:4         | 1280×1024                                          |
| 4:3         | 1024×768, 1600×1200                                |
| 3:2         | 2160×1440, 2560×1700, 3000x2000, 1500x1000         |
| 8:5         | 1280×800, 1920×1200, 2560x1600, 3840x2400          |
| 16:9        | 1366×768, 1920×1080, 2560x1440, 3840x2160 (4K UHD) |
| 256:135     | 4096×2160                                          |
| 64:27       | 2560×1080, 3440×1440                               |
| 32:9        | 3840×1080, 5120×1440                               |
| 4:1         | 17280×4320                                         |
| 21:9        | 3440×1440, alias for 64:27                         |
