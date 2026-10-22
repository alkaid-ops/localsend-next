import ApplicationServices
import CoreGraphics
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func frame(_ element: AXUIElement) -> CGRect? {
    var origin = CGPoint.zero
    var size = CGSize.zero
    guard let position = attribute(element, kAXPositionAttribute as String),
          let bounds = attribute(element, kAXSizeAttribute as String)
    else { return nil }
    AXValueGetValue(position as! AXValue, .cgPoint, &origin)
    AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    return CGRect(origin: origin, size: size)
}

guard AXIsProcessTrusted(),
      CommandLine.arguments.count >= 3,
      let pid = pid_t(CommandLine.arguments[1]),
      let pixels = Int32(CommandLine.arguments[2])
else {
    fputs("usage: macos-ax-scroll.swift PID PIXELS  # negative scrolls down\n", stderr)
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
guard let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement],
      let window = windows.compactMap({ element in
          frame(element).map { (element, $0.width * $0.height) }
      })
          .max(by: { $0.1 < $1.1 })?.0
else {
    fputs("no window found for pid \(pid)\n", stderr)
    exit(1)
}

let bounds = frame(window)!
let point = CGPoint(x: bounds.midX, y: bounds.midY)
CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: point, mouseButton: .left)?
    .post(tap: .cghidEventTap)
usleep(100_000)
CGEvent(scrollWheelEvent2Source: nil, units: .pixel, wheelCount: 1, wheel1: pixels, wheel2: 0, wheel3: 0)?
    .post(tap: .cghidEventTap)
usleep(300_000)
print("scrolled \(pixels)px at \(Int(point.x)),\(Int(point.y))")
