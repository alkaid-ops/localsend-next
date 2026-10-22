import ApplicationServices
import CoreGraphics
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func children(_ element: AXUIElement) -> [AXUIElement] {
    attribute(element, kAXChildrenAttribute as String) as? [AXUIElement] ?? []
}

func frame(_ element: AXUIElement) -> CGRect {
    var origin = CGPoint.zero
    var size = CGSize.zero
    if let position = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(position as! AXValue, .cgPoint, &origin)
    }
    if let bounds = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    }
    return CGRect(origin: origin, size: size)
}

func smallest(_ element: AXUIElement, containing point: CGPoint, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    var match: AXUIElement?
    let bounds = frame(element)
    if bounds.width > 0, bounds.height > 0, bounds.contains(point) {
        match = element
    }
    for child in children(element) {
        if let candidate = smallest(child, containing: point, depth: depth + 1) {
            let candidateArea = frame(candidate).width * frame(candidate).height
            let currentArea = match.map { frame($0).width * frame($0).height } ?? .greatestFiniteMagnitude
            if candidateArea <= currentArea { match = candidate }
        }
    }
    return match
}

guard AXIsProcessTrusted(), (4...5).contains(CommandLine.arguments.count),
      let pid = pid_t(CommandLine.arguments[1]),
      let x = Double(CommandLine.arguments[2]),
      let y = Double(CommandLine.arguments[3])
else {
    fputs("usage: macos-ax-press-at.swift PID X Y [press|focus]\n", stderr)
    exit(64)
}

let mode = CommandLine.arguments.count == 5 ? CommandLine.arguments[4] : "press"
let app = AXUIElementCreateApplication(pid)
let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement] ?? []
guard let element = smallest(app, containing: CGPoint(x: x, y: y)) ?? windows.first else {
    fputs("no AX element at \(x),\(y)\n", stderr)
    exit(1)
}
if mode == "focus" || mode.hasPrefix("set:") {
    let result = AXUIElementSetAttributeValue(element, kAXFocusedAttribute as CFString, kCFBooleanTrue)
    guard result == .success else {
        fputs("AXFocus failed at \(x),\(y): \(result.rawValue)\n", stderr)
        exit(1)
    }
    if mode.hasPrefix("set:") {
        let value = String(mode.dropFirst(4))
        let result = AXUIElementSetAttributeValue(element, kAXValueAttribute as CFString, value as CFTypeRef)
        guard result == .success else {
            fputs("AXValue failed at \(x),\(y): \(result.rawValue)\n", stderr)
            exit(1)
        }
        print("set AX value at \(x),\(y) frame=\(frame(element))")
    } else {
        print("focused AX element at \(x),\(y) frame=\(frame(element))")
    }
} else {
    if AXUIElementPerformAction(element, kAXPressAction as CFString) != .success {
        fputs("AXPress failed at \(x),\(y)\n", stderr)
        exit(1)
    }
    print("pressed AX element at \(x),\(y) frame=\(frame(element))")
}
