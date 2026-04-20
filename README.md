# Mahasiswa Smart Contract (Soroban)

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue.svg)](https://stellar.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Contract ID](https://img.shields.io/badge/Contract%20ID-CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY-orange.svg)](#contract-details)

A simple yet powerful CRUD (Create, Read, Update, Delete) smart contract for managing student records on the Stellar blockchain using Soroban SDK. This contract demonstrates best practices for persistent storage, data management, and educational blockchain applications.

id=CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY

screenshot
<img width="1919" height="1015" alt="image" src="https://github.com/user-attachments/assets/b5ad02ac-b395-48fb-a4dd-7b89247c403e" />


## 📋 Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Data Structure](#-data-structure)
- [Smart Contract Functions](#-smart-contract-functions)
- [Installation & Setup](#-installation--setup)
- [Usage Examples](#-usage-examples)
- [Storage Architecture](#-storage-architecture)
- [Testing](#-testing)
- [Security Considerations](#-security-considerations)
- [Project Structure](#-project-structure)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Overview

This smart contract enables educational institutions or student management systems to:

- **Store** student records persistently on the Stellar blockchain
- **Retrieve** student information by their unique ID (NIM)
- **Update** individual student fields without full data replacement
- **Delete** student records when necessary

Each student is uniquely identified by their **NIM** (Student ID), which serves as the primary key for all operations.

## ✨ Features

✅ **CRUD Operations** - Full Create, Read, Update, Delete functionality  
✅ **Persistent Storage** - Data stored permanently on-chain via Soroban  
✅ **Type-Safe** - Built with Rust for memory safety and type safety  
✅ **Efficient Updates** - Selectively update fields without rewriting entire records  
✅ **Optional Returns** - Graceful handling of missing data  
✅ **Educational Focus** - Clean code with detailed comments  
✅ **Testable** - Comprehensive unit test suite included  
✅ **Open Source** - Available for educational and commercial use  

## 🧱 Data Structure

### MahasiswaData

```rust
pub struct MahasiswaData {
    pub nim: String,
    pub nama: String,
    pub jurusan: String,
    pub tahun_masuk: u32,
}
```

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `nim` | `String` | Unique Student ID (Primary Key) | `"20240001"` |
| `nama` | `String` | Student Name | `"Ahmad Wijaya"` |
| `jurusan` | `String` | Major / Department | `"Computer Science"` |
| `tahun_masuk` | `u32` | Enrollment Year | `2024` |

## ⚙️ Smart Contract Functions

### 1. Create / Update Student (set_mahasiswa)

```rust
pub fn set_mahasiswa(
    env: Env,
    nim: String,
    nama: String,
    jurusan: String,
    tahun_masuk: u32
)
```

**Purpose:** Create a new student record or update an existing one.

**Parameters:**
- `env` - Soroban environment (contract context)
- `nim` - Student ID (unique identifier)
- `nama` - Student's full name
- `jurusan` - Major or department
- `tahun_masuk` - Year of enrollment

**Behavior:**
- Creates a new record if NIM doesn't exist
- Overwrites all fields if NIM already exists
- Stores data persistently on-chain

**Example:**
```
set_mahasiswa("20240001", "Ahmad Wijaya", "Computer Science", 2024)
```

---

### 2. Retrieve Student Data (get_mahasiswa)

```rust
pub fn get_mahasiswa(
    env: Env,
    nim: String
) -> Option<MahasiswaData>
```

**Purpose:** Fetch student information by their NIM.

**Parameters:**
- `env` - Soroban environment
- `nim` - Student ID to look up

**Returns:**
- `Some(MahasiswaData)` - If student exists
- `None` - If student not found

**Example:**
```
let student = get_mahasiswa("20240001");
// Returns: Some(MahasiswaData { ... }) or None
```

---

### 3. Delete Student Record (delete_mahasiswa)

```rust
pub fn delete_mahasiswa(
    env: Env,
    nim: String
)
```

**Purpose:** Remove a student record from storage.

**Parameters:**
- `env` - Soroban environment
- `nim` - Student ID to delete

**Behavior:**
- Removes the complete record for the specified NIM
- Does nothing if student doesn't exist
- Operation is permanent and irreversible

**Example:**
```
delete_mahasiswa("20240001")
```

---

### 4. Update Major Only (update_jurusan)

```rust
pub fn update_jurusan(
    env: Env,
    nim: String,
    jurusan_baru: String
)
```

**Purpose:** Update only the `jurusan` field without affecting other data.

**Parameters:**
- `env` - Soroban environment
- `nim` - Student ID to update
- `jurusan_baru` - New major/department name

**Behavior:**
- Only modifies the `jurusan` field
- Preserves `nama` and `tahun_masuk`
- Silent no-op if student doesn't exist
- More gas-efficient than full re-write

**Example:**
```
update_jurusan("20240001", "Information Technology")
```

---

## 📦 Installation & Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Soroban CLI](https://developers.stellar.org/docs/build/smart-contracts)
- Stellar testnet account

### Clone the Repository

```bash
git clone https://github.com/yourusername/mahasiswa-smart-contract.git
cd mahasiswa-smart-contract
```

### Build the Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mahasiswa_smart_contract.wasm \
  --source <your-account-secret-key> \
  --network testnet
```

This will output your contract ID. For this project:
```
Contract ID: CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY
```

## 💡 Usage Examples

### Creating a Student Record

```bash
soroban contract invoke \
  --id CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY \
  --source <your-key> \
  --network testnet \
  -- set_mahasiswa \
  --nim "20240001" \
  --nama "Ahmad Wijaya" \
  --jurusan "Computer Science" \
  --tahun_masuk 2024
```

### Retrieving Student Information

```bash
soroban contract invoke \
  --id CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY \
  --source <your-key> \
  --network testnet \
  -- get_mahasiswa \
  --nim "20240001"
```

**Expected Response:**
```
MahasiswaData {
  nim: "20240001",
  nama: "Ahmad Wijaya",
  jurusan: "Computer Science",
  tahun_masuk: 2024
}
```

### Updating a Student's Major

```bash
soroban contract invoke \
  --id CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY \
  --source <your-key> \
  --network testnet \
  -- update_jurusan \
  --nim "20240001" \
  --jurusan_baru "Information Technology"
```

### Deleting a Student Record

```bash
soroban contract invoke \
  --id CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY \
  --source <your-key> \
  --network testnet \
  -- delete_mahasiswa \
  --nim "20240001"
```

## 💾 Storage Architecture

This contract uses **Soroban's persistent storage** for reliable, permanent data management:

```rust
env.storage().persistent()
```

**Storage Details:**
- **Storage Type:** Persistent (on-chain)
- **Key Format:** NIM (Student ID) as String key
- **Data Durability:** Permanent unless explicitly deleted
- **Access Pattern:** O(1) lookup by NIM
- **Cost:** Minimal storage overhead per record

**Storage Layout:**
```
Storage Root
├── Key: "20240001" → Value: MahasiswaData { ... }
├── Key: "20240002" → Value: MahasiswaData { ... }
└── Key: "20240003" → Value: MahasiswaData { ... }
```

## 🧪 Testing

The contract includes comprehensive unit tests:

```bash
cargo test
```

### Test Coverage

- ✅ **Create Operations** - Verify new records are stored correctly
- ✅ **Read Operations** - Test data retrieval and None handling
- ✅ **Update Operations** - Ensure selective field updates work
- ✅ **Delete Operations** - Verify removal and state consistency
- ✅ **Edge Cases** - Handle missing data gracefully
- ✅ **Data Integrity** - Confirm data remains unchanged unless modified

### Running Specific Tests

```bash
cargo test test_set_mahasiswa -- --nocapture
cargo test test_get_mahasiswa -- --nocapture
cargo test test_update_jurusan -- --nocapture
cargo test test_delete_mahasiswa -- --nocapture
```

## 🔒 Security Considerations

### Current Implementation

⚠️ **Important:** This is an educational contract with minimal access controls.

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust naming conventions (snake_case for functions)
- Add tests for new features
- Update documentation in PR description
- Ensure `cargo test` passes locally
- Keep commits atomic and meaningful

## 📚 Learning Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Smart Contracts](https://developers.stellar.org/docs/build/smart-contracts)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Soroban SDK Examples](https://github.com/stellar/rs-soroban-sdk)

## 🐛 Bug Reports & Issues

Found a bug? Have a suggestion? Please open an [Issue](https://github.com/yourusername/mahasiswa-smart-contract/issues) with:

- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (Rust version, OS, etc.)

## 🎯 Roadmap

### v1.0 (Current)
- ✅ Basic CRUD operations
- ✅ Persistent storage
- ✅ Unit tests

### v1.1 (Planned)
- 🔄 Admin role access control
- 🔄 Batch operations (create multiple students)
- 🔄 Query all students (with pagination)
- 🔄 Audit logging

### v2.0 (Future)
- 🔄 Authentication system
- 🔄 Permission levels (view, edit, delete)
- 🔄 Data encryption
- 🔄 Contract upgrade mechanism

---

**Last Updated:** April 2026 
**Maintained By:** Your Name / Your Organization  
**Current Version:** 1.0.0
