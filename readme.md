# Portfolio Program 📁

The **Portfolio Program** is a Solana-based decentralized application (dApp) for creating and managing user portfolios. Users can store personal details, add links, upload an image, request and approve vouches, send messages, and receive tips.

---

## Features

- **Portfolio Management**: Create and update personal portfolios with a bio, links, and an image.
- **Vouches**: Request and approve vouches to build credibility.
- **Messaging**: Send and receive messages tied to the portfolio.
- **Tips**: Support portfolio owners with direct tips.

---

## Prerequisites

Before running the program, ensure you have the following installed:

1. **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
2. **Anchor CLI**: [Install Anchor](https://book.anchor-lang.com/chapter_3/installation.html)
3. **Solana CLI**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
4. **Node.js** (optional for testing): [Install Node.js](https://nodejs.org)

---

## Running the Program

### 1. Clone the Repository

```bash
git clone <repository-url>
cd portfolio-program
```

### 2. Build the Program

Compile the program with Anchor:

```bash
anchor build
```

### 3. Deploy the Program

Deploy to your local Solana validator:

```bash
anchor deploy
```

### 4. Set Up Local Validator

Start a Solana local validator:

```bash
solana-test-validator
```

Connect Anchor to the local cluster:

```bash
solana config set --url localhost
```

---

## Usage

### Initialize the Program

Run the `initialize` instruction to create a new portfolio:

```bash
anchor test --skip-build
```

### Key Instructions

1. **Create Portfolio**: Set up a portfolio with a bio.
2. **Store Links**: Add links to the portfolio.
3. **Store Image**: Upload an image URL to the portfolio.
4. **Request Vouch**: Request a vouch from another user.
5. **Approve Vouch**: Approve a pending vouch.
6. **Send Message**: Send a message to the portfolio owner.
7. **Tip**: Send tips to the portfolio owner.

---

## Testing

Anchor includes a robust testing framework. To run the test suite, execute:

```bash
anchor test
```

---

## Program Structure

- **Portfolio PDA**: Stores portfolio details, including:
  - Owner’s public key.
  - Bio, links, and image URL.
  - Vouches and vouch requests.
  - Messages and tips received.

- **Error Codes**:
  - `Unauthorized`: Triggered if the action is performed by someone other than the owner.

---

## Contributing

We welcome contributions to enhance the program. Fork the repository and submit a pull request. For major changes, open an issue to discuss your ideas.

---

## License

This project is open-source and available under the [MIT License](LICENSE).