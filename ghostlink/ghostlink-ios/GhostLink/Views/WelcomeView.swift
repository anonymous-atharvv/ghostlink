import SwiftUI

struct WelcomeView: View {
    @Binding var isLoggedIn: Bool
    @State private var showRegister = false
    @State private var showLogin = false

    var body: some View {
        ZStack {
            Color(red: 10/255, green: 14/255, blue: 23/255).ignoresSafeArea()

            VStack(spacing: 24) {
                Spacer()

                Text("🕵️")
                    .font(.system(size: 64))

                Text("GhostLink")
                    .font(.largeTitle)
                    .fontWeight(.bold)
                    .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))

                Text("Zero logs. Zero trace. Zero identity.")
                    .font(.body)
                    .foregroundColor(.gray)

                Spacer().frame(height: 32)

                Button(action: { showRegister = true }) {
                    Text("Create Account")
                        .fontWeight(.semibold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color(red: 100/255, green: 181/255, blue: 246/255))
                        .foregroundColor(.white)
                        .cornerRadius(12)
                }
                .padding(.horizontal, 32)

                Button(action: { showLogin = true }) {
                    Text("Login")
                        .fontWeight(.semibold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(Color(red: 100/255, green: 181/255, blue: 246/255), lineWidth: 1)
                        )
                        .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))
                }
                .padding(.horizontal, 32)

                Spacer()

                Text("No phone number or email required.\nNo account recovery available.")
                    .font(.caption)
                    .foregroundColor(.gray.opacity(0.6))
                    .multilineTextAlignment(.center)
                    .padding(.bottom, 32)
            }
        }
        .fullScreenCover(isPresented: $showRegister) {
            RegisterView(isLoggedIn: $isLoggedIn)
        }
        .fullScreenCover(isPresented: $showLogin) {
            LoginView(isLoggedIn: $isLoggedIn)
        }
    }
}
