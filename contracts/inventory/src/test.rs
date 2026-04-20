#![cfg(test)]
use super::*;
use soroban_sdk::{Env, symbol_short};

#[test]
fn test_inventory_crud() {
    // 1. Siapkan Environment (Ini membuat simulasi blockchain lokal di memori)
    let env = Env::default();
    
    // 2. Daftarkan kontrak ke environment dan buat "Client" untuk memanggil fungsinya
    let contract_id = env.register_contract(None, InventoryContract);
    let client = InventoryContractClient::new(&env, &contract_id);

    // 3. Data *dummy* untuk testing
    // Gunakan symbol_short! untuk Symbol maksimal 9 karakter
    let item_id = symbol_short!("item_01");
    let name = symbol_short!("Laptop");
    let price = 15000000;
    let stock = 10;

    // ==========================================
    // 🟢 TEST 1: CREATE (Simpan Barang)
    // ==========================================
    client.set_item(&item_id, &name, &price, &stock);

    // ==========================================
    // 🔵 TEST 2: READ (Cek Barang)
    // ==========================================
    // Kita ambil datanya, gunakan unwrap() karena kembaliannya berupa Option<ItemData>
    let saved_item = client.get_item(&item_id).unwrap();
    
    // assert_eq! berfungsi untuk mengecek apakah data yang tersimpan SESUAI dengan yang kita input
    assert_eq!(saved_item.name, name);
    assert_eq!(saved_item.price, price);
    assert_eq!(saved_item.stock, stock);

    // ==========================================
    // 🟡 TEST 3: UPDATE (Ubah Stok)
    // ==========================================
    // Kita panggil fungsi update_stock menjadi 25
    client.update_stock(&item_id, &25);
    
    // Ambil lagi datanya dan pastikan stoknya sudah berubah
    let updated_item = client.get_item(&item_id).unwrap();
    assert_eq!(updated_item.stock, 25);

    // ==========================================
    // 🔴 TEST 4: DELETE (Hapus Barang)
    // ==========================================
    client.delete_item(&item_id);
    
    // Coba ambil lagi datanya. Seharusnya sekarang menjadi `None` (kosong/tidak ditemukan)
    let deleted_item = client.get_item(&item_id);
    assert!(deleted_item.is_none());
}