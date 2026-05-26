import SwiftUI

struct ChatListView: View {
    @State private var contacts: [String] = []
    @State private var showNewChat = false
    @State private var newChatUsername = ""

    var body: some View {
        NavigationStack {
            ZStack {
                Color(red: 10/255, green: 14/255, blue: 23/255).ignoresSafeArea()

                if contacts.isEmpty {
                    VStack(spacing: 16) {
                        Text("💬").font(.system(size: 48))
                        Text("No conversations yet")
                            .foregroundColor(.gray)
                        Text("Tap + to add a contact")
                            .font(.caption)
                            .foregroundColor(.gray.opacity(0.6))
                    }
                } else {
                    List(contacts, id: \.self) { contact in
                        NavigationLink(destination: ConversationView(contactUsername: contact)) {
                            HStack {
                                ZStack {
                                    RoundedRectangle(cornerRadius: 8)
                                        .fill(Color(red: 100/255, green: 181/255, blue: 246/255).opacity(0.2))
                                        .frame(width: 48, height: 48)
                                    Text(String(contact.prefix(2).uppercased()))
                                        .fontWeight(.bold)
                                        .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))
                                }
                                VStack(alignment: .leading) {
                                    Text(contact)
                                        .fontWeight(.semibold)
                                        .foregroundColor(.white)
                                    Text("Connected")
                                        .font(.caption)
                                        .foregroundColor(.gray)
                                }
                                .padding(.leading, 8)
                            }
                        }
                        .listRowBackground(Color(white: 0.1))
                    }
                    .listStyle(.plain)
                }
            }
            .navigationTitle("GhostLink")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showNewChat = true }) {
                        Image(systemName: "plus")
                            .foregroundColor(Color(red: 100/255, green: 181/255, blue: 246/255))
                    }
                }
            }
            .alert("New Chat", isPresented: $showNewChat) {
                TextField("Username", text: $newChatUsername)
                    .autocapitalization(.none)
                Button("Add") {
                    if !newChatUsername.isEmpty {
                        contacts.append(newChatUsername.lowercased())
                        newChatUsername = ""
                    }
                }
                Button("Cancel", role: .cancel) { newChatUsername = "" }
            } message: {
                Text("Enter the exact username")
            }
        }
    }
}
