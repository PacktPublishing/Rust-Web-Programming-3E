use std::marker::PhantomData;


struct InProgress;
struct Committed;
struct RolledBack;


struct Transaction<State> {
    id: u32,
    state: PhantomData<State>,
}

impl Transaction<InProgress> {
    fn new(id: u32) -> Self {
        Transaction {
            id,
            state: PhantomData,
        }
    }

    fn commit(self) -> Transaction<Committed> {
        println!("Transaction {} committed.", self.id);
        Transaction {
            id: self.id,
            state: PhantomData,
        }
    }

    fn rollback(self) -> Transaction<RolledBack> {
        println!("Transaction {} rolled back.", self.id);
        Transaction {
            id: self.id,
            state: PhantomData,
        }
    }
}

fn process_in_progress_transaction(tx: &Transaction<InProgress>) {
    println!("Processing transaction {} which is in progress.", tx.id);
}

fn main() {
    let tx: Transaction<InProgress> = Transaction::<InProgress>::new(1);
    let tx = tx.commit();
    process_in_progress_transaction(&tx);
}
