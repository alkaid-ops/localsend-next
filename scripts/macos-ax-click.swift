import ApplicationServices
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

func find(_ element: AXUIElement, label: String, role: String? = nil, depth: Int = 0) -> AXUIElement? {
    if depth > 30 { return nil }
    let roleMatches = role.map { string(element, kAXRoleAttribute as String) == $0 } ?? true
    for key in [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute] {
        if roleMatches && string(element, key as String)?.contains(label) == true { return element }
    }
    for child in children(element) {
        if let match = find(child, label: label, role: role, depth: depth + 1) { return match }
    }
    return nil
}

func findRole(_ element: AXUIElement, role wanted: String, depth: Int = 0) -> AXUIElement? {
    if depth > 30 { return nil }
    if string(element, kAXRoleAttribute as String) == wanted { return element }
    for child in children(element) {
        if let match = findRole(child, role: wanted, depth: depth + 1) { return match }
    }
    return nil
}

func frame(_ element: AXUIElement) -> CGRect {
    var point = CGPoint.zero
    var size = CGSize.zero
    if let value = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgPoint, &point)
    }
    if let value = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgSize, &size)
    }
    return CGRect(origin: point, size: size)
}

guard AXIsProcessTrusted(), CommandLine.arguments.count >= 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-click.swift PID LABEL [set:VALUE|text:LABEL]")
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
AXUIElementSetAttributeValue(app, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
AXUIElementSetAttributeValue(app, "AXManualAccessibility" as CFString, kCFBooleanTrue)
if let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement],
   let window = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height })
{
    _ = AXUIElementPerformAction(window, kAXRaiseAction as CFString)
}
usleep(200_000)
let mode = CommandLine.arguments.count >= 4 ? CommandLine.arguments[3] : nil
if let mode, mode == "focus" || mode.hasPrefix("set:") {
    guard let field = findRole(app, role: kAXTextFieldRole as String) else {
        print("text field not found")
        exit(1)
    }
    _ = AXUIElementSetAttributeValue(field, kAXFocusedAttribute as CFString, kCFBooleanTrue)
    guard mode.hasPrefix("set:") else { exit(0) }
    let result = AXUIElementSetAttributeValue(field, kAXValueAttribute as CFString, String(mode.dropFirst(4)) as CFTypeRef)
    exit(result == .success ? 0 : 1)
}
guard let element = find(app, label: CommandLine.arguments[2]) else {
    print("element not found")
    exit(1)
}
if AXUIElementPerformAction(element, kAXPressAction as CFString) == .success {
    exit(0)
}
let bounds = frame(element)
let point = CGPoint(x: bounds.midX, y: bounds.midY)
CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: point, mouseButton: .left)?.post(tap: .cghidEventTap)
CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: point, mouseButton: .left)?.post(tap: .cghidEventTap)
