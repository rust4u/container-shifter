const MAX_ITEMS: usize = 3;

#[derive(Debug, PartialEq)]
struct Element {
    color: String,
}

#[derive(Debug, Default, PartialEq)]
struct Container {
    elements: Vec<Option<Element>>,
}

impl Container {
    fn new() -> Self {
        Self {
            elements: Vec::with_capacity(MAX_ITEMS),
        }
    }

    fn add_element(&mut self, value: Element) {
        if self.is_full() {
            println!("Container is full!");
            return;
        }
        self.elements.push(Some(value));
    }

    fn is_full(&self) -> bool {
        self.elements.len() >= MAX_ITEMS
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn display(&self) {
        println!("{:?}", self);
    }
}

fn pour(from: &mut Container, to: &mut Container) {
    if std::ptr::eq(from, to) {
        println!("Cannot pour from the same container!");
        return;
    }

    if from.is_empty() {
        println!("Source container is empty!");
        return;
    }

    if let Some(element) = from.elements.pop() {
        if to.is_full() {
            println!("Destination container is full!");
            from.elements.push(element);
            return;
        }
        to.elements.push(element);
    }
}

fn main() {
    let mut containers = [Container::new(), Container::new(), Container::new()];
    let colors = ["Red", "Green", "Blue"];

    for (i, container) in containers.iter_mut().enumerate() {
        let color = colors[i];
        container.add_element(Element {
            color: color.to_string(),
        })
    }

    for c in &containers {
        c.display();
    }

    println!("---");

    // Pop and extract containers individually
    let mut container0 = std::mem::take(&mut containers[0]);
    let mut container1 = std::mem::take(&mut containers[1]);
    let mut container2 = std::mem::take(&mut containers[2]);

    // Perform pour operationns
    pour(&mut container0, &mut container1);
    pour(&mut container1, &mut container2);
    // pour(&mut container2, &mut container0);

    // Assign them back to the vector after modifications
    containers[0] = container0;
    containers[1] = container1;
    containers[2] = container2;

    // Display final state of containers
    for c in &containers {
        c.display();
    }
}

#[cfg(test)]
mod tests {
    use crate::{Container, Element};

    use super::*;

    #[test]
    fn test_add_element() {
        let mut container = Container::new();
        assert!(container.is_empty());

        container.add_element(Element {
            color: "Red".to_string(),
        });
        assert_eq!(container.elements.len(), 1);
        assert!(!container.is_empty());

        container.add_element(Element {
            color: "Green".to_string(),
        });
        container.add_element(Element {
            color: "Blue".to_string(),
        });
        assert_eq!(container.elements.len(), 3);
        assert!(container.is_full());

        // Attempt to add an element when full
        container.add_element(Element {
            color: "Yellow".to_string(),
        });
        assert_eq!(container.elements.len(), 3);
    }

    #[test]
    fn test_pour_from_empty_container() {
        let mut container1 = Container::new();
        let mut container2 = Container::new();

        // Attempt to pour from an empty container
        pour(&mut container1, &mut container2);
        assert!(container2.is_empty());
    }

    #[test]
    fn test_pour_into_full_container() {
        let mut container1 = Container::new();
        let mut container2 = Container::new();

        container1.add_element(Element {
            color: "Red".to_string(),
        });
        container1.add_element(Element {
            color: "Green".to_string(),
        });
        container1.add_element(Element {
            color: "Blue".to_string(),
        });

        container2.add_element(Element {
            color: "Yellow".to_string(),
        });
        container2.add_element(Element {
            color: "Purple".to_string(),
        });
        container2.add_element(Element {
            color: "Orange".to_string(),
        });

        // Attempt to pour when the destination is full
        pour(&mut container1, &mut container2);
        assert_eq!(container1.elements.len(), 3);
        assert_eq!(container2.elements.len(), 3);
    }

    #[test]
    fn test_successful_pour() {
        let mut container1 = Container::new();
        let mut container2 = Container::new();

        container1.add_element(Element {
            color: "Red".to_string(),
        });

        // Pour from container1 to container2
        pour(&mut container1, &mut container2);
        assert_eq!(container1.elements.len(), 0);
        assert_eq!(container2.elements.len(), 1);
        assert_eq!(
            container2.elements[0],
            Some(Element {
                color: "Red".to_string()
            })
        );
    }
}
