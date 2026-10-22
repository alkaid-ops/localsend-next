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

func frame(_ element: AXUIElement) -> CGRect {
    var origin = CGPoint.zero
    var size = CGSize.zero
    if let value = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgPoint, &origin)
    }
    if let value = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(value as! AXValue, .cgSize, &size)
    }
    return CGRect(origin: origin, size: size)
}

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
}

guard AXIsProcessTrusted(), CommandLine.arguments.count >= 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-scan.swift PID LABEL [press]")
    exit(64)
}

let query = CommandLine.arguments[2].lowercased()
let action = CommandLine.arguments.count > 3 ? CommandLine.arguments[3] : "find"
let application = AXUIElementCreateApplication(pid)
NSRunningApplication(processIdentifier: pid)?.activate()
usleep(200_000)
let windows = attribute(application, kAXWindowsAttribute as String) as? [AXUIElement] ?? []
guard let targetWindow = windows.max(by: { frame($0).width * frame($0).height < frame($1).width * frame($1).height }) else {
    print("no window")
    exit(1)
}
let bounds = frame(targetWindow)
guard bounds.width >= 100, bounds.height >= 100 else {
    print("invalid window bounds")
    exit(1)
}

var found: AXUIElement?
var foundPoint = CGPoint.zero
var foundLabels: [String] = []
let system = AXUIElementCreateSystemWide()
var y = bounds.minY + 2
while y < bounds.maxY {
    var x = bounds.minX + 2
    while x < bounds.maxX {
        var hit: AXUIElement?
        if AXUIElementCopyElementAtPosition(system, Float(x), Float(y), &hit) == .success,
           let hit
        {
            let values = labels(hit)
            if values.contains(where: { $0.lowercased().contains(query) }) {
                found = hit
                foundPoint = CGPoint(x: x, y: y)
                foundLabels = values
                break
            }
        }
        x += 10
    }
    if found != nil { break }
    y += 10
}

guard let found else {
    print("not found")
    exit(1)
}
print(String(format: "found %.0f,%.0f %@", foundPoint.x, foundPoint.y, foundLabels.joined(separator: " | ")))
if action == "press" {
    let result = AXUIElementPerformAction(found, kAXPressAction as CFString)
    guard result == .success else {
        print("press failed: \(result.rawValue)")
        exit(1)
    }
} else if action == "click" || action == "double-click" {
    let clicks = action == "double-click" ? 2 : 1
    for click in 1...clicks {
        let down = CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: foundPoint, mouseButton: .left)
        let up = CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: foundPoint, mouseButton: .left)
        down?.setIntegerValueField(.mouseEventClickState, value: Int64(click))
        up?.setIntegerValueField(.mouseEventClickState, value: Int64(click))
        down?.post(tap: .cghidEventTap)
        up?.post(tap: .cghidEventTap)
        usleep(80_000)
    }
}
