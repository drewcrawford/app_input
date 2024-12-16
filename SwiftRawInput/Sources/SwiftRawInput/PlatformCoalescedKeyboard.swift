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

final class PlatformCoalescedKeyboard:
    /*Rust type implements send/sync
     **/
    Sendable
{
    init(notifyFunc: KeyNotifyFunc) {
        MainActor.shared.dispatchMainThreadFromRustContextDetached {
            NSApplication.shared.setActivationPolicy(.regular)
        }
        NSEvent.addLocalMonitorForEvents(matching: [.keyDown]) { event in
            notifyFunc()
            return event
        }
    }
}

public typealias KeyNotifyFunc = @convention(c) () -> ()

@_cdecl("PlatformCoalescedKeyboardNew") public func PlatformCoalescedKeyboardNew(keyNotify: KeyNotifyFunc) -> UnsafeMutableRawPointer {
    let p = PlatformCoalescedKeyboard(notifyFunc: keyNotify)
    return Unmanaged.passRetained(p).toOpaque()
}

@_cdecl("PlatformCoalescedKeyboardFree") public func PlatformCoalescedKeyboardFree(_ p: UnsafeMutableRawPointer) {
    Unmanaged<PlatformCoalescedKeyboard>.fromOpaque(p).release()
}
