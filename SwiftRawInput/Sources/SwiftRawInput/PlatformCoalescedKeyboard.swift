//
//  PlatformCoalescedKeyboard.swift
//  SwiftRawInput
//
//  Created by Drew Crawford on 12/13/24.
//

#if canImport(AppKit)
import AppKit
#else
#error("This platform is not yet supported")
#endif
import SwiftRawInputRustBindings

final class PlatformCoalescedKeyboard:
    /*Rust type implements send/sync
     **/
    Sendable
{
    nonisolated(unsafe) let monitor: Any?
    nonisolated(unsafe) let context: UnsafeMutableRawPointer
    
    init(notifyFunc: KeyNotifyFunc, context: UnsafeMutableRawPointer) {
        MainActor.shared.dispatchMainThreadFromRustContextDetached {
            NSApplication.shared.setActivationPolicy(.regular)
        }
        self.context = context
        self.monitor = NSEvent.addLocalMonitorForEvents(matching: [.keyDown, .keyUp, .flagsChanged]) { event in
            switch event.type {
            case .keyDown:
                notifyFunc(context, event.keyCode, true)
            case .keyUp:
                notifyFunc(context, event.keyCode, false)
            case .flagsChanged:
                func notifyModifier(event: NSEvent, flag: NSEvent.ModifierFlags) {
                    if event.modifierFlags.contains(flag) {
                        notifyFunc(context, event.keyCode, true)
                    }
                    else {
                        notifyFunc(context, event.keyCode, false)
                    }
                }

                switch event.keyCode {
                case 0x3B: //control
                    notifyModifier(event: event, flag: .control)
                case 0x3E: //right control
                    notifyModifier(event: event, flag: .control)
                case 0x3A: //option
                    notifyModifier(event: event, flag: .option)
                case 0x3D://right option
                    notifyModifier(event: event, flag: .option)
                case 0x37://command
                    notifyModifier(event: event, flag: .command)
                case 0x36: //right command
                    notifyModifier(event: event, flag: .command)
                case 0x38: //shift
                    notifyModifier(event: event, flag: .shift)
                case 0x3C: //right shift
                    notifyModifier(event: event, flag: .shift)
                case 0x3F: //function
                    notifyModifier(event: event, flag: .function)
                case 0x39: //caps lock
                    notifyModifier(event: event, flag: .capsLock)
                    
                
                default:
                    fatalError("\(event)")
                }
            default:
                fatalError("Unknown event type \(event.type)")
            }
            
            
            return event
        }

    }
    deinit {
        if let monitor {
            NSEvent.removeMonitor(monitor)
        }
        raw_input_finish_event_context(self.context)
        
    }
}

public typealias KeyNotifyFunc = @convention(c) (UnsafeMutableRawPointer, UInt16, Bool) -> ()

@_cdecl("PlatformCoalescedKeyboardNew") public func PlatformCoalescedKeyboardNew(keyNotify: KeyNotifyFunc, context: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let p = PlatformCoalescedKeyboard(notifyFunc: keyNotify, context: context)
    return Unmanaged.passRetained(p).toOpaque()
}

@_cdecl("PlatformCoalescedKeyboardFree") public func PlatformCoalescedKeyboardFree(_ p: UnsafeMutableRawPointer) {
    Unmanaged<PlatformCoalescedKeyboard>.fromOpaque(p).release()
}
