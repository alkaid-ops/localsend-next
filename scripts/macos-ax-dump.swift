import AppKit
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

func rect(_ element: AXUIElement) -> String {
    var point = CGPoint.zero
    var size = CGSize.zero
    if let position = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(position as! AXValue, .cgPoint, &point)
    }
    if let bounds = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    }
    return String(format: "%.0f,%.0f %.0fx%.0f", point.x, point.y, size.width, size.height)
}

guard AXIsProcessTrusted() else {
    print("BLOCKED: enable Accessibility permission")
    exit(2)
}
guard CommandLine.arguments.count >= 2, let pid = pid_t(CommandLine.arguments[1]) else {
    print("usage: macos-ax-dump.swift PID [filter]")
    exit(64)
}

let filter = CommandLine.arguments.count >= 3 ? CommandLine.arguments[2].lowercased() : nil
let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateIgnoringOtherApps])
AXUIElementSetAttributeValue(app, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
AXUIElementSetAttributeValue(app, "AXManualAccessibility" as CFString, kCFBooleanTrue)
usleep(300_000)

func dump(_ element: AXUIElement, depth: Int = 0) {
    if depth > 20 { return }
    let role = string(element, kAXRoleAttribute as String) ?? ""
    let values = [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute, kAXHelpAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
    let line = "[\(depth)] \(role) \(rect(element)) \(values.joined(separator: " | "))"
    if filter.map({ line.lowercased().contains($0) }) ?? true {
        print(line)
    }
    for child in children(element) {
        dump(child, depth: depth + 1)
    }
}

if let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement] {
    for window in windows {
        dump(window)
    }
} else {
    dump(app)
}
