// TODO: define the `Ticket` struct with `description: String`.

// TODO: `buy(description: &str) -> Ticket`
// fn buy(description: &str) -> Ticket { ... }

// TODO: `check(ticket: &Ticket) -> &str`
// fn check(ticket: &Ticket) -> &str { ... }

// TODO: `redeem(ticket: Ticket)`
// fn redeem(ticket: Ticket) { ... }

// TODO: `upgrade(ticket: &mut Ticket, new_desc: &str)`
// fn upgrade(ticket: &mut Ticket, new_desc: &str) { ... }

fn main() {
    let mut t = buy("Concert");
    println!("check: {}", check(&t));
    upgrade(&mut t, "VIP Concert");
    println!("check: {}", check(&t));
    redeem(t);
    // println!("{}", check(&t));  // try uncommenting — ownership was moved!
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_and_check() {
        let t = buy("Show");
        assert_eq!(check(&t), "Show");
    }

    #[test]
    fn test_upgrade() {
        let mut t = buy("Standard");
        upgrade(&mut t, "Premium");
        assert_eq!(check(&t), "Premium");
    }

    #[test]
    fn test_redeem_prints() {
        let t = buy("Test");
        // redeem should not panic and should print the description
        redeem(t);
        // t is moved, can't use it here
    }

    #[test]
    fn test_check_after_upgrade() {
        let mut t = buy("Basic");
        assert_eq!(check(&t), "Basic");
        upgrade(&mut t, "Deluxe");
        assert_eq!(check(&t), "Deluxe");
    }
}
