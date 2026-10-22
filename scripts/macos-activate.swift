import AppKit
import Foundation

guard CommandLine.arguments.count == 2, let pid = pid_t(CommandLine.arguments[1]) else {
    fputs("usage: macos-activate.swift PID\n", stderr)
    exit(64)
}
guard let app = NSRunningApplication(processIdentifier: pid) else {
    fputs("process not found: \(pid)\n", stderr)
    exit(1)
}
if !app.activate(options: [.activateAllWindows, .activateIgnoringOtherApps]) {
    fputs("failed to activate pid \(pid)\n", stderr)
    exit(1)
}
usleep(200_000)
