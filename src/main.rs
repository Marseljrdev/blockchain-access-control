mod node_runtime;
mod access_control;

use access_control::{AccessControl, Role};
use node_runtime::AccountId;

fn main() {
    let mut access = AccessControl::new();

    let marsel: AccountId = "marsel".into();
    let flavio: AccountId = "flavio".into();
    let junior: AccountId = "junior".into();

    access.add_role(&marsel, Role::Admin);
    access.add_role(&flavio, Role::Seller);
    access.add_role(&junior, Role::Trader);

    println!("\n[INFO] Estado inicial:");
    access.print_roles();

    println!("\n[CHECK] flavio é Seller? {}", access.has_role(&flavio, Role::Seller));
    println!("[CHECK] junior é Admin? {}", access.has_role(&junior, Role::Admin));

    access.update_role(&flavio, Role::Trader);
    access.remove_role(&junior);

    println!("\n[INFO] Depois das mudanças:");
    access.print_roles();
}
