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

func findRole(_ element: AXUIElement, role: String, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    if string(element, kAXRoleAttribute as String) == role { return element }
    for child in children(element) {
        if let match = findRole(child, role: role, depth: depth + 1) { return match }
    }
    return nil
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    fputs("usage: macos-ax-focus.swift PID ROLE\n", stderr)
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
guard let target = findRole(app, role: CommandLine.arguments[2]) else {
    fputs("role not found: \(CommandLine.arguments[2])\n", stderr)
    exit(1)
}
let result = AXUIElementSetAttributeValue(target, kAXFocusedAttribute as CFString, kCFBooleanTrue)
guard result == .success else {
    fputs("focus failed: \(result.rawValue)\n", stderr)
    exit(1)
}
usleep(200_000)
