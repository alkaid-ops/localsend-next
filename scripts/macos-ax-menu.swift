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

func find(_ element: AXUIElement, label: String, depth: Int = 0) -> AXUIElement? {
    if depth > 20 { return nil }
    for key in [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute] {
        if string(element, key as String) == label { return element }
    }
    for child in children(element) {
        if let match = find(child, label: label, depth: depth + 1) { return match }
    }
    return nil
}

func printLabels(_ element: AXUIElement, depth: Int = 0) {
    if depth > 5 { return }
    let labels = [kAXTitleAttribute, kAXDescriptionAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
    if !labels.isEmpty {
        print("\(String(repeating: "  ", count: depth))\(labels.joined(separator: " | "))")
    }
    for child in children(element) {
        printLabels(child, depth: depth + 1)
    }
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 4,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-menu.swift PID MENU ITEM")
    exit(64)
}

let menu = CommandLine.arguments[2]
let item = CommandLine.arguments[3]
let app = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate()
usleep(250_000)
guard let menuBar = attribute(app, kAXMenuBarAttribute as String) else {
    print("menu bar unavailable")
    exit(1)
}
let menuBarElement = unsafeBitCast(menuBar, to: AXUIElement.self)
guard let menuElement = find(menuBarElement, label: menu) else {
    print("menu not found: \(menu)")
    printLabels(menuBarElement)
    exit(1)
}
_ = AXUIElementPerformAction(menuElement, kAXPressAction as CFString)
usleep(250_000)
guard let itemElement = find(menuBarElement, label: item) else {
    print("menu item not found: \(item)")
    exit(1)
}
let result = AXUIElementPerformAction(itemElement, kAXPressAction as CFString)
guard result == .success else {
    print("menu item press failed: \(result.rawValue)")
    exit(1)
}
