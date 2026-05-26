import SwiftUI

struct SettingsView: View {
    @Binding var isLoggedIn: Bool
    @State private var showLogoutConfirm = false
    @State private var username: String = ""

    var body: some View {
        NavigationStack {
            ZStack {
                Color(red: 10/255, green: 14/255, blue: 23/255).ignoresSafeArea()

                VStack(spacing: 24) {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Account")
                            .font(.headline)
                            .foregroundColor(.white)
                        Text("Username: \(username)")
                            .font(.body)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color(white: 0.12))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Security")
                            .font(.headline)
                            .foregroundColor(.white)
                        Text("App Lock: Face ID / Passcode")
                            .font(.body)
                            .foregroundColor(.gray)
                        Text("Screenshot Protection: Active")
                            .font(.body)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color(white: 0.12))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Privacy")
                            .font(.headline)
                            .foregroundColor(.white)
                        Text("Messages are end-to-end encrypted.")
                            .font(.body)
                            .foregroundColor(.gray)
                        Text("No message content stored on servers.")
                            .font(.body)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color(white: 0.12))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    Spacer()

                    Button(action: { showLogoutConfirm = true }) {
                        Text("Logout")
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.red.opacity(0.8))
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .padding(.horizontal, 32)
                    .padding(.bottom, 32)
                }
                .padding(.top)
            }
            .navigationTitle("Settings")
            .navigationBarTitleDisplayMode(.inline)
            .onAppear {
                username = KeychainManager.shared.get(forKey: "com.ghostlink.auth.username") ?? ""
            }
            .alert("Logout", isPresented: $showLogoutConfirm) {
                Button("Logout", role: .destructive) {
                    _ = KeychainManager.shared.delete(forKey: "com.ghostlink.auth.jwt")
                    _ = KeychainManager.shared.delete(forKey: "com.ghostlink.auth.username")
                    DatabaseManager.shared.resetDatabase()
                    isLoggedIn = false
                }
                Button("Cancel", role: .cancel) {}
            } message: {
                Text("This will clear all local data. You will need your credentials to log back in.")
            }
        }
    }
}

extension DatabaseManager {
    func resetDatabase() {
        dbQueue = nil
        setupDatabase()
    }
}
