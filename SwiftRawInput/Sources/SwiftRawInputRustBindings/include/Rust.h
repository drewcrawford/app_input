//
//  Rust.h
//  SwiftRawInput
//
//  Created by Drew Crawford on 12/16/24.
//
#include <stdbool.h>
#include <stdint.h>

extern void raw_input_finish_key_event_context(const void* context);
extern void raw_input_finish_mouse_event_context(const void* context);
extern void raw_input_key_notify_func(const void *context, uint16_t keyCode, bool pressed);
extern void raw_input_mouse_move(const void *context, double screenPosX, double screenPosY);

