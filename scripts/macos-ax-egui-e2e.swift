import AppKit
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

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
}

func find(_ element: AXUIElement, label: String, role: String? = nil, contains: Bool = false, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    let roleMatches = role.map { string(element, kAXRoleAttribute as String) == $0 } ?? true
    if roleMatches {
        for value in labels(element) {
            if contains ? value.localizedCaseInsensitiveContains(label) : value == label {
                return element
            }
        }
    }
    for child in children(element) {
        if let match = find(child, label: label, role: role, contains: contains, depth: depth + 1) {
            return match
        }
    }
    return nil
}

func findTextInput(_ element: AXUIElement, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    let role = string(element, kAXRoleAttribute as String)
    if role == kAXTextFieldRole as String || role == kAXTextAreaRole as String {
        return element
    }
    for child in children(element) {
        if let match = findTextInput(child, depth: depth + 1) { return match }
    }
    return nil
}

func click(_ element: AXUIElement) {
    let bounds = frame(element)
    let point = CGPoint(x: bounds.midX, y: bounds.midY)
    CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: point, mouseButton: .left)?.post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: point, mouseButton: .left)?.post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: point, mouseButton: .left)?.post(tap: .cghidEventTap)
}

func typeText(_ value: String, pid: pid_t) {
    NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows])
    usleep(250_000)
    for character in value {
        let escaped = String(character)
            .replacingOccurrences(of: "\\", with: "\\\\")
            .replacingOccurrences(of: "\"", with: "\\\"")
        let process = Process()
        process.executableURL = URL(fileURLWithPath: "/usr/bin/osascript")
        process.arguments = ["-e", "tell application \"System Events\" to keystroke \"\(escaped)\""]
        try? process.run()
        process.waitUntilExit()
        usleep(45_000)
    }
    usleep(700_000)
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 4,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-egui-e2e.swift PID TARGET PIN")
    exit(64)
}

let target = CommandLine.arguments[2]
let pin = CommandLine.arguments[3]
let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows])
AXUIElementSetAttributeValue(app, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
AXUIElementSetAttributeValue(app, "AXManualAccessibility" as CFString, kCFBooleanTrue)

func waitFor(_ label: String, role: String? = nil, contains: Bool = false, timeout: TimeInterval = 10) -> AXUIElement? {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let match = find(app, label: label, role: role, contains: contains) { return match }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

func press(_ label: String, role: String? = nil, contains: Bool = false) -> Bool {
    guard let element = waitFor(label, role: role, contains: contains) else { return false }
    return AXUIElementPerformAction(element, kAXPressAction as CFString) == .success
}

func pressAndWait(
    _ label: String,
    role: String? = nil,
    expected: String,
    expectedRole: String? = nil,
    expectedContains: Bool = false,
    timeout: TimeInterval = 10
) -> Bool {
    guard let element = waitFor(label, role: role) else { return false }
    AXUIElementPerformAction(element, kAXPressAction as CFString)
    if waitFor(expected, role: expectedRole, contains: expectedContains, timeout: 1) != nil {
        return true
    }
    click(element)
    return waitFor(
        expected,
        role: expectedRole,
        contains: expectedContains,
        timeout: timeout
    ) != nil
}

func pressUntilAbsent(
    _ label: String,
    role: String? = nil,
    absent: String,
    timeout: TimeInterval = 10
) -> Bool {
    guard let element = waitFor(label, role: role) else { return false }
    AXUIElementPerformAction(element, kAXPressAction as CFString)
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if find(app, label: absent) == nil { return true }
        usleep(50_000)
    } while Date() < deadline
    click(element)
    let secondDeadline = Date().addingTimeInterval(timeout)
    repeat {
        if find(app, label: absent) == nil { return true }
        usleep(50_000)
    } while Date() < secondDeadline
    return false
}

func navigate(_ label: String, expected: String) -> Bool {
    guard let element = waitFor(label) else { return false }
    AXUIElementPerformAction(element, kAXPressAction as CFString)
    if waitFor(expected, timeout: 0.8) != nil { return true }
    click(element)
    return waitFor(expected) != nil
}

guard navigate("发送", expected: "文件") else {
    print("FAIL: send page did not open")
    exit(1)
}
guard pressAndWait(
    "手动 IP",
    role: kAXButtonRole as String,
    expected: "设备地址或主机名"
) else {
    print("FAIL: manual-address dialog did not open")
    exit(1)
}
guard let input = findTextInput(app) else {
    print("FAIL: manual-address input missing")
    exit(1)
}
AXUIElementSetAttributeValue(input, kAXFocusedAttribute as CFString, kCFBooleanTrue)
click(input)
AXUIElementSetAttributeValue(input, kAXFocusedAttribute as CFString, kCFBooleanTrue)
typeText(target, pid: pid)
usleep(300_000)
guard pressAndWait(
    "添加",
    role: kAXButtonRole as String,
    expected: "发送",
    expectedRole: kAXButtonRole as String
) else {
    print("FAIL: manual-address add button missing")
    exit(1)
}
guard pressAndWait(
    "文件",
    role: kAXButtonRole as String,
    expected: "已选择",
    expectedContains: true
) else {
    print("FAIL: fixture file was not selected")
    exit(1)
}
guard pressAndWait(
    "发送",
    role: kAXButtonRole as String,
    expected: "需要 PIN"
) else {
    print("FAIL: PIN dialog did not appear")
    exit(1)
}
guard let pinInput = findTextInput(app) else {
    print("FAIL: PIN input missing")
    exit(1)
}
AXUIElementSetAttributeValue(pinInput, kAXFocusedAttribute as CFString, kCFBooleanTrue)
click(pinInput)
AXUIElementSetAttributeValue(pinInput, kAXFocusedAttribute as CFString, kCFBooleanTrue)
typeText(pin, pid: pid)
usleep(300_000)
guard pressUntilAbsent("重试", role: kAXButtonRole as String, absent: "需要 PIN") else {
    print("FAIL: PIN retry button missing")
    exit(1)
}
print("PASS: egui selected file, added manual target, retried PIN")
