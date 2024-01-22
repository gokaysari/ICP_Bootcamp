fn main() {
    #[derive(Clone)]
    pub struct Task {
        id: usize,
        description: String,
        completed: bool,
    }

    pub struct TodoList {
        tasks: Vec<Task>,
    }

    impl TodoList {
        pub fn add_task(&mut self, description: String) -> Task {
            let id = self.tasks.len(); // Assuming the length of the vector gives a unique ID
            let task = Task {
                id,
                description,
                completed: false,
            };
            self.tasks.push(task.clone());
            task
        }

        pub fn complete_task(&mut self, id: usize) -> Option<&Task> {
            match self.tasks.iter_mut().find(|task| task.id == id) {
                Some(task) => {
                    task.completed = true;
                    Some(task)
                },
                None => None,
            }
        }

        pub fn list_tasks(&self) {
            for task in &self.tasks {
                println!(
                    "ID: {}, Description: {}, Completed: {}",
                    task.id, task.description, task.completed
                );
            }
        }
    }

    let mut todo_list = TodoList { tasks: vec![] };
    todo_list.add_task("Homework 1 Done".into());
    todo_list.add_task("Final Project Done".into());
    todo_list.list_tasks();
    todo_list.complete_task(0);
    todo_list.list_tasks();
}
