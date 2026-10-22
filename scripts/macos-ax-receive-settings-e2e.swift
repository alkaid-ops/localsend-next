import ApplicationServices
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}

func text(_ element: AXUIElement, _ name: String) -> String? {
    attribute(element, name) as? String
}

func children(_ element: AXUIElement) -> [AXUIElement] {
    attribute(element, kAXChildrenAttribute as String) as? [AXUIElement] ?? []
}

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute, kAXHelpAttribute]
        .compactMap { text(element, $0 as String) }
}

func find(
    _ element: AXUIElement,
    equals: String? = nil,
    contains: String? = nil,
    depth: Int = 0
) -> AXUIElement? {
    if depth > 60 { return nil }
    let values = labels(element)
    let matches = equals.map(values.contains) ?? contains.map { needle in values.contains { $0.contains(needle) } } ?? true
    if matches { return element }
    for child in children(element) {
        if let match = find(child, equals: equals, contains: contains, depth: depth + 1) {
            return match
        }
    }
    return nil
}

func waitFor(
    _ app: AXUIElement,
    equals: String? = nil,
    contains: String? = nil,
    timeout: TimeInterval = 15
) -> AXUIElement? {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let element = find(app, equals: equals, contains: contains) { return element }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

func waitForAbsent(
    _ app: AXUIElement,
    equals: String,
    timeout: TimeInterval = 15
) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if find(app, equals: equals) == nil { return true }
        usleep(50_000)
    } while Date() < deadline
    return false
}

func press(_ app: AXUIElement, _ label: String, timeout: TimeInterval = 15) throws {
    guard let element = waitFor(app, equals: label, timeout: timeout) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 1, userInfo: [NSLocalizedDescriptionKey: "control not found: \(label)"])
    }
    let result = AXUIElementPerformAction(element, kAXPressAction as CFString)
    guard result == .success else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 2, userInfo: [NSLocalizedDescriptionKey: "AXPress failed for \(label): \(result.rawValue)"])
    }
}

func setText(_ app: AXUIElement, label: String, value: String) throws {
    guard let input = waitFor(app, equals: label) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 3, userInfo: [NSLocalizedDescriptionKey: "text input not found: \(label)"])
    }
    let result = AXUIElementSetAttributeValue(input, kAXValueAttribute as CFString, value as CFTypeRef)
    guard result == .success else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 4, userInfo: [NSLocalizedDescriptionKey: "set value failed: \(result.rawValue)"])
    }
}

func writeAction(_ directory: URL, name: String, object: [String: Any]) throws {
    let data = try JSONSerialization.data(withJSONObject: object, options: [.prettyPrinted, .sortedKeys])
    try data.write(to: directory.appendingPathComponent("\(name).json"), options: .atomic)
}

func scrollDown(_ app: AXUIElement, pixels: Int32 = -500) {
    guard let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement],
          let window = windows.first
    else { return }
    var point = CGPoint.zero
    var size = CGSize.zero
    if let position = attribute(window, kAXPositionAttribute as String) {
        AXValueGetValue(position as! AXValue, .cgPoint, &point)
    }
    if let bounds = attribute(window, kAXSizeAttribute as String) {
        AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    }
    let center = CGPoint(x: point.x + size.width / 2, y: point.y + size.height / 2)
    CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: center, mouseButton: .left)?.post(tap: .cghidEventTap)
    CGEvent(scrollWheelEvent2Source: nil, units: .pixel, wheelCount: 1, wheel1: pixels, wheel2: 0, wheel3: 0)?.post(tap: .cghidEventTap)
    usleep(300_000)
}

func settingValue(_ statusURL: URL, key: String) -> Any? {
    guard let data = try? Data(contentsOf: statusURL),
          let root = try? JSONSerialization.jsonObject(with: data) as? [String: Any]
    else { return nil }
    let settings = (root["settings"] as? [String: Any])
        ?? ((root["snapshot"] as? [String: Any])?["settings"] as? [String: Any])
    guard let settings else { return nil }
    return settings[key]
}

func waitForSetting(
    _ statusURL: URL,
    key: String,
    value: Any,
    timeout: TimeInterval = 10
) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let actual = settingValue(statusURL, key: key) {
            switch value {
            case let expected as String where ((actual as? NSString) as String?) == expected:
                return true
            case let expected as Bool where (actual as? NSNumber)?.boolValue == expected:
                return true
            default:
                break
            }
        }
        usleep(50_000)
    } while Date() < deadline
    return false
}

func incomingCount(_ statusURL: URL) -> Int {
    guard let data = try? Data(contentsOf: statusURL),
          let root = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let incoming = root["incoming_results"] as? [Any]
    else { return 0 }
    return incoming.count
}

func waitForIncomingCount(
    _ statusURL: URL,
    count: Int,
    timeout: TimeInterval = 25
) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if incomingCount(statusURL) >= count { return true }
        usleep(50_000)
    } while Date() < deadline
    return false
}

guard AXIsProcessTrusted() else {
    print("BLOCKED: enable Accessibility permission for the terminal or Codex app")
    exit(2)
}
guard CommandLine.arguments.count == 6,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-receive-settings-e2e.swift PID ACTIONS_DIR STATUS_PATH PIN TEXT")
    exit(64)
}

let actionsDir = URL(fileURLWithPath: CommandLine.arguments[2], isDirectory: true)
let statusURL = URL(fileURLWithPath: CommandLine.arguments[3])
let pin = CommandLine.arguments[4]
let message = CommandLine.arguments[5]
let app = AXUIElementCreateApplication(pid)
var sequence = 0

func actionName(_ name: String) -> String {
    sequence += 1
    return String(format: "%03d-%@", sequence, name)
}

do {
    try press(app, "接收")
    guard waitFor(app, equals: "通过链接接收") != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 5, userInfo: [NSLocalizedDescriptionKey: "receive page did not open"])
    }

    try writeAction(actionsDir, name: actionName("partial"), object: [
        "kind": "send_files",
        "paths": [actionsDir.deletingLastPathComponent().appendingPathComponent("partial-a.txt").path,
                  actionsDir.deletingLastPathComponent().appendingPathComponent("partial-b.txt").path]
    ])
    guard waitFor(app, equals: "接受所选") != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 6, userInfo: [NSLocalizedDescriptionKey: "partial request did not appear"])
    }
    // Requests start fully selected. Uncheck beta so only alpha is accepted.
    try press(app, "partial-b.txt")
    try press(app, "接受所选")
    guard waitForAbsent(app, equals: "接受所选") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 7, userInfo: [NSLocalizedDescriptionKey: "partial request did not close"])
    }
    print("PASS: partial receive through checkbox and accept-selected")

    try writeAction(actionsDir, name: actionName("reject"), object: [
        "kind": "send_files",
        "paths": [actionsDir.deletingLastPathComponent().appendingPathComponent("reject-me.txt").path]
    ])
    guard waitFor(app, equals: "全部接受") != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 8, userInfo: [NSLocalizedDescriptionKey: "reject request did not appear"])
    }
    try press(app, "拒绝")
    guard waitForAbsent(app, equals: "全部接受") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 9, userInfo: [NSLocalizedDescriptionKey: "rejected request did not close"])
    }
    print("PASS: reject incoming request")

    try writeAction(actionsDir, name: actionName("text"), object: [
        "kind": "send_text",
        "text": message
    ])
    guard waitFor(app, equals: "文本消息") != nil, waitFor(app, contains: message) != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 10, userInfo: [NSLocalizedDescriptionKey: "incoming text message did not appear"])
    }
    try press(app, "关闭")
    print("PASS: incoming text message")

    try press(app, "设置")
    // Flutter may still be laying out the settings list for a few frames after
    // the navigation transition. Scroll until the receive section becomes
    // semantically visible instead of discarding the first wheel event.
    var receiveSettingsVisible = false
    for _ in 0..<4 {
        usleep(350_000)
        scrollDown(app)
        receiveSettingsVisible = waitFor(app, equals: "自动保存", timeout: 0.8) != nil
            && waitFor(app, equals: "自动完成传输任务", timeout: 0.8) != nil
            && waitFor(app, equals: "启用 PIN 密码", timeout: 0.8) != nil
        if receiveSettingsVisible { break }
    }
    guard waitFor(app, equals: "自动保存") != nil,
          waitFor(app, equals: "自动完成传输任务") != nil,
          waitFor(app, equals: "启用 PIN 密码") != nil,
          receiveSettingsVisible
    else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 11, userInfo: [NSLocalizedDescriptionKey: "receive settings are missing"])
    }
    try press(app, "自动保存")
    guard waitForSetting(statusURL, key: "auto_save", value: "on") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 11, userInfo: [NSLocalizedDescriptionKey: "quick-save setting did not persist"])
    }
    guard waitFor(app, contains: "自动接受所有文件传输请求") != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 20, userInfo: [NSLocalizedDescriptionKey: "auto-save help dialog did not appear"])
    }
    try press(app, "关闭")
    try press(app, "自动完成传输任务")
    guard waitForSetting(statusURL, key: "auto_finish", value: true) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 11, userInfo: [NSLocalizedDescriptionKey: "auto-finish setting did not persist"])
    }
    print("PASS: settings quick-save and auto-finish changed through UI")

    try press(app, "接收")
    try writeAction(actionsDir, name: actionName("quick-save"), object: [
        "kind": "send_files",
        "paths": [actionsDir.deletingLastPathComponent().appendingPathComponent("auto-pin.txt").path]
    ])
    guard waitForIncomingCount(statusURL, count: 4) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 14, userInfo: [NSLocalizedDescriptionKey: "quick-save action did not finish"])
    }
    guard waitFor(app, equals: "全部接受", timeout: 0.8) == nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 14, userInfo: [NSLocalizedDescriptionKey: "quick-save still prompted for an incoming file"])
    }
    guard waitFor(app, equals: "打开文件") != nil,
          waitFor(app, contains: "auto-pin.txt") != nil,
          waitFor(app, equals: "在文件管理器中显示") != nil
    else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 17, userInfo: [NSLocalizedDescriptionKey: "receive completion dialog did not appear"])
    }
    guard waitForAbsent(app, equals: "打开文件", timeout: 8) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 18, userInfo: [NSLocalizedDescriptionKey: "auto-finish did not close the completion dialog"])
    }
    print("PASS: quick-save accepted without a request prompt")

    try press(app, "设置")
    try press(app, "自动保存")
    guard waitForSetting(statusURL, key: "auto_save", value: "off") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 14, userInfo: [NSLocalizedDescriptionKey: "quick-save off did not persist"])
    }
    try press(app, "启用 PIN 密码")
    try setText(app, label: "设置值", value: pin)
    try press(app, "保存")
    guard waitForAbsent(app, equals: "设置值") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 12, userInfo: [NSLocalizedDescriptionKey: "PIN settings dialog did not close"])
    }
    guard waitForSetting(statusURL, key: "pin", value: pin) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 12, userInfo: [NSLocalizedDescriptionKey: "PIN setting did not persist"])
    }
    if (settingValue(statusURL, key: "auto_save") as? String) != "off" {
        try press(app, "自动保存")
        _ = waitForSetting(statusURL, key: "auto_save", value: "off")
    }
    if (settingValue(statusURL, key: "auto_finish") as? Bool) != true {
        try press(app, "自动完成传输任务")
        _ = waitForSetting(statusURL, key: "auto_finish", value: true)
    }
    usleep(500_000)

    try press(app, "接收")
    try writeAction(actionsDir, name: actionName("wrong-pin"), object: [
        "kind": "send_files",
        "paths": [actionsDir.deletingLastPathComponent().appendingPathComponent("wrong-pin.txt").path]
    ])
    guard waitForIncomingCount(statusURL, count: 5) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 13, userInfo: [NSLocalizedDescriptionKey: "wrong-PIN action did not finish"])
    }
    guard find(app, equals: "全部接受") == nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 13, userInfo: [NSLocalizedDescriptionKey: "request without PIN reached the receiver UI"])
    }

    try writeAction(actionsDir, name: actionName("manual-pin"), object: [
        "kind": "send_files",
        "paths": [actionsDir.deletingLastPathComponent().appendingPathComponent("manual-pin.txt").path],
        "pin": pin
    ])
    guard waitFor(app, equals: "全部接受") != nil else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 15, userInfo: [NSLocalizedDescriptionKey: "manual PIN request did not appear"])
    }
    try press(app, "全部接受")
    guard waitForAbsent(app, equals: "全部接受") else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 16, userInfo: [NSLocalizedDescriptionKey: "manual PIN request did not close"])
    }
    guard waitForIncomingCount(statusURL, count: 6) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 16, userInfo: [NSLocalizedDescriptionKey: "manual PIN action did not finish"])
    }
    guard waitForAbsent(app, equals: "打开文件", timeout: 8) else {
        throw NSError(domain: "LocalSendReceiveE2E", code: 19, userInfo: [NSLocalizedDescriptionKey: "manual PIN completion dialog did not close"])
    }
    print("PASS: PIN rejected without value, accepted with PIN and auto-finished")
} catch {
    print("FAIL: \(error.localizedDescription)")
    exit(1)
}
