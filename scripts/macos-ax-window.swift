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

guard AXIsProcessTrusted(), CommandLine.arguments.count == 6,
      let pid = pid_t(CommandLine.arguments[1]),
      let x = Double(CommandLine.arguments[2]),
      let y = Double(CommandLine.arguments[3]),
      let width = Double(CommandLine.arguments[4]),
      let height = Double(CommandLine.arguments[5])
else {
    print("usage: macos-ax-window.swift PID X Y WIDTH HEIGHT")
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement] ?? []
guard let window = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height }) else {
    print("no window")
    exit(1)
}
var origin = CGPoint(x: x, y: y)
var size = CGSize(width: width, height: height)
let position = AXValueCreate(.cgPoint, &origin)!
let bounds = AXValueCreate(.cgSize, &size)!
guard AXUIElementSetAttributeValue(window, kAXPositionAttribute as CFString, position) == .success,
      AXUIElementSetAttributeValue(window, kAXSizeAttribute as CFString, bounds) == .success
else {
    print("failed to set window frame")
    exit(1)
}
print("moved to \(frame(window))")
