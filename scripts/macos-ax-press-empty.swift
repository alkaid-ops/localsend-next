import ApplicationServices
import CoreGraphics
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func string(_ element: AXUIElement, _ name: String) -> String? {
    attribute(element, name) as? String
}

func children(_ element: AXUIElement) -> [AXUIElement] {
    attribute(element, kAXChildrenAttribute as String) as? [AXUIElement] ?? []
}

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
}

func contains(_ element: AXUIElement, _ needle: String, depth: Int = 0) -> Bool {
    guard depth <= 30 else { return false }
    if labels(element).contains(where: { $0.localizedCaseInsensitiveContains(needle) }) {
        return true
    }
    return children(element).contains { contains($0, needle, depth: depth + 1) }
}

func firstEmptyButton(_ element: AXUIElement, depth: Int = 0) -> AXUIElement? {
    guard depth <= 30 else { return nil }
    if string(element, kAXRoleAttribute as String) == kAXButtonRole as String,
       labels(element).isEmpty
    {
        return element
    }
    for child in children(element) {
        if let button = firstEmptyButton(child, depth: depth + 1) { return button }
    }
    return nil
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

guard AXIsProcessTrusted(), CommandLine.arguments.count == 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    fputs("usage: macos-ax-press-empty.swift PID ANCESTOR_TEXT\n", stderr)
    exit(64)
}

let needle = CommandLine.arguments[2]
let app = AXUIElementCreateApplication(pid)
let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement] ?? []
guard let window = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height }),
      let selection = children(window).first(where: { contains($0, needle) }),
      let button = firstEmptyButton(selection)
else {
    fputs("empty button not found below \(needle)\n", stderr)
    exit(1)
}

if AXUIElementPerformAction(button, kAXPressAction as CFString) != .success {
    let point = CGPoint(x: frame(button).midX, y: frame(button).midY)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: point, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: point, mouseButton: .left)?
        .post(tap: .cghidEventTap)
}
print("pressed empty button below \(needle) at \(frame(button))")
