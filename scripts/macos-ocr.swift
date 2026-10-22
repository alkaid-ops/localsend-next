import AppKit
import Foundation
import Vision

guard CommandLine.arguments.count >= 2 else {
    fputs("usage: macos-ocr.swift IMAGE [FILTER]\n", stderr)
    exit(64)
}

let imageURL = URL(fileURLWithPath: CommandLine.arguments[1])
guard let image = NSImage(contentsOf: imageURL),
      let data = image.tiffRepresentation,
      let bitmap = NSBitmapImageRep(data: data)
else {
    fputs("cannot read image: \(imageURL.path)\n", stderr)
    exit(1)
}
let width = CGFloat(bitmap.pixelsWide)
let height = CGFloat(bitmap.pixelsHigh)
let scale = NSScreen.main?.backingScaleFactor ?? 1
let filter = CommandLine.arguments.count >= 3 ? CommandLine.arguments[2].lowercased() : nil
let request = VNRecognizeTextRequest()
request.recognitionLevel = .accurate
request.usesLanguageCorrection = false
request.recognitionLanguages = ["zh-Hans", "en-US"]
let handler = VNImageRequestHandler(cgImage: bitmap.cgImage!, options: [:])
try handler.perform([request])

for observation in request.results ?? [] {
    guard let text = observation.topCandidates(1).first?.string else { continue }
    if let filter, !text.lowercased().contains(filter) { continue }
    let box = observation.boundingBox
    let x = box.minX * width
    let y = (1 - box.maxY) * height
    let w = box.width * width
    let h = box.height * height
    print(
        String(
            format: "%.0f %.0f %.0f %.0f | logical %.1f %.1f %.1f %.1f | %@",
            x, y, w, h,
            x / scale, y / scale, w / scale, h / scale,
            text
        )
    )
}
