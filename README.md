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


* `*`: Needs platform-native event integration before events are delivered.  Consider using [app_window](https://sealedabstract.com/code/app_window)!

# WASM/JavaScript Support

This library is also available as an npm package for JavaScript/TypeScript projects targeting WebAssembly.
The package provides type definitions and can be used in web applications.

# Debug Features

The library provides debug window functions for testing keyboard input:

* `debug_window_show()` - Shows a debug window for testing keyboard input (macOS only)
* `debug_window_hide()` - Hides the debug window (macOS only)

These functions are useful for debugging keyboard event handling without interference from other applications.

# Window Information

Events include information about the window they were delivered to via the `Window` struct:

* On Windows, this contains an HWND
* On macOS, this is the pointer of an NSWindow (no memory management performed)
* On wasm32, this attaches to the global DOM window with an opaque value
* On Linux, this returns the wayland surface ID (no memory management performed)