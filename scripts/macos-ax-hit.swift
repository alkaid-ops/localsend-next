import ApplicationServices
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func string(_ element: AXUIElement, _ name: String) -> String? {
    attribute(element, name) as? String
}

func describe(_ element: AXUIElement) -> String {
    let role = string(element, kAXRoleAttribute as String) ?? ""
    let values = [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
    return "\(role) \(values.joined(separator: " | "))"
}

guard AXIsProcessTrusted(), (3...4).contains(CommandLine.arguments.count),
      let x = Double(CommandLine.arguments[1]),
      let y = Double(CommandLine.arguments[2])
else {
    print("usage: macos-ax-hit.swift X Y")
    exit(64)
}

var element: AXUIElement?
let result = AXUIElementCopyElementAtPosition(
    AXUIElementCreateSystemWide(),
    Float(x),
    Float(y),
    &element
)
guard result == .success, var current = element else {
    print("hit test failed: \(result.rawValue)")
    exit(1)
}
if CommandLine.arguments.count == 4, CommandLine.arguments[3] == "press" {
    var candidate: AXUIElement? = current
    var pressed = false
    for _ in 0..<6 {
        guard let element = candidate else { break }
        let result = AXUIElementPerformAction(element, kAXPressAction as CFString)
        if result == .success {
            pressed = true
            break
        }
        candidate = attribute(element, kAXParentAttribute as String).map {
            unsafeBitCast($0, to: AXUIElement.self)
        }
    }
    guard pressed else {
        print("press failed")
        exit(1)
    }
} else if CommandLine.arguments.count == 4, CommandLine.arguments[3] == "click" {
    var origin = CGPoint.zero
    var size = CGSize.zero
    if let value = attribute(current, kAXPositionAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgPoint, &origin)
    }
    if let value = attribute(current, kAXSizeAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgSize, &size)
    }
    let point = size == .zero ? CGPoint(x: x, y: y) : CGPoint(x: origin.x + size.width / 2, y: origin.y + size.height / 2)
    let down = CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: point, mouseButton: .left)
    let up = CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: point, mouseButton: .left)
    down?.post(tap: .cghidEventTap)
    up?.post(tap: .cghidEventTap)
}
for depth in 0..<12 {
    var actionNames: CFArray?
    let actions = AXUIElementCopyActionNames(current, &actionNames) == .success
        ? (actionNames as? [String] ?? [])
        : []
    print("[\(depth)] \(describe(current)) actions=\(actions.joined(separator: ","))")
    guard let parent = attribute(current, kAXParentAttribute as String) else { break }
    current = unsafeBitCast(parent, to: AXUIElement.self)
}
