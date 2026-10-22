import AppKit
import ApplicationServices
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    let error = AXUIElementCopyAttributeValue(element, name as CFString, &value)
    return error == .success ? value : nil
}

func stringAttribute(_ element: AXUIElement, _ name: String) -> String? {
    attribute(element, name) as? String
}

func children(_ element: AXUIElement) -> [AXUIElement] {
    attribute(element, kAXChildrenAttribute as String) as? [AXUIElement] ?? []
}

func role(_ element: AXUIElement) -> String {
    stringAttribute(element, kAXRoleAttribute as String) ?? ""
}

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute, kAXHelpAttribute]
        .compactMap { stringAttribute(element, $0 as String) }
}

func find(
    _ element: AXUIElement,
    role expectedRole: String? = nil,
    equals: String? = nil,
    contains: String? = nil,
    depth: Int = 0
) -> AXUIElement? {
    if depth > 60 { return nil }
    let elementRole = role(element)
    let elementLabels = labels(element)
    let roleMatches = expectedRole.map { elementRole == $0 } ?? true
    let labelMatches = equals.map { elementLabels.contains($0) }
        ?? contains.map { needle in elementLabels.contains { $0.contains(needle) } }
        ?? true
    if roleMatches && labelMatches { return element }
    for child in children(element) {
        if let match = find(
            child,
            role: expectedRole,
            equals: equals,
            contains: contains,
            depth: depth + 1
        ) {
            return match
        }
    }
    return nil
}

func waitFor(
    _ app: AXUIElement,
    role expectedRole: String? = nil,
    equals: String? = nil,
    contains: String? = nil,
    timeout: TimeInterval = 12
) -> AXUIElement? {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let match = find(app, role: expectedRole, equals: equals, contains: contains) {
            return match
        }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

func waitForAbsent(
    _ app: AXUIElement,
    role expectedRole: String? = nil,
    equals: String? = nil,
    contains: String? = nil,
    timeout: TimeInterval = 12
) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if find(app, role: expectedRole, equals: equals, contains: contains) == nil {
            return true
        }
        usleep(50_000)
    } while Date() < deadline
    return false
}

func press(_ app: AXUIElement, _ label: String) throws {
    guard let element = waitFor(app, role: kAXButtonRole as String, equals: label) else {
        throw NSError(domain: "LocalSendE2E", code: 1, userInfo: [NSLocalizedDescriptionKey: "button not found: \(label)"])
    }
    let error = AXUIElementPerformAction(element, kAXPressAction as CFString)
    guard error == .success else {
        throw NSError(domain: "LocalSendE2E", code: 2, userInfo: [NSLocalizedDescriptionKey: "AXPress failed for \(label): \(error.rawValue)"])
    }
}

func pressTarget(_ app: AXUIElement, alias: String) throws {
    guard let element = waitFor(app, role: kAXButtonRole as String, contains: alias) else {
        throw NSError(domain: "LocalSendE2E", code: 3, userInfo: [NSLocalizedDescriptionKey: "device button not found: \(alias)"])
    }
    let error = AXUIElementPerformAction(element, kAXPressAction as CFString)
    guard error == .success else {
        throw NSError(domain: "LocalSendE2E", code: 4, userInfo: [NSLocalizedDescriptionKey: "AXPress failed for device \(alias): \(error.rawValue)"])
    }
}

func clearSelectionIfNeeded(_ app: AXUIElement, timeout: TimeInterval = 20) throws {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if waitFor(app, role: kAXButtonRole as String, equals: "文件", timeout: 0.1) != nil {
            return
        }
        if let clear = find(app, role: kAXButtonRole as String, equals: "清除所选项") {
            AXUIElementPerformAction(clear, kAXPressAction as CFString)
            if waitFor(app, role: kAXButtonRole as String, equals: "文件", timeout: 2) != nil {
                return
            }
        }
        let selectedVisible = find(app, contains: "已选择") != nil
        if !selectedVisible {
            return
        }
        usleep(100_000)
    } while Date() < deadline
    throw NSError(domain: "LocalSendE2E", code: 5, userInfo: [NSLocalizedDescriptionKey: "selection was not cleared after sending"])
}

func setText(_ app: AXUIElement, label: String, value: String) throws {
    guard let input = waitFor(app, role: kAXTextFieldRole as String, equals: label)
        ?? waitFor(app, role: kAXTextAreaRole as String, equals: label)
    else {
        throw NSError(domain: "LocalSendE2E", code: 6, userInfo: [NSLocalizedDescriptionKey: "text input not found: \(label)"])
    }
    let error = AXUIElementSetAttributeValue(input, kAXValueAttribute as CFString, value as CFTypeRef)
    guard error == .success else {
        throw NSError(domain: "LocalSendE2E", code: 7, userInfo: [NSLocalizedDescriptionKey: "setting text input failed: \(error.rawValue)"])
    }
}

guard AXIsProcessTrusted() else {
    print("BLOCKED: enable Accessibility permission for the terminal or Codex app")
    exit(2)
}
guard CommandLine.arguments.count == 5,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-e2e.swift PID DEVICE_ALIAS TEXT CLIPBOARD_TEXT")
    exit(64)
}

let alias = CommandLine.arguments[2]
let textMessage = CommandLine.arguments[3]
let clipboardMessage = CommandLine.arguments[4]
let app = AXUIElementCreateApplication(pid)

do {
    try press(app, "发送")
    guard waitFor(app, equals: "选择") != nil else {
        throw NSError(domain: "LocalSendE2E", code: 8, userInfo: [NSLocalizedDescriptionKey: "send page did not open"])
    }

    try press(app, "文件")
    guard waitFor(app, role: kAXButtonRole as String, equals: "清除所选项") != nil else {
        throw NSError(domain: "LocalSendE2E", code: 9, userInfo: [NSLocalizedDescriptionKey: "file selection did not appear"])
    }
    try pressTarget(app, alias: alias)
    try clearSelectionIfNeeded(app)
    usleep(1_000_000)
    print("PASS: file button -> device -> transfer")

    try press(app, "文件夹")
    guard waitFor(app, role: kAXButtonRole as String, equals: "清除所选项") != nil else {
        throw NSError(domain: "LocalSendE2E", code: 10, userInfo: [NSLocalizedDescriptionKey: "folder selection did not appear"])
    }
    try pressTarget(app, alias: alias)
    try clearSelectionIfNeeded(app)
    usleep(1_000_000)
    print("PASS: folder button -> device -> transfer")

    try press(app, "文本")
    try setText(app, label: "发送文本内容", value: textMessage)
    try press(app, "确定")
    guard waitForAbsent(app, equals: "发送文本内容", timeout: 5) else {
        throw NSError(domain: "LocalSendE2E", code: 11, userInfo: [NSLocalizedDescriptionKey: "text dialog did not close"])
    }
    try pressTarget(app, alias: alias)
    usleep(1_000_000)
    print("PASS: text dialog -> device -> transfer")

    let pasteboard = NSPasteboard.general
    pasteboard.clearContents()
    guard pasteboard.setString(clipboardMessage, forType: .string) else {
        throw NSError(domain: "LocalSendE2E", code: 12, userInfo: [NSLocalizedDescriptionKey: "could not set clipboard"])
    }
    try press(app, "剪贴板")
    try pressTarget(app, alias: alias)
    usleep(1_000_000)
    print("PASS: clipboard button -> device -> transfer")
} catch {
    print("FAIL: \(error.localizedDescription)")
    exit(1)
}
