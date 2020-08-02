#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.2")]
mod erc20 {
    use ink_core::storage;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Erc20 {
        /// 总供应量
        total_supply: storage::Value<Balance>,
        /// 每个账户的余额
        balances: storage::HashMap<AccountId, Balance>,
        /// 每个账户授权指定账户转账的额度 key:(自己账户，授权账户)
        allowance: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    /// 合约初始化事件
    #[ink(event)]
    struct ContractNew {
        #[ink(topic)]
        creator: Option<AccountId>,
    }

    /// 转移事件
    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// 授权额度事件
    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, init_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(init_supply);
            self.balances.insert(caller, *self.total_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: *self.total_supply,
            });
            self.env().emit_event(ContractNew {
                creator: Some(caller)
            })
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        /// 查询自己授权给花费方的额度
        #[ink(message)]
        fn approval(&self, spender: AccountId) -> Balance {
            let caller = self.env().caller();
            self.approval_or_zero(caller, spender)
        }

        fn approval_or_zero(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        ///【公开调用】设置授权花费方，允许转移你资产的额度
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            self.approve_internal(caller, spender, value)
        }

        /// 【内部调用】设置授权花费方，允许转移你资产的额度
        fn approve_internal(&mut self, owner: AccountId, spender: AccountId, value: Balance) -> bool {
            self.allowance.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                value,
            });
            true
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            self.transfer_from_to(caller, to, value)
        }

        /// 从from到to账户的转账，如果from跟合约调用方相同，那么则转账不受allowance控制，反之则受allowance控制
        #[ink(message)]
        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // 检查调用方跟fromAccount是否相同
            let caller = self.env().caller();
            let from_owner_self = (caller == from);
            if !from_owner_self {
                let left_allowance = self.approval_or_zero(from, caller);
                if left_allowance < value {
                    // 可转移值大于允许的值，直接返回false
                    return false;
                }
            }

            let from_account_balance = self.balance_of_or_zero(&from);
            if from_account_balance < value {
                // 账户剩余值小于转移值
                return false;
            }
            // from账户扣减
            self.balances.insert(from, from_account_balance - value);
            // to账户增加
            let to_account_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_account_balance + value);

            // 更新授权可转移额度
            if !from_owner_self {
                let left_allowance = self.approval_or_zero(from, caller);
                self.approve_internal(from, caller, left_allowance - value);
            }

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            true
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_core::env::AccountId;

        /// 合约创建测试
        #[test]
        fn contract_new_works() {
            let contract = Erc20::new(99);
            assert_eq!(contract.total_supply(), 99);
        }

        /// 余额测试
        #[test]
        fn balance_works() {
            let contract = Erc20::new(99);
            assert_eq!(contract.total_supply(), 99);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 99);
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 0);
        }

        /// 自己账户转账测试
        #[test]
        fn transfer_from_owner_works() {
            let mut contract = Erc20::new(99);
            assert_eq!(contract.total_supply(), 99);
            assert!(contract.transfer(AccountId::from([0x2; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 10);
        }

        /// allowance测试
        #[test]
        fn allownce_works() {
            let mut contract = Erc20::new(99);
            assert_eq!(contract.total_supply(), 99);
            assert!(contract.approve(AccountId::from([0x3; 32]), 18));
            assert_eq!(contract.approval(AccountId::from([0x3; 32])), 18);
        }

        /// 通用转账测试
        #[test]
        fn transfer_common_works() {
            let mut contract = Erc20::new(99);
            assert_eq!(contract.total_supply(), 99);
            // 0x1 授权其他账户
            assert!(contract.approve(AccountId::from([0x3; 32]), 18));
            assert_eq!(contract.approval(AccountId::from([0x3; 32])), 18);

            assert!(contract.transfer_from_to(AccountId::from([0x1; 32]), AccountId::from([0x4; 32]), 15));
            assert_eq!(contract.balance_of(AccountId::from([0x4; 32])), 15)
        }
    }
}
