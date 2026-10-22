import AppKit
import ApplicationServices
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
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

func frame(_ element: AXUIElement) -> CGRect {
    var point = CGPoint.zero
    var size = CGSize.zero
    if let position = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(position as! AXValue, .cgPoint, &point)
    }
    if let bounds = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    }
    return CGRect(origin: point, size: size)
}

func find(
    _ element: AXUIElement,
    role expectedRole: String? = nil,
    equals: String? = nil,
    contains: String? = nil,
    identifier: String? = nil,
    depth: Int = 0
) -> AXUIElement? {
    if depth > 60 { return nil }
    let roleMatches = expectedRole.map { role(element) == $0 } ?? true
    let elementLabels = labels(element)
    let labelMatches = equals.map { elementLabels.contains($0) }
        ?? contains.map { needle in elementLabels.contains { $0.contains(needle) } }
        ?? true
    let identifierMatches = identifier.map {
        stringAttribute(element, kAXIdentifierAttribute as String) == $0
    } ?? true
    if roleMatches && labelMatches && identifierMatches { return element }
    for child in children(element) {
        if let match = find(
            child,
            role: expectedRole,
            equals: equals,
            contains: contains,
            identifier: identifier,
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
    identifier: String? = nil,
    timeout: TimeInterval = 12
) -> AXUIElement? {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let match = find(
            app,
            role: expectedRole,
            equals: equals,
            contains: contains,
            identifier: identifier
        ) {
            return match
        }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

func allWithIdentifier(_ element: AXUIElement, identifier: String, depth: Int = 0, into matches: inout [AXUIElement]) {
    if depth > 60 { return }
    if stringAttribute(element, kAXIdentifierAttribute as String) == identifier {
        matches.append(element)
    }
    for child in children(element) {
        allWithIdentifier(child, identifier: identifier, depth: depth + 1, into: &matches)
    }
}

func waitForVisibleIdentifier(_ app: AXUIElement, identifier: String, timeout: TimeInterval = 12) -> AXUIElement? {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        var matches: [AXUIElement] = []
        allWithIdentifier(app, identifier: identifier, into: &matches)
        if let visible = matches.first(where: {
            let rect = frame($0)
            return rect.width > 1 && rect.height > 1
        }) {
            return visible
        }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

func waitForAbsentIdentifier(_ app: AXUIElement, identifier: String, timeout: TimeInterval = 2) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        var matches: [AXUIElement] = []
        allWithIdentifier(app, identifier: identifier, into: &matches)
        if !matches.contains(where: {
            let rect = frame($0)
            return rect.width > 1 && rect.height > 1
        }) {
            return true
        }
        usleep(50_000)
    } while Date() < deadline
    return false
}

func waitForAbsent(_ app: AXUIElement, equals: String, timeout: TimeInterval = 12) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if find(app, equals: equals) == nil { return true }
        usleep(50_000)
    } while Date() < deadline
    return false
}

func press(_ app: AXUIElement, _ label: String) throws {
    guard let element = waitFor(app, role: kAXButtonRole as String, equals: label) else {
        throw NSError(domain: "LocalSendPointerE2E", code: 1, userInfo: [NSLocalizedDescriptionKey: "button not found: \(label)"])
    }
    let error = AXUIElementPerformAction(element, kAXPressAction as CFString)
    guard error == .success else {
        throw NSError(domain: "LocalSendPointerE2E", code: 2, userInfo: [NSLocalizedDescriptionKey: "AXPress failed for \(label): \(error.rawValue)"])
    }
}

func setText(_ app: AXUIElement, label: String, value: String) throws {
    guard let input = waitFor(app, role: kAXTextFieldRole as String, equals: label)
        ?? waitFor(app, role: kAXTextAreaRole as String, equals: label)
    else {
        throw NSError(domain: "LocalSendPointerE2E", code: 3, userInfo: [NSLocalizedDescriptionKey: "text input not found: \(label)"])
    }
    let error = AXUIElementSetAttributeValue(input, kAXValueAttribute as CFString, value as CFTypeRef)
    guard error == .success else {
        throw NSError(domain: "LocalSendPointerE2E", code: 4, userInfo: [NSLocalizedDescriptionKey: "setting text input failed: \(error.rawValue)"])
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
        usleep(100_000)
    } while Date() < deadline
    throw NSError(domain: "LocalSendPointerE2E", code: 5, userInfo: [NSLocalizedDescriptionKey: "selection was not cleared after sending"])
}

func primaryWindow(_ app: AXUIElement) -> AXUIElement? {
    (attribute(app, kAXWindowsAttribute as String) as? [AXUIElement])?.first
}

func windowFrame(_ app: AXUIElement) -> CGRect {
    primaryWindow(app).map(frame) ?? .zero
}

func describeElementAt(_ point: CGPoint) -> String {
    let system = AXUIElementCreateSystemWide()
    var hit: AXUIElement?
    let error = AXUIElementCopyElementAtPosition(system, Float(point.x), Float(point.y), &hit)
    guard error == .success, let hit else { return "hit_error=\(error.rawValue)" }
    let id = stringAttribute(hit, kAXIdentifierAttribute as String) ?? ""
    let value = labels(hit).joined(separator: "|")
    return "hit_role=\(role(hit)) hit_id=\(id) hit_labels=\(value)"
}

func frameDictionary(_ rect: CGRect) -> [String: Double] {
    ["x": rect.origin.x, "y": rect.origin.y, "width": rect.width, "height": rect.height]
}

func peerRequestCount(_ path: String) -> Int? {
    guard let data = FileManager.default.contents(atPath: path),
          let root = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let peer = root["peer"] as? [String: Any],
          let requests = peer["requests"] as? [Any]
    else { return nil }
    return requests.count
}

func waitForPeerRequests(_ path: String, expected: Int, timeout: TimeInterval = 12) throws {
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        if let count = peerRequestCount(path), count >= expected { return }
        usleep(100_000)
    } while Date() < deadline
    throw NSError(
        domain: "LocalSendPointerE2E",
        code: 15,
        userInfo: [NSLocalizedDescriptionKey: "physical click did not reach peer; expected \(expected) requests"]
    )
}

func physicalClick(
    _ app: AXUIElement,
    pid: pid_t,
    element: AXUIElement,
    xFraction: CGFloat,
    yFraction: CGFloat,
    name: String
) throws -> [String: Any] {
    try physicalClickFrame(
        app,
        pid: pid,
        elementFrame: frame(element),
        xFraction: xFraction,
        yFraction: yFraction,
        name: name
    )
}

func physicalClickFrame(
    _ app: AXUIElement,
    pid: pid_t,
    elementFrame: CGRect,
    xFraction: CGFloat,
    yFraction: CGFloat,
    name: String
) throws -> [String: Any] {
    if let runningApp = NSRunningApplication(processIdentifier: pid) {
        runningApp.activate(options: [.activateAllWindows, .activateIgnoringOtherApps])
    }
    if let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement],
       let window = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height })
    {
        AXUIElementPerformAction(window, kAXRaiseAction as CFString)
    }
    usleep(250_000)
    let frontmost = NSWorkspace.shared.frontmostApplication?.processIdentifier ?? 0
    guard frontmost == pid else {
        throw NSError(
            domain: "LocalSendPointerE2E",
            code: 8,
            userInfo: [NSLocalizedDescriptionKey: "app did not become frontmost for \(name): frontmost=\(frontmost), expected=\(pid)"]
        )
    }

    let windowBefore = windowFrame(app)
    let scaleBefore = NSScreen.main?.backingScaleFactor ?? 0
    guard elementFrame.width > 1, elementFrame.height > 1 else {
        throw NSError(domain: "LocalSendPointerE2E", code: 6, userInfo: [NSLocalizedDescriptionKey: "invalid frame for \(name): \(elementFrame)"])
    }
    let point = CGPoint(
        x: elementFrame.origin.x + elementFrame.width * xFraction,
        y: elementFrame.origin.y + elementFrame.height * yFraction
    )
    let eventPoint = point
    fputs(
        "POINTER \(name) frontmost=\(frontmost) expected=\(pid) element=\(frameDictionary(elementFrame)) window=\(frameDictionary(windowBefore)) point=(\(point.x),\(point.y)) \(describeElementAt(point)) event_point=(\(eventPoint.x),\(eventPoint.y)) scale=\(scaleBefore)\n",
        stderr
    )

    CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: eventPoint, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    usleep(80_000)
    let actualPoint = CGEvent(source: nil)?.location ?? .zero
    FileHandle.standardError.write("CURSOR (\(actualPoint.x),\(actualPoint.y))\n".data(using: .utf8)!)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: eventPoint, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    usleep(80_000)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: eventPoint, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    usleep(200_000)

    let windowAfter = windowFrame(app)
    let scaleAfter = NSScreen.main?.backingScaleFactor ?? 0
    guard abs(windowBefore.origin.x - windowAfter.origin.x) <= 1,
          abs(windowBefore.origin.y - windowAfter.origin.y) <= 1,
          abs(windowBefore.width - windowAfter.width) <= 1,
          abs(windowBefore.height - windowAfter.height) <= 1,
          abs(scaleBefore - scaleAfter) <= 0.001
    else {
        throw NSError(domain: "LocalSendPointerE2E", code: 7, userInfo: [NSLocalizedDescriptionKey: "UNSTABLE_WINDOW_GEOMETRY for \(name)"])
    }

    return [
        "name": name,
        "x": point.x,
        "y": point.y,
        "event_x": eventPoint.x,
        "event_y": eventPoint.y,
        "cursor_x": actualPoint.x,
        "cursor_y": actualPoint.y,
        "element_frame": frameDictionary(elementFrame),
        "window_before": frameDictionary(windowBefore),
        "window_after": frameDictionary(windowAfter),
        "scale_before": scaleBefore,
        "scale_after": scaleAfter,
    ]
}

guard AXIsProcessTrusted() else {
    fputs("BLOCKED: enable Accessibility permission for the terminal or Codex app\n", stderr)
    exit(2)
}
guard CommandLine.arguments.count == 10,
      let pid = pid_t(CommandLine.arguments[1])
else {
    fputs("usage: macos-cgevent-e2e.swift PID ALIAS FINGERPRINT TEXT CLIPBOARD STATUS_JSON OUTPUT_JSON SCENARIO EXPECTED_REQUESTS\n", stderr)
    exit(64)
}

let alias = CommandLine.arguments[2]
let fingerprint = CommandLine.arguments[3]
let textMessage = CommandLine.arguments[4]
let clipboardMessage = CommandLine.arguments[5]
let statusPath = CommandLine.arguments[6]
let outputPath = CommandLine.arguments[7]
let scenario = CommandLine.arguments[8]
guard let expectedRequests = Int(CommandLine.arguments[9]) else { exit(64) }
func freshApp() -> AXUIElement {
    let application = AXUIElementCreateApplication(pid)
    AXUIElementSetAttributeValue(application, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
    AXUIElementSetAttributeValue(application, "AXManualAccessibility" as CFString, kCFBooleanTrue)
    return application
}

let app = freshApp()
var clicks: [[String: Any]] = []

func deviceCard() throws -> AXUIElement {
    guard let element = waitForVisibleIdentifier(freshApp(), identifier: "device-card-\(fingerprint)") else {
        throw NSError(domain: "LocalSendPointerE2E", code: 8, userInfo: [NSLocalizedDescriptionKey: "device card AXIdentifier not found for \(alias)"])
    }
    return element
}

func deviceSendButton() throws -> AXUIElement {
    guard let element = waitForVisibleIdentifier(freshApp(), identifier: "device-send-\(fingerprint)") else {
        throw NSError(domain: "LocalSendPointerE2E", code: 9, userInfo: [NSLocalizedDescriptionKey: "device send AXIdentifier not found for \(alias)"])
    }
    return element
}

func physicalClickDeviceCard(
    xFraction: CGFloat,
    yFraction: CGFloat,
    name: String
) throws -> [String: Any] {
    let deadline = Date().addingTimeInterval(12)
    repeat {
        if let element = try? deviceCard() {
            do {
                return try physicalClick(
                    app,
                    pid: pid,
                    element: element,
                    xFraction: xFraction,
                    yFraction: yFraction,
                    name: name
                )
            } catch {
                if !error.localizedDescription.contains("invalid frame") { throw error }
            }
        }
        usleep(100_000)
    } while Date() < deadline
    throw NSError(
        domain: "LocalSendPointerE2E",
        code: 21,
        userInfo: [NSLocalizedDescriptionKey: "stable device card frame not found for \(name)"]
    )
}

do {
    try press(app, "发送")
    guard waitFor(app, equals: "选择") != nil else {
        throw NSError(domain: "LocalSendPointerE2E", code: 10, userInfo: [NSLocalizedDescriptionKey: "send page did not open"])
    }

    switch scenario {
    case "file":
        guard let fileButton = waitFor(app, role: kAXButtonRole as String, equals: "文件") else {
            throw NSError(domain: "LocalSendPointerE2E", code: 20, userInfo: [NSLocalizedDescriptionKey: "file button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: fileButton, xFraction: 0.5, yFraction: 0.5, name: "file-button"))
        guard waitFor(app, role: kAXButtonRole as String, equals: "清除所选项") != nil else {
            throw NSError(domain: "LocalSendPointerE2E", code: 11, userInfo: [NSLocalizedDescriptionKey: "file selection did not appear"])
        }
        clicks.append(try physicalClickDeviceCard(xFraction: 0.35, yFraction: 0.25, name: "file-card-title"))
        try waitForPeerRequests(statusPath, expected: expectedRequests)
        guard let advanced = waitForVisibleIdentifier(freshApp(), identifier: "send-progress-advanced", timeout: 5) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 34, userInfo: [NSLocalizedDescriptionKey: "send progress advanced button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: advanced, xFraction: 0.5, yFraction: 0.5, name: "file-progress-advanced"))
        guard waitFor(freshApp(), role: kAXButtonRole as String, equals: "打开防火墙设置", timeout: 3) != nil else {
            throw NSError(domain: "LocalSendPointerE2E", code: 35, userInfo: [NSLocalizedDescriptionKey: "advanced button did not open diagnostics"])
        }
        guard let closeDiagnostics = waitFor(freshApp(), role: kAXButtonRole as String, equals: "关闭", timeout: 3) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 36, userInfo: [NSLocalizedDescriptionKey: "diagnostics close button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: closeDiagnostics, xFraction: 0.5, yFraction: 0.5, name: "file-diagnostics-close"))
        guard let finish = waitForVisibleIdentifier(freshApp(), identifier: "send-progress-finish", timeout: 5) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 30, userInfo: [NSLocalizedDescriptionKey: "send progress finish button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: finish, xFraction: 0.5, yFraction: 0.5, name: "file-progress-finish"))
        guard waitForAbsentIdentifier(freshApp(), identifier: "send-progress-finish", timeout: 1.2) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 31, userInfo: [NSLocalizedDescriptionKey: "finish did not dismiss send progress page"])
        }
    case "folder":
        try press(app, "文件夹")
        guard waitFor(app, role: kAXButtonRole as String, equals: "清除所选项") != nil else {
            throw NSError(domain: "LocalSendPointerE2E", code: 12, userInfo: [NSLocalizedDescriptionKey: "folder selection did not appear"])
        }
        clicks.append(try physicalClickDeviceCard(xFraction: 0.68, yFraction: 0.25, name: "folder-card-title-tail"))
        try waitForPeerRequests(statusPath, expected: expectedRequests)
        guard let back = waitForVisibleIdentifier(freshApp(), identifier: "send-progress-back", timeout: 5) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 32, userInfo: [NSLocalizedDescriptionKey: "send progress back button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: back, xFraction: 0.5, yFraction: 0.5, name: "folder-progress-back"))
        guard waitForAbsentIdentifier(freshApp(), identifier: "send-progress-back", timeout: 1.2) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 33, userInfo: [NSLocalizedDescriptionKey: "back did not dismiss send progress page"])
        }
    case "text":
        try press(app, "文本")
        try setText(app, label: "发送文本内容", value: textMessage)
        try press(app, "确定")
        guard waitForAbsent(app, equals: "发送文本内容", timeout: 5) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 13, userInfo: [NSLocalizedDescriptionKey: "text dialog did not close"])
        }
        clicks.append(try physicalClickDeviceCard(xFraction: 0.35, yFraction: 0.25, name: "text-card-title"))
    case "clipboard":
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        guard pasteboard.setString(clipboardMessage, forType: .string) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 14, userInfo: [NSLocalizedDescriptionKey: "could not set clipboard"])
        }
        try press(app, "剪贴板")
        clicks.append(try physicalClickDeviceCard(xFraction: 0.35, yFraction: 0.25, name: "clipboard-card-title"))
    default:
        throw NSError(domain: "LocalSendPointerE2E", code: 22, userInfo: [NSLocalizedDescriptionKey: "unknown scenario: \(scenario)"])
    }

    try waitForPeerRequests(statusPath, expected: expectedRequests)
    if scenario == "text" {
        guard let finish = waitForVisibleIdentifier(freshApp(), identifier: "send-progress-finish", timeout: 5) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 37, userInfo: [NSLocalizedDescriptionKey: "text progress finish button not found"])
        }
        clicks.append(try physicalClick(app, pid: pid, element: finish, xFraction: 0.5, yFraction: 0.5, name: "text-progress-finish"))
        guard waitForAbsentIdentifier(freshApp(), identifier: "send-progress-finish", timeout: 1.2) else {
            throw NSError(domain: "LocalSendPointerE2E", code: 38, userInfo: [NSLocalizedDescriptionKey: "text finish did not dismiss send progress page"])
        }
    }
    print("PASS: physical \(scenario) click reached peer")

    let output: [String: Any] = [
        "executed": true,
        "physical_pointer": true,
        "clicks": clicks,
    ]
    let data = try JSONSerialization.data(withJSONObject: output, options: [.prettyPrinted, .sortedKeys])
    try data.write(to: URL(fileURLWithPath: outputPath))
} catch {
    fputs("FAIL: \(error.localizedDescription)\n", stderr)
    exit(1)
}
