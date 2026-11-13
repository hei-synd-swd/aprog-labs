#[derive(Debug)]
struct Pizza {
    size: String,
    toppings: Vec<String>,
}

struct PizzaBuilder {
    // TODO: Add a field containing a `Pizza`
}

impl PizzaBuilder {
    // TODO: Implement `new()` that creates a default `Pizza` with "medium" size and empty toppings
    // fn new() -> Self { ... }

    // TODO: Implement chaining method `size(self, s: &str) -> Self`
    // fn size(mut self, s: &str) -> Self { ... }

    // TODO: Implement chaining method `topping(self, t: &str) -> Self`
    // fn topping(mut self, t: &str) -> Self { ... }

    // TODO: Implement `build(self) -> Pizza` that consumes the builder
    // fn build(self) -> Pizza { ... }
}

fn main() {
    // TODO: Build a pizza with size "large" and at least two toppings, then print with Debug
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pizza() {
        let pizza = PizzaBuilder::new().build();
        assert_eq!(pizza.size, "medium");
        assert!(pizza.toppings.is_empty());
    }

    #[test]
    fn test_custom_pizza() {
        let pizza = PizzaBuilder::new()
            .size("large")
            .topping("cheese")
            .topping("pepperoni")
            .build();
        assert_eq!(pizza.size, "large");
        assert_eq!(pizza.toppings, vec!["cheese", "pepperoni"]);
    }
}
