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

func labels(_ element: AXUIElement) -> [String] {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute, kAXHelpAttribute]
        .compactMap { string(element, $0 as String) }
        .filter { !$0.isEmpty }
}

func frame(_ element: AXUIElement) -> String {
    var origin = CGPoint.zero
    var size = CGSize.zero
    if let position = attribute(element, kAXPositionAttribute as String) {
        AXValueGetValue(position as! AXValue, .cgPoint, &origin)
    }
    if let bounds = attribute(element, kAXSizeAttribute as String) {
        AXValueGetValue(bounds as! AXValue, .cgSize, &size)
    }
    return String(format: "%.0f,%.0f %.0fx%.0f", origin.x, origin.y, size.width, size.height)
}

func collect(
    _ element: AXUIElement,
    text: String,
    role wantedRole: String?,
    exact: Bool,
    into result: inout [AXUIElement],
    depth: Int = 0
) {
    guard depth <= 60 else { return }
    let role = string(element, kAXRoleAttribute as String) ?? ""
    let roleMatches = wantedRole.map { $0 == role } ?? true
    let labelMatches = labels(element).contains { value in
        exact ? value == text : value.localizedCaseInsensitiveContains(text)
    }
    if roleMatches && labelMatches {
        result.append(element)
    }
    for child in children(element) {
        collect(
            child,
            text: text,
            role: wantedRole,
            exact: exact,
            into: &result,
            depth: depth + 1
        )
    }
}

func actions(_ element: AXUIElement) -> [String] {
    var names: CFArray?
    guard AXUIElementCopyActionNames(element, &names) == .success else { return [] }
    return names as? [String] ?? []
}

func roots(_ app: AXUIElement) -> [AXUIElement] {
    if let windows = attribute(app, kAXWindowsAttribute as String) as? [AXUIElement],
       !windows.isEmpty
    {
        return windows
    }
    return [app]
}

func firstEmptyButton(_ element: AXUIElement, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    if string(element, kAXRoleAttribute as String) == kAXButtonRole as String,
       labels(element).isEmpty
    {
        return element
    }
    for child in children(element) {
        if let button = firstEmptyButton(child, depth: depth + 1) { return button }
    }
    return nil
}

func firstEditable(_ element: AXUIElement, depth: Int = 0) -> AXUIElement? {
    guard depth <= 40 else { return nil }
    let role = string(element, kAXRoleAttribute as String)
    if role == kAXTextFieldRole as String || role == kAXTextAreaRole as String {
        return element
    }
    for child in children(element) {
        if let field = firstEditable(child, depth: depth + 1) { return field }
    }
    return nil
}

func dump(_ element: AXUIElement, filter: String?, depth: Int = 0) {
    guard depth <= 20 else { return }
    let role = string(element, kAXRoleAttribute as String) ?? ""
    let values = labels(element).joined(separator: " | ")
    let line = "[\(depth)] \(role) \(frame(element)) \(values)"
    if filter.map({ line.localizedCaseInsensitiveContains($0) }) ?? true {
        print(line)
    }
    for child in children(element) {
        dump(child, filter: filter, depth: depth + 1)
    }
}

guard AXIsProcessTrusted(), CommandLine.arguments.count >= 3,
      let pid = pid_t(CommandLine.arguments[1])
else {
    print("usage: macos-ax-background.swift PID dump|press|actions|set-text ...")
    exit(64)
}

let command = CommandLine.arguments[2]
let app = AXUIElementCreateApplication(pid)
_ = AXUIElementSetAttributeValue(app, "AXEnhancedUserInterface" as CFString, kCFBooleanTrue)
_ = AXUIElementSetAttributeValue(app, "AXManualAccessibility" as CFString, kCFBooleanTrue)

switch command {
case "dump":
    let filter = CommandLine.arguments.count >= 4 ? CommandLine.arguments[3] : nil
    for root in roots(app) {
        dump(root, filter: filter)
    }
case "press", "actions", "press-empty", "press-menu", "actions-menu":
    guard CommandLine.arguments.count >= 4 else { exit(64) }
    let label = CommandLine.arguments[3]
    let role = CommandLine.arguments.count >= 5 ? CommandLine.arguments[4] : nil
    let exact = false
    var matches: [AXUIElement] = []
    let searchRoots = if command.hasSuffix("-menu") {
        [app]
    } else {
        roots(app)
    }
    for root in searchRoots {
        collect(root, text: label, role: role, exact: exact, into: &matches)
    }
    guard !matches.isEmpty else {
        print("not found: \(label), role=\(role ?? "any")")
        exit(1)
    }
    for (index, element) in matches.enumerated() {
        let role = string(element, kAXRoleAttribute as String) ?? ""
        let names = actions(element).joined(separator: ",")
        print("[\(index)] \(role) \(frame(element)) \(labels(element).joined(separator: " | ")) actions=\(names)")
    }
    if command == "press-menu" {
        guard let target = matches.first(where: { actions($0).contains(kAXPressAction as String) }) else {
            print("menu item has no AXPress action: \(label)")
            exit(1)
        }
        let result = AXUIElementPerformAction(target, kAXPressAction as CFString)
        print("press-menu=\(result.rawValue)")
        exit(result == .success ? 0 : 1)
    }
    if command == "press-empty" {
        let ancestor = matches[0]
        guard let button = firstEmptyButton(ancestor) else {
            print("empty button not found below: \(label)")
            exit(1)
        }
        let result = AXUIElementPerformAction(button, kAXPressAction as CFString)
        print("press-empty=\(result.rawValue) \(frame(button))")
        exit(result == .success ? 0 : 1)
    }
    if command == "press" {
        let fromEnd = CommandLine.arguments.count >= 6 ? Int(CommandLine.arguments[5]) ?? 0 : 0
        guard fromEnd < matches.count else { exit(1) }
        let target = matches[matches.count - 1 - fromEnd]
        let result = AXUIElementPerformAction(target, kAXPressAction as CFString)
        print("press=\(result.rawValue)")
        exit(result == .success ? 0 : 1)
    }
case "set-text", "type-text", "key-digits":
    guard CommandLine.arguments.count >= 4 else { exit(64) }
    let value = CommandLine.arguments[3]
    let label = CommandLine.arguments.count >= 5 ? CommandLine.arguments[4] : ""
    var editable: AXUIElement?
    if label.isEmpty {
        editable = roots(app).lazy.compactMap { firstEditable($0) }.first
    } else {
        var matches: [AXUIElement] = []
        for root in roots(app) {
            collect(root, text: label, role: nil, exact: false, into: &matches)
        }
        editable = matches.first { element in
            let role = string(element, kAXRoleAttribute as String)
            return role == kAXTextFieldRole as String || role == kAXTextAreaRole as String
        }
    }
    guard let field = editable else {
        print("editable field not found: \(label)")
        exit(1)
    }
    if command == "type-text" || command == "key-digits" {
        let focus = AXUIElementSetAttributeValue(
            field,
            kAXFocusedAttribute as CFString,
            kCFBooleanTrue
        )
        let source = CGEventSource(stateID: .hidSystemState)
        if command == "key-digits" {
            let keycodes: [Character: CGKeyCode] = [
                "0": 29, "1": 18, "2": 19, "3": 20, "4": 21,
                "5": 23, "6": 22, "7": 26, "8": 28, "9": 25,
            ]
            for character in value {
                guard let keycode = keycodes[character] else { continue }
                CGEvent(keyboardEventSource: source, virtualKey: keycode, keyDown: true)?.postToPid(pid)
                CGEvent(keyboardEventSource: source, virtualKey: keycode, keyDown: false)?.postToPid(pid)
                usleep(60_000)
            }
        } else {
            var units = Array(value.utf16)
            let down = CGEvent(
                keyboardEventSource: source,
                virtualKey: 0,
                keyDown: true
            )
            let up = CGEvent(
                keyboardEventSource: source,
                virtualKey: 0,
                keyDown: false
            )
            down?.keyboardSetUnicodeString(stringLength: units.count, unicodeString: &units)
            up?.keyboardSetUnicodeString(stringLength: units.count, unicodeString: &units)
            if let down { down.postToPid(pid) }
            usleep(50_000)
            if let up { up.postToPid(pid) }
        }
        usleep(200_000)
        print("focus=\(focus.rawValue) type=posted")
        exit(0)
    }
    let focused = AXUIElementSetAttributeValue(
        field,
        kAXFocusedAttribute as CFString,
        kCFBooleanTrue
    )
    var range = CFRange(location: 0, length: 0)
    if let length = attribute(field, kAXNumberOfCharactersAttribute as String) as? Int {
        range.length = length
    }
    let selectedRange = AXValueCreate(.cfRange, &range).map {
        AXUIElementSetAttributeValue(field, kAXSelectedTextRangeAttribute as CFString, $0)
    } ?? .failure
    let selectedText = AXUIElementSetAttributeValue(
        field,
        kAXSelectedTextAttribute as CFString,
        value as CFTypeRef
    )
    let result = AXUIElementSetAttributeValue(
        field,
        kAXValueAttribute as CFString,
        value as CFTypeRef
    )
    let confirm = AXUIElementPerformAction(field, kAXConfirmAction as CFString)
    print(
        "focus=\(focused.rawValue) range=\(selectedRange.rawValue) " +
        "selected=\(selectedText.rawValue) value=\(result.rawValue) confirm=\(confirm.rawValue)"
    )
    exit(result == .success ? 0 : 1)
default:
    print("unknown command: \(command)")
    exit(64)
}
