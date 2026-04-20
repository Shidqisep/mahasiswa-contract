#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

// Struktur data Mahasiswa yang baru
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MahasiswaData {
    pub nim: String,
    pub nama: String,
    pub jurusan: String,
    pub tahun_masuk: u32,
}

#[contract]
pub struct MahasiswaContract;

#[contractimpl]
impl MahasiswaContract {

    // 1. CREATE & UPDATE
    // Return: true = sukses
    pub fn set_mahasiswa(env: Env, nim: String, nama: String, jurusan: String, tahun_masuk: u32) -> bool {
        let mhs_data = MahasiswaData {
            nim: nim.clone(),
            nama,
            jurusan,
            tahun_masuk,
        };

        env.storage().persistent().set(&nim, &mhs_data);
        true
    }

    // 2. READ
    // Return: Some(data) atau None
    pub fn get_mahasiswa(env: Env, nim: String) -> Option<MahasiswaData> {
        env.storage().persistent().get(&nim)
    }

    // 3. DELETE
    // Return: true jika data ada & berhasil dihapus, false jika tidak ada
    pub fn delete_mahasiswa(env: Env, nim: String) -> bool {
        if env.storage().persistent().has(&nim) {
            env.storage().persistent().remove(&nim);
            true
        } else {
            false
        }
    }

    // 4. UPDATE JURUSAN
    // Return: true jika berhasil, false jika NIM tidak ditemukan
    pub fn update_jurusan(env: Env, nim: String, jurusan_baru: String) -> bool {
        if let Some(mut mhs) = env.storage().persistent().get::<String, MahasiswaData>(&nim) {
            mhs.jurusan = jurusan_baru;
            env.storage().persistent().set(&nim, &mhs);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test;