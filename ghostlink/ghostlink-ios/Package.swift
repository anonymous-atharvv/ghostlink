// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "GhostLinkDependencies",
    platforms: [
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "GhostLinkDependencies",
            targets: ["GhostLinkDependencies"]
        ),
    ],
    dependencies: [
        // Secure SQLite storage via SQLCipher
        .package(url: "https://github.com/groue/GRDB.swift.git", from: "6.24.0"),
        
        // Official Signal Client bindings for Swift
        .package(url: "https://github.com/signalapp/libsignal-client.git", from: "0.36.0"),
        
        // Image loading
        .package(url: "https://github.com/SDWebImage/SDWebImageSwiftUI.git", from: "2.2.0"),
        
        // Security helpers
        .package(url: "https://github.com/securing/IOSSecuritySuite.git", from: "1.9.0")
    ],
    targets: [
        .target(
            name: "GhostLinkDependencies",
            dependencies: [
                .product(name: "GRDB", package: "GRDB.swift"),
                .product(name: "LibSignalClient", package: "libsignal-client"),
                .product(name: "SDWebImageSwiftUI", package: "SDWebImageSwiftUI"),
                .product(name: "IOSSecuritySuite", package: "IOSSecuritySuite")
            ]
        ),
    ]
)
