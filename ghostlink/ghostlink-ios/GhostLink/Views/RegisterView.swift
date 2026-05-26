import SwiftUI

struct RegisterView: View {
    @Binding var isLoggedIn: Bool
    @Environment(\.dismiss) var dismiss

    @State private var username = ""
    @State private var password = ""
    @State private var confirmPassword = ""
    @State private var acceptedWarning = false
    @State private var isLoading = false
    @State private var errorMessage: String?

    var body: some View {
        ZStack {
            Color(red: 10/255, green: 14/255, blue: 23/255).ignoresSafeArea()

            ScrollView {
                VStack(spacing: 16) {
                    Text("Create Account")
                        .font(.title2)
                        .fontWeight(.bold)
                        .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))
                        .padding(.top, 40)

                    VStack(spacing: 12) {
                        TextField("Username", text: $username)
                            .textFieldStyle(.plain)
                            .padding()
                            .background(Color(white: 0.15))
                            .cornerRadius(10)
                            .foregroundColor(.white)
                            .autocapitalization(.none)
                            .disableAutocorrection(true)

                        SecureField("Password (min 8 chars)", text: $password)
                            .textFieldStyle(.plain)
                            .padding()
                            .background(Color(white: 0.15))
                            .cornerRadius(10)
                            .foregroundColor(.white)

                        SecureField("Confirm Password", text: $confirmPassword)
                            .textFieldStyle(.plain)
                            .padding()
                            .background(Color(white: 0.15))
                            .cornerRadius(10)
                            .foregroundColor(.white)
                    }
                    .padding(.horizontal, 32)

                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Image(systemName: "exclamationmark.triangle.fill")
                                .foregroundColor(.orange)
                            Text("No Account Recovery")
                                .fontWeight(.bold)
                                .foregroundColor(.orange)
                        }
                        Text("GhostLink does not store email, phone, or any recovery information. If you lose your credentials, your account is permanently lost.")
                            .font(.caption)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .background(Color.orange.opacity(0.1))
                    .cornerRadius(12)
                    .padding(.horizontal, 32)

                    Toggle(isOn: $acceptedWarning) {
                        Text("I understand and accept that account recovery is impossible")
                            .font(.caption)
                            .foregroundColor(.gray)
                    }
                    .padding(.horizontal, 32)

                    if let error = errorMessage {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                            .padding(.horizontal, 32)
                    }

                    Button(action: register) {
                        if isLoading {
                            ProgressView()
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else {
                            Text("Create Account")
                                .fontWeight(.semibold)
                                .frame(maxWidth: .infinity)
                                .padding()
                        }
                    }
                    .background(canRegister ? Color(red: 100/255, green: 181/255, blue: 246/255) : Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(12)
                    .padding(.horizontal, 32)
                    .disabled(!canRegister || isLoading)

                    Spacer()
                }
            }
        }
    }

    private var canRegister: Bool {
        username.count >= 3 && password.count >= 8 && password == confirmPassword && acceptedWarning
    }

    private func register() {
        isLoading = true
        errorMessage = nil

        Task {
            do {
                let keyBundle = SignalManager.shared.generateKeys()

                let request = RegisterRequest(
                    username: username.lowercased().trimmingCharacters(in: .whitespaces),
                    password: password,
                    identity_key: keyBundle.identityKey,
                    signed_pre_key: keyBundle.signedPreKey,
                    one_time_pre_keys: keyBundle.oneTimeKeys
                )

                let response: AuthResponse = try await APIClient.shared.request(
                    "/auth/register", method: "POST", body: request
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
