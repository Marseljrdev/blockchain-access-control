use std::collections::HashMap;
use crate::node_runtime::AccountId;

// Enum que define os papéis possíveis na aplicaçao
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    Seller,
    Trader,
    Admin,
}

// Estrutura de controle de acesso, que mapeia contas -> papeis
pub struct AccessControl {
    roles: HashMap<AccountId, Role>,
}

impl AccessControl {
    // Cria uma nova instancia vazia
    pub fn new() -> Self {
        Self {
            roles: HashMap::new(),
        }
    }

    // Atribui um papel a uma conta
    pub fn add_role(&mut self, account: &AccountId, role: Role) {
        self.roles.insert(account.clone(), role);
        println!("[EVENT] RoleAdded({:?}, {:?})", account, role);
    }

    // Remove o papel de uma conta
    pub fn remove_role(&mut self, account: &AccountId) {
        if self.roles.remove(account).is_some() {
            println!("[EVENT] RoleRemoved({:?})", account);
        }
    }

    // Atualiza o papel de uma conta
    pub fn update_role(&mut self, account: &AccountId, new_role: Role) {
        if self.roles.contains_key(account) {
            self.roles.insert(account.clone(), new_role);
            println!("[EVENT] RoleUpdated({:?}, {:?})", account, new_role);
        }
    }

    // Verifica se a conta possui determinado papel
    pub fn has_role(&self, account: &AccountId, role: Role) -> bool {
        self.roles.get(account) == Some(&role)
    }

    // Imprime todos os pares conta ->> papel
    pub fn print_roles(&self) {
        for (account, role) in &self.roles {
            println!("[ROLE] {} => {:?}", account, role);
        }
    }
}
