import SwiftUI

struct MessageRow: Identifiable {
    let id = UUID()
    let text: String
    let isMine: Bool
    let timestamp: Date
}

struct ConversationView: View {
    let contactUsername: String
    @State private var messages: [MessageRow] = []
    @State private var messageText = ""

    var body: some View {
        VStack(spacing: 0) {
            if messages.isEmpty {
                Spacer()
                Text("Send a message to start the conversation")
                    .foregroundColor(.gray.opacity(0.6))
                Spacer()
            } else {
                ScrollView {
                    LazyVStack(spacing: 8) {
                        ForEach(messages) { msg in
                            HStack {
                                if msg.isMine { Spacer() }
                                Text(msg.text)
                                    .padding(.horizontal, 16)
                                    .padding(.vertical, 10)
                                    .background(msg.isMine
                                        ? Color(red: 100/255, green: 181/255, blue: 246/255).opacity(0.85)
                                        : Color(white: 0.15))
                                    .foregroundColor(.white)
                                    .cornerRadius(16, corners: msg.isMine
                                        ? [.topLeading, .topTrailing, .bottomLeading]
                                        : [.topLeading, .topTrailing, .bottomTrailing])
                                    .frame(maxWidth: 280, alignment: msg.isMine ? .trailing : .leading)
                                if !msg.isMine { Spacer() }
                            }
                            .padding(.horizontal)
                        }
                    }
                    .padding(.vertical, 8)
                }
            }

            HStack(spacing: 8) {
                TextField("Message...", text: $messageText)
                    .textFieldStyle(.plain)
                    .padding(12)
                    .background(Color(white: 0.12))
                    .cornerRadius(24)
                    .foregroundColor(.white)

                Button(action: sendMessage) {
                    Image(systemName: "arrow.up.circle.fill")
                        .font(.system(size: 32))
                        .foregroundColor(messageText.isEmpty
                            ? .gray
                            : Color(red: 100/255, green: 181/255, blue: 246/255))
                }
                .disabled(messageText.isEmpty)
            }
            .padding()
            .background(Color(white: 0.08))
        }
        .background(Color(red: 10/255, green: 14/255, blue: 23/255))
        .navigationTitle(contactUsername)
        .navigationBarTitleDisplayMode(.inline)
    }

    private func sendMessage() {
        guard !messageText.isEmpty else { return }
        let text = messageText
        messageText = ""

        let msg = MessageRow(text: text, isMine: true, timestamp: Date())
        messages.append(msg)

        let reply = MessageRow(
            text: "🔒 End-to-end encrypted",
            isMine: false,
            timestamp: Date()
        )
        messages.append(reply)
    }
}

extension View {
    func cornerRadius(_ radius: CGFloat, corners: UIRectCorner) -> some View {
        clipShape(RoundedCorner(radius: radius, corners: corners))
    }
}

struct RoundedCorner: Shape {
    var radius: CGFloat = .infinity
    var corners: UIRectCorner = .allCorners

    func path(in rect: CGRect) -> Path {
        let path = UIBezierPath(
            roundedRect: rect,
            byRoundingCorners: corners,
            cornerRadii: CGSize(width: radius, height: radius)
        )
        return Path(path.cgPath)
    }
}
