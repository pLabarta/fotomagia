# Errors and solutions

```
Could not claim the USB device', src/main.rs:5:61
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`px aux | grep gphoto` -> `kill -9 <pid>`

# Setting configs

Configs must be set before capturing image, else they will be ignored in the capture but be set for the next capture.
`gphoto2 --set-config capturetarget=1` (SD card)

When camera is in Automatic mode (P), the aperture and shutter speed are set automatically by the camera thus are READ ONLY. Camera MUST BE in MANUAL MODE to set aperture and shutter speed.

# Capture image

`gphoto2 --capture-image-and-download --filename --force-overwrite`

# Capture mode

Type: RADIO
Current: Burst
Choice: 0 Single Shot
Choice: 1 Burst
Choice: 2 Timer
Choice: 3 Quick Response Remote
Choice: 4 Delayed Remote
Choice: 5 Quiet Release

Usage: `gphoto2 --set-config capturemode=0` (Single Shot)
