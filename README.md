📚 Mahasiswa Smart Contract (Soroban)
id = CBG3JGAJAGO55VHCTINHEISZR57CNDHC43WSJD7KNRBS6WX63VZSWQGY

This project is a simple CRUD (Create, Read, Update, Delete) smart contract built using Soroban SDK on the Stellar blockchain. It manages student data (Mahasiswa) using persistent storage.

🚀 Overview

The contract allows you to:

Store student data
Retrieve student information
Update specific fields
Delete student records

Each student is uniquely identified by their NIM (Student ID).

🧱 Data Structure
pub struct MahasiswaData {
    pub nim: String,
    pub nama: String,
    pub jurusan: String,
    pub tahun_masuk: u32,
}
Field Description:
nim → Unique student ID (Primary Key)
nama → Student name
jurusan → Major / Department
tahun_masuk → Enrollment year
⚙️ Functions
1. Create / Update Mahasiswa
pub fn set_mahasiswa(env: Env, nim: String, nama: String, jurusan: String, tahun_masuk: u32)
Creates a new student record
Updates existing data if the NIM already exists
2. Read Mahasiswa
pub fn get_mahasiswa(env: Env, nim: String) -> Option<MahasiswaData>
Retrieves student data by NIM
Returns None if the data does not exist
3. Delete Mahasiswa
pub fn delete_mahasiswa(env: Env, nim: String)
Removes a student record from storage
4. Update Jurusan Only
pub fn update_jurusan(env: Env, nim: String, jurusan_baru: String)
Updates only the jurusan field
Keeps other data unchanged
Does nothing if the student is not found
💾 Storage

This contract uses:

env.storage().persistent()
Data is stored permanently on-chain
Uses nim as the key
🧪 Testing
#[cfg(test)]
mod test;

You can implement unit tests to verify each function’s behavior.

🛠️ Built With
Rust (#![no_std])
Soroban SDK
Stellar Smart Contracts
📌 Notes
This is a basic implementation intended for learning purposes
No validation or error handling is implemented (e.g., duplicate checks, input validation)
Can be extended with:
Access control (admin-only updates)
Events/logging
Pagination or listing all students
📈 Future Improvements
Add authorization (only owner can modify data)
Implement indexing or student listing
Add validation for input fields
Integrate with frontend (e.g., React + Soroban RPC)
📄 License

This project is open-source and available for educational use.