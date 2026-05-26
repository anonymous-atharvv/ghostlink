import SwiftUI

struct LoginView: View {
    @Binding var isLoggedIn: Bool
    @Environment(\.dismiss) var dismiss

    @State private var username = ""
    @State private var password = ""
    @State private var isLoading = false
    @State private var errorMessage: String?

    var body: some View {
        ZStack {
            Color(red: 10/255, green: 14/255, blue: 23/255).ignoresSafeArea()

            VStack(spacing: 16) {
                Spacer()

                Text("Welcome Back")
                    .font(.title2)
                    .fontWeight(.bold)
                    .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))

                VStack(spacing: 12) {
                    TextField("Username", text: $username)
                        .textFieldStyle(.plain)
                        .padding()
                        .background(Color(white: 0.15))
                        .cornerRadius(10)
                        .foregroundColor(.white)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)

                    SecureField("Password", text: $password)
                        .textFieldStyle(.plain)
                        .padding()
                        .background(Color(white: 0.15))
                        .cornerRadius(10)
                        .foregroundColor(.white)
                }
                .padding(.horizontal, 32)

                if let error = errorMessage {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                }

                Button(action: login) {
                    if isLoading {
                        ProgressView()
                            .frame(maxWidth: .infinity)
                            .padding()
                    } else {
                        Text("Login")
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                            .padding()
                    }
                }
                .background(username.isEmpty || password.isEmpty || isLoading
                    ? Color.gray
                    : Color(red: 100/255, green: 181/255, blue: 246/255))
                .foregroundColor(.white)
                .cornerRadius(12)
                .padding(.horizontal, 32)
                .disabled(username.isEmpty || password.isEmpty || isLoading)

                Spacer()
            }
        }
    }

    private func login() {
        isLoading = true
        errorMessage = nil

        Task {
            do {
                let request = LoginRequest(
                    username: username.lowercased().trimmingCharacters(in: .whitespaces),
                    password: password
                )

                let response: AuthResponse = try await APIClient.shared.request(
                    "/auth/login", method: "POST", body: request
                )

                _ = KeychainManager.shared.save(response.token, forKey: "com.ghostlink.auth.jwt")
                _ = KeychainManager.shared.save(response.username, forKey: "com.ghostlink.auth.username")

                await MainActor.run {
                    isLoggedIn = true
                }
            } catch {
                await MainActor.run {
                    errorMessage = error.localizedDescription
                    isLoading = false
                }
            }
        }
    }
}
