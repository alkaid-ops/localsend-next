import AppKit
import ApplicationServices
import CoreGraphics
import Foundation

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    let error = AXUIElementCopyAttributeValue(element, name as CFString, &value)
    return error == .success ? value : nil
}

func text(_ element: AXUIElement, _ name: String) -> String? {
    attribute(element, name) as? String
}

func children(_ element: AXUIElement) -> [AXUIElement] {
    attribute(element, kAXChildrenAttribute as String) as? [AXUIElement] ?? []
}

func find(_ element: AXUIElement, label: String, depth: Int = 0) -> AXUIElement? {
    if depth > 30 { return nil }
    for key in [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute] {
        if text(element, key as String) == label { return element }
    }
    for child in children(element) {
        if let match = find(child, label: label, depth: depth + 1) { return match }
    }
    return nil
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

func click(_ element: AXUIElement) {
    let bounds = frame(element)
    let point = CGPoint(x: bounds.midX, y: bounds.midY)
    CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: point, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: point, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: point, mouseButton: .left)?
        .post(tap: .cghidEventTap)
}

guard AXIsProcessTrusted() else {
    print("BLOCKED: enable Accessibility permission for the terminal or Codex app")
    exit(2)
}
guard CommandLine.arguments.count == 2, let pid = pid_t(CommandLine.arguments[1]) else {
    print("usage: macos-ax-smoke.swift PID")
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate(options: [.activateAllWindows])
AXUIElementSetAttributeValue(app, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
AXUIElementSetAttributeValue(app, "AXManualAccessibility" as CFString, kCFBooleanTrue)
func waitFor(_ label: String) -> AXUIElement? {
    for _ in 0..<100 {
        if let element = find(app, label: label) { return element }
        usleep(50_000)
    }
    return nil
}

func navigate(_ label: String, expected: String) -> Bool {
    guard let element = waitFor(label) else { return false }
    AXUIElementPerformAction(element, kAXPressAction as CFString)
    if waitForUntil(expected, deadline: Date().addingTimeInterval(1.0)) != nil {
        return true
    }
    click(element)
    return waitFor(expected) != nil
}

func waitForUntil(_ label: String, deadline: Date) -> AXUIElement? {
    repeat {
        if let element = find(app, label: label) { return element }
        usleep(50_000)
    } while Date() < deadline
    return nil
}

guard navigate("发送", expected: "文件") else {
    print("FAIL: send page did not become visible")
    exit(1)
}
guard navigate("设置", expected: "主题") else {
    print("FAIL: settings page did not become visible")
    exit(1)
}
print("PASS: native accessibility navigated send and settings pages")
