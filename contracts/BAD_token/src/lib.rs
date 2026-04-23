#![no_std] // Bắt buộc đối với Soroban (không dùng thư viện chuẩn của HĐH)
use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

// Định nghĩa contract
#[contract]
pub struct BadmintonLoyaltyToken;

#[contractimpl]
impl BadmintonLoyaltyToken {
    
    // 1. Hàm cấp điểm (Mint) - Gọi từ Java Backend khi hóa đơn thanh toán xong
    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        // Yêu cầu chữ ký xác thực từ Admin hệ thống
        admin.require_auth(); 
        
        let mut balance = Self::get_balance(env.clone(), to.clone());
        balance += amount;
        
        // Lưu trữ dữ liệu vào Blockchain
        env.storage().persistent().set(&to, &balance);
        
        // Ghi log sự kiện (Event) để ứng dụng Java bắt được
        env.events().publish((symbol_short!("Mint"), to), amount);
    }

    // 2. Hàm trừ điểm (Burn) - Khi khách hàng dùng điểm đổi nước/sân
    pub fn burn(env: Env, from: Address, amount: i128) {
        // Xác thực chính khách hàng (chủ ví) đang yêu cầu trừ điểm
        from.require_auth();
        
        let mut balance = Self::get_balance(env.clone(), from.clone());
        assert!(balance >= amount, "Khong du diem tich luy!"); // Báo lỗi nếu thiếu điểm
        
        balance -= amount;
        env.storage().persistent().set(&from, &balance);
    }

    // 3. Hàm xem số dư hiện tại (Read-only)
    pub fn get_balance(env: Env, user: Address) -> i128 {
        // Nếu ví chưa có điểm, mặc định trả về 0
        env.storage().persistent().get(&user).unwrap_or(0)
    }
}