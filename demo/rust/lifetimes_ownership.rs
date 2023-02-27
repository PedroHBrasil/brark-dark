struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Person<'a> {
    fn greeting(&self) -> String {
        format!(
            "Hello, my name is {} and I'm {} years old.",
            self.name, self.age
        )
    }
}

struct Company<'a> {
    employees: Vec<&'a Person<'a>>,
}

impl<'a> Company<'a> {
    fn new() -> Self {
        Company { employees: vec![] }
    }

    fn add_employee(&mut self, employee: &'a Person<'a>) {
        self.employees.push(employee);
    }
}

fn main() {
    let alice = Person {
        name: "Alice",
        age: 25,
    };
    let bob = Person {
        name: "Bob",
        age: 30,
    };
    let mut company = Company::new();
    company.add_employee(&alice);
    company.add_employee(&bob);
    for employee in company.employees {
        println!("{}", employee.greeting());
    }
}
