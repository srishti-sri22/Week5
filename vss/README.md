## 📚 Module Descriptions

### 🎯 `main.rs`
- **Purpose**: Entry point of the application
- **What it does**: Parses command-line arguments and starts the program
- **Beginner tip**: This is where execution begins - it's like the "start button"

### 🖥️ `cli.rs`
- **Purpose**: Defines the command-line interface
- **What it does**: Specifies what commands the program accepts (split, verify-secret, verify-share, reconstruct)
- **Beginner tip**: This tells Rust what arguments users can provide

### 🔐 `crypto/` Module

#### `params.rs`
- **Purpose**: Defines fixed cryptographic parameters
- **What it does**: Returns the large prime numbers (p, q) and generator (g) used in calculations
- **Beginner tip**: These are like the "settings" that make the crypto secure

#### `polynomial.rs`
- **Purpose**: Handles polynomial mathematics
- **What it does**: Evaluates a polynomial at a given point (used to create shares)
- **Beginner tip**: A polynomial is like `y = a + bx + cx²` - this calculates the y value

#### `feldman.rs`
- **Purpose**: Implements Feldman's Verifiable Secret Sharing
- **What it does**: Creates commitments and verifies shares
- **Beginner tip**: This is the "proof system" that lets you verify shares are correct

### 🧮 `math/` Module

#### `gcd.rs`
- **Purpose**: Extended Euclidean Algorithm
- **What it does**: Finds the greatest common divisor and modular inverses
- **Beginner tip**: This is needed for division in modular arithmetic

#### `lagrange.rs`
- **Purpose**: Lagrange interpolation
- **What it does**: Reconstructs the secret from shares
- **Beginner tip**: This is the mathematical magic that combines shares back into the secret

### ⚙️ `commands/` Module

#### `mod.rs`
- **Purpose**: Command dispatcher
- **What it does**: Routes each command to the right handler
- **Beginner tip**: This is like a switchboard connecting commands to their implementations

#### `split.rs`
- **Purpose**: Split command implementation
- **What it does**: Splits a secret into n shares (k needed to reconstruct)
- **Example**: `--secret "hello" --n 5 --k 3` creates 5 shares, any 3 can recreate "hello"

#### `verify_secret.rs`
- **Purpose**: Secret verification
- **What it does**: Checks if a secret matches its commitment
- **Example**: Verifies the dealer isn't lying about the secret

#### `verify_share.rs`
- **Purpose**: Share verification
- **What it does**: Checks if a share is valid without revealing the secret
- **Example**: Confirms a share hasn't been tampered with

#### `reconstruct.rs`
- **Purpose**: Secret reconstruction
- **What it does**: Combines k or more shares to recover the original secret
- **Example**: Takes shares `1,456;2,789;3,123` and outputs the original secret

## 🚀 How to Use

### Build the project
```bash
cargo build --release
```

### Split a secret
```bash
cargo run -- split --secret "mysecret" --n 5 --k 3
```

### Verify a secret
```bash
cargo run -- verify-secret --secret "mysecret" --commitments "123,456,789"
```

### Verify a share
```bash
cargo run -- verify-share --share "1,456" --commitments "123,456,789"
```

### Reconstruct a secret
```bash
cargo run -- reconstruct --shares "1,456;2,789;3,123"
```

## 🎓 Learning Path

If you're new to this codebase, study the files in this order:

1. **main.rs** - See how the program starts
2. **cli.rs** - Understand what commands are available
3. **commands/mod.rs** - See how commands are dispatched
4. **commands/split.rs** - Learn how secrets are split
5. **crypto/polynomial.rs** - Understand polynomial evaluation
6. **crypto/feldman.rs** - Learn about verification
7. **math/lagrange.rs** - Understand reconstruction
8. **commands/reconstruct.rs** - See the full reconstruction process

## 🔑 Key Concepts

- **Secret Sharing**: A way to split a secret so that k people must cooperate to recover it
- **Polynomial**: The mathematical tool used to create shares
- **Commitment**: A cryptographic "promise" that proves shares are valid
- **Lagrange Interpolation**: The math that combines shares back into the secret
- **Modular Arithmetic**: Math with wraparound (like a clock)

## 💡 Why This Structure?

- **Separation of Concerns**: Each file has one clear purpose
- **Easy to Test**: Small modules are easier to test individually
- **Easy to Understand**: Beginners can focus on one concept at a time
- **Easy to Extend**: Want to add a new command? Just add a new file in `commands/`
- **Maintainable**: Changes to crypto don't affect commands, and vice versa