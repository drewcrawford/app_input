//
//  DetectMainThreadPolicy.swift
//  SwiftRawInput
//
//  Created by Drew Crawford on 12/15/24.
//

/*
 Utilities to interact with Apple threads from Rust.
 
 In general, we don't want to assume we're on the main thread, we might be called from an arbitrary thread.
 And it is easier to solve this problem on the Swift side (with easy access to dispatch, etc.)
 
 However if we're NOT on the main thread, it's possible the MT is blocked.  And so we probably want to error in this case, rather than block indefinitely.
 */
import Foundation

extension MainActor {
    /**
    Runs the attached function on the main thread.
     
     1.  If we are already on the main thread, this runs inline.
     2.  If we are not on the main thread, dispatch onto the main thread, detaching from the operation and without waiting for results.
     3.  If the main thread seems to be blocked, crash.
     */
    nonisolated func dispatchMainThreadFromRustContextDetached(_ operation: @MainActor @escaping @Sendable () -> ()) {
        if Thread.current.isMainThread {
            MainActor.assumeIsolated(operation)
        }
        else {
            let sema = DispatchSemaphore(value: 0)
            Task {
                await MainActor.run {
                    sema.signal() //mt is unblocked
                    operation()
                }
            }
            Task.detached(priority: .low) {
                precondition(sema.wait(timeout: .now() + 1) == .success, "Main thread is blocked")
            }
            
        }
        
    }

}
