import SwiftUI

@main
struct GhostLinkApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    @Environment(\.scenePhase) var scenePhase
    @State private var isLoggedIn = false

    var body: some Scene {
        WindowGroup {
            if isLoggedIn {
                MainTabView(isLoggedIn: $isLoggedIn)
            } else {
                WelcomeView(isLoggedIn: $isLoggedIn)
            }
        }
        .onChange(of: scenePhase) { phase in
            if phase == .active {
                isLoggedIn = KeychainManager.shared.get(forKey: "com.ghostlink.auth.jwt") != nil
            }
        }
    }
}

struct VisualEffectView: UIViewRepresentable {
    var effect: UIVisualEffect?
    func makeUIView(context: Context) -> UIVisualEffectView { UIVisualEffectView(effect: effect) }
    func updateUIView(_ uiView: UIVisualEffectView, context: Context) { uiView.effect = effect }
}
