import ApplicationServices
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

guard AXIsProcessTrusted() else {
    print("BLOCKED: enable Accessibility permission for the terminal or Codex app")
    exit(2)
}
guard CommandLine.arguments.count == 2, let pid = pid_t(CommandLine.arguments[1]) else {
    print("usage: macos-ax-smoke.swift PID")
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
func waitFor(_ label: String) -> AXUIElement? {
    for _ in 0..<100 {
        if let element = find(app, label: label) { return element }
        usleep(50_000)
    }
    return nil
}

guard let send = waitFor("发送") else {
    print("FAIL: send navigation element not found")
    exit(1)
}
AXUIElementPerformAction(send, kAXPressAction as CFString)
guard waitFor("选择") != nil else {
    print("FAIL: send page did not become visible")
    exit(1)
}
guard let settings = find(app, label: "设置") else {
    print("FAIL: settings navigation not found")
    exit(1)
}
AXUIElementPerformAction(settings, kAXPressAction as CFString)
guard waitFor("主题") != nil else {
    print("FAIL: settings page did not become visible")
    exit(1)
}
print("PASS: native accessibility navigated send and settings pages")
