import AppKit
import Foundation

guard CommandLine.arguments.count == 2 else {
    fputs("usage: macos-copy-file.swift PATH\n", stderr)
    exit(64)
}
let url = URL(fileURLWithPath: CommandLine.arguments[1]).standardizedFileURL
guard FileManager.default.fileExists(atPath: url.path) else {
    fputs("file does not exist: \(url.path)\n", stderr)
    exit(1)
}
let pasteboard = NSPasteboard.general
pasteboard.clearContents()
guard pasteboard.writeObjects([url as NSURL]) else {
    fputs("failed to write file URL to pasteboard\n", stderr)
    exit(1)
}
print(url.path)
