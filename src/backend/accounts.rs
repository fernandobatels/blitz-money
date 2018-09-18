use qtbindingsinterface::AccountsList;
use qtbindingsinterface::AccountsEmitter;
use qtbindingsinterface::AccountsTrait;

#[derive(Default, Clone)]
struct Account {
    bank: String,
    id: i32,
    name: String,
}

pub struct Accounts {
    emit: AccountsEmitter,
    #[allow(dead_code)]
    model: AccountsList,
    list: Vec<Account>,
}

impl AccountsTrait for Accounts {
    fn new(emit: AccountsEmitter, model: AccountsList) -> Accounts {
        let mut ac = Accounts {
            emit: emit,
            model: model,
            list: [].to_vec()
        };
        ac.list.push(Account{ bank: "BB".to_string(), id: 10, name: "Conta corrente".to_string()});
        ac
    }
    fn emit(&self) -> &AccountsEmitter {
        &self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn bank(&self, index: usize) -> &str {
        &self.list[index].bank
    }
    fn set_bank(&mut self, index: usize, v: String) -> bool {
        self.list[index].bank = v;
        true
    }
    fn id(&self, index: usize) -> i32 {
        self.list[index].id
    }
    fn set_id(&mut self, index: usize, v: i32) -> bool {
        self.list[index].id = v;
        true
    }
    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }
    fn set_name(&mut self, index: usize, v: String) -> bool {
        self.list[index].name = v;
        true
    }
}
