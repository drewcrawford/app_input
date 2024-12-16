//
//  PlatformCoalescedMouse.swift
//  SwiftRawInput
//
//  Created by Drew Crawford on 12/16/24.
//
import AppKit
import SwiftRawInputRustBindings

func convertToRustCoordinates(absolutePoint: NSPoint) -> (x: Double, y: Double) {
    let minX = NSScreen.screens.map({$0.frame.minX}).min() ?? 0
    let minY = NSScreen.screens.map({$0.frame.maxY}).max() ?? 0
    //flip to upper left coordinate system
    return (x: absolutePoint.x - minX, y: minY - absolutePoint.y)
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
        
        self.monitor = NSEvent.addLocalMonitorForEvents(matching: [.mouseMoved, .leftMouseUp, .otherMouseUp, .rightMouseUp,.scrollWheel]) { event in
            switch event.type {
            case .mouseMoved:

                let location = event.locationInWindow
                if let window = event.window {
                    MainActor.assumeIsolated {
                        //treat location as relative coordinates
//                        let windowPosInScreen = window.convertPoint(toScreen: location)
//                        let absolutePos = location
//                        let screen = window.convert
                        let cocoaPos = NSPoint(x: window.frame.origin.x + location.x, y: window.frame.origin.y + location.y)
                        
//                        let absolutePos = NSPoint(x: windowPosInScreen.x + location.x, y: windowPosInScreen.y + location.y)
                        let recvContext = UnsafeMutableRawPointer(bitPattern: sendContext)
                        let rustCoords = convertToRustCoordinates(absolutePoint: cocoaPos)
                        raw_input_mouse_move(recvContext, rustCoords.x, rustCoords.y)
                        
                    }
                    
                }
                else {
                    //treat location as absolute coordinates
                    
                    let rustcoords = convertToRustCoordinates(absolutePoint: location)
                    raw_input_mouse_move(context, rustCoords.x, rustCoords.y)
                    
                }
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
