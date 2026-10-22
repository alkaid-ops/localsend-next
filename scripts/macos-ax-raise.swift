import AppKit
import ApplicationServices
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func frame(_ element: AXUIElement) -> CGRect {
    var origin = CGPoint.zero
    var size = CGSize.zero
    if let value = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgPoint, &origin)
    }
    if let value = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgSize, &size)
    }
    return CGRect(origin: origin, size: size)
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 2, let pid = pid_t(CommandLine.arguments[1]) else {
    print("usage: macos-ax-raise.swift PID")
    exit(64)
}
let activated = NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows]) ?? false
print("activated=\(activated)")
let app = AXUIElementCreateApplication(pid)
let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement] ?? []
guard let window = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height }) else {
    print("no window")
    exit(1)
}
let result = AXUIElementPerformAction(window, kAXRaiseAction as CFString)
guard result == .success else {
    print("raise failed: \(result.rawValue)")
    exit(1)
}
usleep(300_000)
