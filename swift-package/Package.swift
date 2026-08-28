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
            url: "PLACEHOLDER_URL",
            checksum: "PLACEHOLDER_CHECKSUM"
        ),
        .target(name: "Blockself", dependencies: ["BlockselfFFI"])
    ]
)