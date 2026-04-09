/*
* The Story
* It's Halloween night. You've been hired to program the vending machine at Dracula's carnival.
The machine stocks 5 spooky items across 5 slots. Carnival-goers walk up, pick a slot number,
insert their coins — and either walk away with something cursed, or get rejected by the machine
(which, honestly, is also a Halloween experience).

* You're building a vending machine for a Halloween carnival. It stocks items like Popcorn,
SpookyJuice, WitchBrew, and EmptySlot. Each item has a price (f64) and a name. A user inputs a coin amount; the machine must return either the item + change or an error if: (a) slot is empty, (b) insufficient coins.
Model the inventory as a Vec of slots, use a Result to handle vend outcomes, and implement a method on your vending machine struct.

*/
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Item {
    name: String,
    spook_level: u8,
}

#[derive(Debug)]
struct Slot {
    slot_number: u32,
    item: Option<Item>,
    price: f64,
}

#[derive(Debug)]
struct VendingMachine {
    slots: Vec<Slot>,
}

#[derive(Debug)]
enum VendError {
    SlotNotFound(u32),
    SlotEmpty(u32),
    InsufficientFunds { required: f64, inserted: f64 },
    SlotAlreadyOccupied(u32),
}

impl Display for VendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VendError::SlotNotFound(slot) => {
                write!(f, "Slot {} doesn't exist in this dimension.", slot)
            }
            VendError::SlotEmpty(slot) => write!(
                f,
                "Slot {} is emptied — even ghosts left this one behind.",
                slot
            ),
            VendError::InsufficientFunds { required, inserted } => {
                write!(
                    f,
                    "You inserted {}, but it costs {}. {} more coins, cheapskate...",
                    inserted,
                    required,
                    required - inserted
                )
            }
            VendError::SlotAlreadyOccupied(slot) => {
                write!(f, "Slot {} is occupied.", slot)
            }
        }
    }
}

impl VendingMachine {
    fn new() -> Self {
        let mut slots: Vec<Slot> = Vec::new();
        slots.push(Slot {
            slot_number: 1,
            item: Some(Item {
                name: "Witch's Brew".to_string(),
                spook_level: 4,
            }),
            price: 15.00,
        });
        slots.push(Slot {
            slot_number: 2,
            item: Some(Item {
                name: "Candy Eyeball".to_string(),
                spook_level: 2,
            }),
            price: 8.50,
        });
        slots.push(Slot {
            slot_number: 3,
            item: Some(Item {
                name: "Ghost Popcorn".to_string(),
                spook_level: 3,
            }),
            price: 12.00,
        });
        slots.push(Slot {
            slot_number: 4,
            item: None,
            price: 0.00,
        });
        slots.push(Slot {
            slot_number: 5,
            item: Some(Item {
                name: "Vampire Juice".to_string(),
                spook_level: 5,
            }),
            price: 20.00,
        });

        VendingMachine { slots }
    }

    fn vend(&mut self, slot: u32, inserted: f64) -> Result<(Item, f64), VendError> {
        match self.slots.iter_mut().find(|sl| sl.slot_number == slot) {
            Some(res) => match res.item.take() {
                Some(item) => {
                    if res.price > inserted {
                        res.item = Some(item);
                        Err(VendError::InsufficientFunds {
                            required: res.price,
                            inserted,
                        })
                    } else {
                        Ok((item, inserted - res.price))
                    }
                }
                None => Err(VendError::SlotEmpty(slot)),
            },
            None => Err(VendError::SlotNotFound(slot)),
        }
    }

    fn restock(&mut self, slot: u32, item: Item, price: f64) -> Result<(), VendError> {
        match self.slots.iter_mut().find(|sl| sl.slot_number == slot) {
            Some(res) => match &res.item {
                Some(_) => Err(VendError::SlotAlreadyOccupied(slot)),
                None => {
                    res.item = Some(item);
                    res.price = price;
                    Ok(())
                }
            },
            None => Err(VendError::SlotNotFound(slot)),
        }
    }
}

fn vend(vm: &mut VendingMachine, slot: u32, price: f64) {
    match vm.vend(slot, price) {
        Ok((item, change)) => println!(
            "Dispensing: {} (spook level: {}) | Change: ₹{}",
            item.name, item.spook_level, change
        ),
        Err(e) => match e {
            VendError::SlotNotFound(_slot) => {
                println!("Error: {}", e)
            }
            VendError::SlotEmpty(_slot) => {
                println!("Error: {}", e)
            }
            VendError::InsufficientFunds {
                required: _required,
                inserted: _inserted,
            } => println!("Error: {}", e),
            VendError::SlotAlreadyOccupied(_slot) => println!("Error: {}", e),
        },
    }
}

fn restock(vm: &mut VendingMachine, slot: u32, price: f64, item: Item) {
    let name = item.name.clone();
    match vm.restock(slot, item, price) {
        Ok(()) => println!("Restocked slot {} with {}", slot, name),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let mut vm = VendingMachine::new();

    println!(" initial slots: {:?}", vm.slots);

    // Attempt 1: successful vend with change
    let slot = 1;
    let price = 15.00;
    vend(&mut vm, slot, price);

    println!(" After attempt1 slots: {:?}", vm.slots);

    // Attempt 2: insufficient funds
    let slot = 2;
    let price = 6.00;
    vend(&mut vm, slot, price);

    println!(" After attempt2 slots: {:?}", vm.slots);

    // Attempt 3: empty slot
    let slot = 4;
    let price = 6.00;
    vend(&mut vm, slot, price);

    println!(" After attempt3 slots: {:?}", vm.slots);

    // Attempt 4: invalid slot number
    let slot = 800;
    let price = 6.00;
    vend(&mut vm, slot, price);

    println!(" After attempt4 slots: {:?}", vm.slots);

    // Attempt 5: vend the now-empty slot again (it was emptied in attempt 1)
    let slot = 1;
    let price = 15.00;
    vend(&mut vm, slot, price);

    println!(" After attempt5 slots: {:?}", vm.slots);

    // Attempt 6: restock slot 1, then vend it successfully
    let slot = 1;
    let price = 11.00;
    let item = Item {
        name: "Pumpkin Smoothie".to_string(),
        spook_level: 3,
    };
    restock(&mut vm, slot, price, item);

    println!(" After attempt6 slots: {:?}", vm.slots);
}
