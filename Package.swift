// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Blockself",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Blockself", targets: ["Blockself"])
    ],
    targets: [
        .binaryTarget(
            name: "BlockselfFFI",
            url: "https://github.com/BlocksHub/Blockself/releases/download/v0.1.0/BlockselfFFI.xcframework.zip",
            checksum: "bb2745510e68c611e0833f2627d167532014fffa40a2ea750fbc06524d899aba"
        ),
        .target(name: "Blockself", dependencies: ["BlockselfFFI"])
    ]
)