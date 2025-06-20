# MJR Task Blockchain – Access Control Pallet (Simulado)

Este projeto simula um pallet de controle de acesso em uma blockchain baseada em Substrate/Polkadot

### Funcionalidades:

- Define um enum `Role` com valores `Admin`, `Seller`, `Trader`.
- Mapeia contas (`AccountId`) para roles.
- Permite:
  - Adicionar papéis (`add_role`)
  - Remover papéis (`remove_role`)
  - Atualizar papéis (`update_role`)
  - Verificar permissões (`has_role`)
- Armazena dados em `HashMap` simulado.
- Organizado em 3 arquivos (`main.rs`, `access_control.rs`, `node_runtime.rs`)
