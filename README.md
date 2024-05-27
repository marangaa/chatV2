
**Chat Application 📱💬**
==========================

A simple chat application built in Rust using Tokio, Tungstenite, and Futures for asynchronous WebSocket communication. 💻

**Features 🎉**
-------------

* **WebSocket-based chat server and client** 📈
* **Real-time messaging between multiple clients** 🕰️
* **Server broadcasts messages to all connected clients** 📢
* **Clients can send and receive messages in a chat room** 💬

**Planned Features (Not Yet Implemented) 🚧**
-----------------------------------------

* **User Authentication** 🔒: Ensure that only authorized users can access the chat server. [ ] 
* **Persistent Storage** 💾: Integrate a database to store chat messages and user information persistently. [ ] 
* **Private Messaging** 📝: Add support for private messaging between users. [ ] 
* **Multiple Rooms** 🏠: Allow users to create or join different chat rooms. [ ] 
* **User Status** 👥: Show online/offline status for users and display when users are typing. [ ] 
* **Message Formatting** 🎨: Support message formatting such as bold, italic, and code blocks. [ ] 
* **File Sharing** 📁: Enable users to share files within the chat. [ ] 
* **Emoji Support** 😊: Implement emoji support for messages. [ ] 
* **Message History** 📚: Provide users with access to message history for the chat room. [ ] 
* **Notifications** 📣: Implement desktop or mobile notifications for new messages. [ ] 
* **Customizable Themes** 🎨: Allow users to customize the appearance of the chat interface with different themes. [ ] 
* **Moderation Tools** 🛠️: Implement moderation tools for admins to manage users and messages. [ ] 
* **Integration with External Services** 📈: Integrate with other platforms or services such as Slack, Discord, or Twitter. [ ] 
* **Localization** 🌎: Add support for multiple languages to make the application accessible to a wider audience. [ ] 
* **Automated Testing** 🧪: Write automated tests to ensure the reliability and stability of the application. [ ] 

**Getting Started 🚀**
-------------------

### Prerequisites 📝

* **Rust (stable version)** 💻
* **Cargo (Rust's package manager)** 📦

### Installation 📂

1. **Clone the repository**:
```bash
git clone https://github.com/yourusername/chatV2.git
```
2. **Navigate to the project directory**:
```bash
cd chatV2
```

### Usage 📊
-------------

To run the chat application, follow these steps:

1. **Start the server**:
```bash
cargo run --bin chatV2 server
```
2. **Start a client**:
```bash
cargo run --bin chatV2 client
```
3. **Enter your username when prompted**.
4. **Start chatting with other users** 💬

**Contributing 🤝**
--------------

Contributions are welcome Please feel free to fork the repository and submit pull requests to contribute new features, enhancements, or bug fixes.

**License 📄**
---------

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Acknowledgments 🙏**
----------------

* Thank you to the developers of Tokio, Tungstenite, and Futures for providing the tools necessary to build this chat application.
* Special thanks to the Rust community for their support and contributions. 💕
