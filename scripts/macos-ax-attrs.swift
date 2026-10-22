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

func smallest(_ element: AXUIElement, containing point: CGPoint, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    var match: AXUIElement?
    let bounds = frame(element)
    if bounds.width > 0, bounds.height > 0, bounds.contains(point) {
        match = element
    }
    for child in children(element) {
        if let candidate = smallest(child, containing: point, depth: depth + 1) {
            let candidateArea = frame(candidate).width * frame(candidate).height
            let currentArea = match.map { frame($0).width * frame($0).height } ?? .greatestFiniteMagnitude
            if candidateArea <= currentArea { match = candidate }
        }
    }
    return match
}

func describe(_ value: CFTypeRef) -> String {
    if let string = value as? String { return string }
    if let number = value as? NSNumber { return number.stringValue }
    return String(describing: value)
}

guard AXIsProcessTrusted(), CommandLine.arguments.count == 4,
      let pid = pid_t(CommandLine.arguments[1]),
      let x = Double(CommandLine.arguments[2]),
      let y = Double(CommandLine.arguments[3])
else {
    print("usage: macos-ax-attrs.swift PID X Y")
    exit(64)
}

let app = AXUIElementCreateApplication(pid)
guard let element = smallest(app, containing: CGPoint(x: x, y: y)) else {
    print("no AX element at \(x),\(y)")
    exit(1)
}

var names: CFArray?
if AXUIElementCopyAttributeNames(element, &names) == .success {
    for name in (names as? [String] ?? []).sorted() {
        if let value = attribute(element, name) {
            print("\(name)=\(describe(value))")
        }
    }
}
var actions: CFArray?
if AXUIElementCopyActionNames(element, &actions) == .success {
    print("actions=\((actions as? [String] ?? []).joined(separator: ","))")
}
