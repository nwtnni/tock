# tock

A digital clock for the terminal, inspired by [tty-clock][0].
Unfortunately only works on UNIX systems due to the use of
the `termios` and `sigaction` related function calls.

# Features

- Efficient bitmap and diff-based drawing
- Timezone support via the `TZ` environment variable and `tzselect` utility
- Military time and second display toggling
- Color customization using ANSI, 8-bit, or 24-bit color values
- Positioned or centered clock
- Adjustable display size
- Synchronization with system clock seconds

# Screenshots

![Screenshot of clock](./resources/tock.png)

![asciicast of multiple clocks](./resources/world.gif)

[0]: https://github.com/xorg62/tty-clock
