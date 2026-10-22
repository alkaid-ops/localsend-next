import ApplicationServices
import CoreGraphics
import Foundation

func typeText(_ text: String) {
    let characters = Array(text.utf16)
    var offset = 0
    while offset < characters.count {
        let end = min(offset + 20, characters.count)
        let chunk = Array(characters[offset..<end])
        for down in [true, false] {
            guard let event = CGEvent(keyboardEventSource: nil, virtualKey: 0, keyDown: down) else {
                continue
            }
            event.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
            event.post(tap: .cghidEventTap)
        }
        offset = end
    }
}

func pressKey(_ keyCode: CGKeyCode, flags: CGEventFlags) {
    let source = CGEventSource(stateID: .combinedSessionState)
    for down in [true, false] {
        guard let event = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: down) else {
            continue
        }
        event.flags = flags
        event.post(tap: .cghidEventTap)
    }
}

guard AXIsProcessTrusted(), CommandLine.arguments.count >= 3 else {
    print("usage: macos-keyinput.swift type TEXT | key KEYCODE [cmd,shift,option,control]")
    exit(64)
}

switch CommandLine.arguments[1] {
case "type":
    typeText(CommandLine.arguments[2])
case "key":
    guard let code = UInt16(CommandLine.arguments[2]) else { exit(64) }
    var flags: CGEventFlags = []
    if CommandLine.arguments.count > 3 {
        for modifier in CommandLine.arguments[3].split(separator: ",") {
            switch modifier {
            case "cmd": flags.insert(.maskCommand)
            case "shift": flags.insert(.maskShift)
            case "option": flags.insert(.maskAlternate)
            case "control": flags.insert(.maskControl)
            default: break
            }
        }
    }
    pressKey(CGKeyCode(code), flags: flags)
default:
    exit(64)
}
