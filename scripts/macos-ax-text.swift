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

func findTextInput(_ element: AXUIElement, depth: Int = 0) -> AXUIElement? {
    if depth > 25 { return nil }
    let role = string(element, kAXRoleAttribute as String)
    if role == kAXTextFieldRole as String || role == kAXTextAreaRole as String { return element }
    for child in children(element) {
        if let match = findTextInput(child, depth: depth + 1) { return match }
    }
    return nil
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-text.swift PID VALUE")
    exit(64)
}

let value = CommandLine.arguments[2]
let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows])
usleep(250_000)
guard let field = findTextInput(app) else {
    print("text input not found")
    exit(1)
}
var actionNames: CFArray?
if AXUIElementCopyActionNames(field, &actionNames) == .success {
    print("actions=\((actionNames as? [String] ?? []).joined(separator: ","))")
}
_ = AXUIElementSetAttributeValue(field, kAXFocusedAttribute as CFString, kCFBooleanTrue)
if value.hasPrefix("keys:") {
    let text = String(value.dropFirst(5))
        .replacingOccurrences(of: "\\", with: "\\\\")
        .replacingOccurrences(of: "\"", with: "\\\"")
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/usr/bin/osascript")
    process.arguments = ["-e", "tell application \"System Events\" to keystroke \"\(text)\""]
    try process.run()
    process.waitUntilExit()
    print("keystroke=\(process.terminationStatus)")
    exit(process.terminationStatus == 0 ? 0 : 1)
}
var range = CFRange(location: 0, length: 0)
if let length = attribute(field, kAXNumberOfCharactersAttribute as String) as? Int {
    range.length = length
}
if let rangeValue = AXValueCreate(.cfRange, &range) {
    _ = AXUIElementSetAttributeValue(field, kAXSelectedTextRangeAttribute as CFString, rangeValue)
}
let selected = AXUIElementSetAttributeValue(field, kAXSelectedTextAttribute as CFString, value as CFTypeRef)
let full = AXUIElementSetAttributeValue(field, kAXValueAttribute as CFString, value as CFTypeRef)
let confirm = AXUIElementPerformAction(field, kAXConfirmAction as CFString)
print("selected=\(selected.rawValue) value=\(full.rawValue) confirm=\(confirm.rawValue)")
