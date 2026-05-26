import SwiftUI

struct MainTabView: View {
    @Binding var isLoggedIn: Bool

    var body: some View {
        TabView {
            ChatListView()
                .tabItem {
                    Label("Chats", systemImage: "message.fill")
                }

            SettingsView(isLoggedIn: $isLoggedIn)
                .tabItem {
                    Label("Settings", systemImage: "gearshape.fill")
                }
        }
        .tint(Color(red: 100/255, green: 181/255, blue: 246/255))
    }
}
