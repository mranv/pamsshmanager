# PAM SSH User Manager

A Rust-based tool for automated SSH user management, integrated with Pluggable Authentication Module (PAM) for session-based user creation and deletion. This tool creates SSH users dynamically and deletes them automatically at the end of their session, enhancing security and simplifying user management.

## Features

- **User Creation and Deletion:** Easily create and delete SSH users via a command-line interface.
- **Automated User Deletion:** Automatically delete users after their SSH session ends, using PAM hooks.
- **Integration with PAM:** Leverages PAM for secure and efficient session management.
- **Robust Error Handling:** Includes comprehensive error messages and logging for troubleshooting.

## Requirements

- **Rust**: Ensure you have Rust installed. You can install it using [rustup](https://rustup.rs/).
- **PAM**: Proper configuration and permissions to integrate with the PAM system on your server.
- **sudo Permissions**: The tool requires `sudo` to create and delete system users.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/pam-ssh-user-manager.git
   cd pam-ssh-user-manager
   ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

3. **Deploy the PAM Module:**

   - Copy the compiled shared library (`pam_ssh_manager.so`) to your PAM modules directory, typically `/lib/security/` or `/lib64/security/`.

4. **Configure PAM:**
   - Edit your PAM configuration file for SSH (`/etc/pam.d/sshd`) to include the new module:
     ```bash
     session required pam_ssh_manager.so
     ```

## Usage

### Command Line Interface

- **Create a User:**

  ```bash
  sudo ./pam-ssh-user-manager create <username>
  ```

- **Delete a User:**
  ```bash
  sudo ./pam-ssh-user-manager delete <username>
  ```

### PAM Integration

- The tool automatically deletes the user after the SSH session ends using PAM session hooks. Ensure proper testing and deployment, as improper configuration can impact system access.

## Use Case

This tool is ideal for environments requiring temporary user access to systems, such as:

- **Secure Testing Environments:** Where users need temporary access to servers.
- **Training Sessions:** Where participants are given temporary user accounts.
- **Shared Workspaces:** Where temporary access to shared systems is necessary.

By automating user management and leveraging PAM, this tool reduces administrative overhead and enhances security by ensuring user accounts do not persist beyond their intended use.

## Contributing

Contributions are welcome! Please fork this repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

Use with caution and ensure all deployments are tested in a secure, controlled environment. Misconfiguration can lead to system access issues.

```

### Notes:
- **Security:** Be cautious with the PAM integration and ensure you have tested the tool thoroughly in a safe environment. Misconfiguration can lock you out of your system.
- **Permissions:** Running user management tasks requires appropriate permissions. Ensure that the tool runs with the necessary privileges.
- **Documentation:** Keep the README up to date with any changes to the tool or its usage.
```
