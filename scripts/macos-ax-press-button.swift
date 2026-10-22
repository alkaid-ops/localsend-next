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

func collect(_ element: AXUIElement, label: String, into result: inout [AXUIElement], depth: Int = 0) {
    if depth > 30 { return }
    if string(element, kAXRoleAttribute as String) == kAXButtonRole as String {
        for key in [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute] {
            if string(element, key as String) == label {
                result.append(element)
                break
            }
        }
    }
    for child in children(element) {
        collect(child, label: label, into: &result, depth: depth + 1)
    }
}

guard AXIsProcessTrusted(), CommandLine.arguments.count >= 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-press-button.swift PID LABEL [INDEX_FROM_END]")
    exit(64)
}
let label = CommandLine.arguments[2]
let fromEnd = CommandLine.arguments.count > 3 ? Int(CommandLine.arguments[3]) ?? 0 : 0
let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows])
usleep(200_000)
var buttons: [AXUIElement] = []
collect(app, label: label, into: &buttons)
guard buttons.count > fromEnd else {
    print("button not found: \(label), count=\(buttons.count)")
    exit(1)
}
let target = buttons[buttons.count - 1 - fromEnd]
let result = AXUIElementPerformAction(target, kAXPressAction as CFString)
guard result == .success else {
    print("press failed: \(result.rawValue)")
    exit(1)
}
