//
//  PlatformCoalescedMouse.swift
//  SwiftRawInput
//
//  Created by Drew Crawford on 12/16/24.
//
import AppKit
import SwiftRawInputRustBindings

func convertToRustCoordinates(absolutePoint: NSPoint, minX: Double, maxY: Double) -> (x: Double, y: Double) {
    //flip to upper left coordinate system
    return (x: absolutePoint.x - minX, y: maxY - absolutePoint.y)
}

final class PlatformCoalescedMouse:
    /*Rust type implements send/sync
     **/
    Sendable
{
    nonisolated(unsafe) let monitor: Any?
    nonisolated(unsafe) let context: UnsafeMutableRawPointer
    
    init(context: UnsafeMutableRawPointer) {
        MainActor.shared.dispatchMainThreadFromRustContextDetached {
            NSApplication.shared.setActivationPolicy(.regular)
        }
        self.context = context
        
        let sendContext = Int(bitPattern: context)
        
        self.monitor = NSEvent.addLocalMonitorForEvents(matching: [.mouseMoved, .leftMouseDown, .leftMouseUp, .otherMouseDown, .otherMouseUp, .rightMouseDown, .rightMouseUp,.scrollWheel]) { event in
            switch event.type {
            case .mouseMoved:

                let minScreenX = NSScreen.screens.map({$0.frame.minX}).min() ?? 0
                let maxScreenY = NSScreen.screens.map({$0.frame.maxY}).max() ?? 0
                let location = event.locationInWindow
                if let window = event.window {
                    MainActor.assumeIsolated {
                        let cocoaPos = NSPoint(x: window.frame.origin.x + location.x, y: window.frame.origin.y + location.y)
                        let recvContext = UnsafeMutableRawPointer(bitPattern: sendContext)
                        let absRustCoords = convertToRustCoordinates(absolutePoint: cocoaPos, minX: minScreenX, maxY: maxScreenY)
                        let windowRustCoords = convertToRustCoordinates(absolutePoint: location, minX: 0, maxY: window.frame.size.height)
                        raw_input_mouse_move(recvContext, absRustCoords.x, absRustCoords.y, Unmanaged.passUnretained(window).toOpaque(), windowRustCoords.x, windowRustCoords.y, window.frame.size.width, window.frame.size.height)
                    }
                    
                }
                else {
                    //treat location as absolute coordinates
                    
                    let rustCoords = convertToRustCoordinates(absolutePoint: location, minX: minScreenX, maxY: maxScreenY)
                    raw_input_mouse_move(context, rustCoords.x, rustCoords.y, nil, 0, 0, 0,0)
                    
                }
            case .leftMouseDown:
                raw_input_mouse_button(context, 0, true)
            case .leftMouseUp:
                raw_input_mouse_button(context, 0, false)
            case .rightMouseDown:
                raw_input_mouse_button(context, 1, true)
            case .rightMouseUp:
                raw_input_mouse_button(context, 1, false)
            case .otherMouseDown:
                raw_input_mouse_button(context, UInt8(event.buttonNumber), true)
            case .otherMouseUp:
                raw_input_mouse_button(context, UInt8(event.buttonNumber), false)
            
            default:
                fatalError("\(event)")
            }
            return event
        }

    }
    deinit {
        if let monitor {
            NSEvent.removeMonitor(monitor)
        }
        raw_input_finish_mouse_event_context(self.context)
        
    }
}

public typealias MouseNotifyFunc = @convention(c) (UnsafeMutableRawPointer) -> ()

@_cdecl("PlatformCoalescedMouseNew") public func PlatformCoalescedMouseNew(context: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {

    let p = PlatformCoalescedMouse(context: context)
    return Unmanaged.passRetained(p).toOpaque()
}

@_cdecl("PlatformCoalescedMouseFree") public func PlatformCoalescedMouseFree(_ p: UnsafeMutableRawPointer) {
    Unmanaged<PlatformCoalescedMouse>.fromOpaque(p).release()
}
