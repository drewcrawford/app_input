app_input is a cross-platform library for receiving keyboard and mouse events.

![logo](art/logo.png)

# Design principles

* Use the best modern backend on each platform
* Zero magic, this library can be easily used without interference alongside any other native code.
    * When native code may interfere, instead this library will be a no-op by default.  You must
      call into it from your eventloop before our events are delivered.
* Mouse events:
    * Mouse position is determined by the compositor.  Platform-specific acceleration will be applied.
        * This is appropriate for GUI apps and topdown strategy games, or anytime you have a system-drawn cursor.
          It is appropriate for some fullscreen games. It is not appropriate for Counter Strike.
    * Coordinates are translated into a platform-independent upper-left coordinate system that works everywhere
    * Mouse events may require the window to be 'active' to be delivered, per platform conventions
* Keyboard events:
    * Report raw up/down events whenever possible
        * We map hardware keys rather than unicode characters
        * If you are trying to implement text input, you have much work to do, including but not limited to the shift key
    * Keycodes are translated into a platform-independent enum that works everywhere
    * On Linux, key events are broadcasted over ATSPI.  Due to some [questionable decisions in the Linux ecosystem](https://github.com/AccessKit/accesskit/discussions/503#discussioncomment-11862133)
        this is required for screenreaders to work but nobody does it.  We do!


# Supported platforms
| Platform | Backend                  |
|----------|--------------------------|
| Windows  | win32*                   |
| macOS    | AppKit                   |
| Linux    | Wayland*                 |
| wasm32   | KeyboardEvent \| MouseEvent  |
| Yours    | Send a PR!               |


* `*`: Needs platform-native event integration before events are delivered