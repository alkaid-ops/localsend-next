import CoreGraphics
import Foundation

guard CommandLine.arguments.count == 2, let pid = Int(CommandLine.arguments[1]) else {
    print("usage: macos-window-id.swift PID")
    exit(64)
}

let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let windows = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []
let candidates = windows.compactMap { window -> (Int, Double)? in
    guard (window[kCGWindowOwnerPID as String] as? Int) == pid,
          let number = window[kCGWindowNumber as String] as? Int,
          let bounds = window[kCGWindowBounds as String] as? [String: Any],
          let width = bounds["Width"] as? Double,
          let height = bounds["Height"] as? Double,
          width >= 300, height >= 300
    else { return nil }
    return (number, width * height)
}
guard let (number, _) = candidates.max(by: { $0.1 < $1.1 }) else {
    fputs("no window found for pid \(pid)\n", stderr)
    exit(1)
}
print(number)
