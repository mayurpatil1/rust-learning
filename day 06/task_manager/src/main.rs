// Day 6 — Consolidation: methods (impl), enums, iterators, if let
// A tiny in-memory task manager that pulls Week 1 together.
 
// An enum WITH a method attached. PartialEq lets us compare with == / !=.
#[derive(Debug, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    // A method on the enum: match on `self` to produce a label.
    fn label (&self) ->&str {
        match (self) {
            Status::Todo => "TODO",
            Status::InProgress => "IN PROGRESS",
            Status::Done => "DONE",
        }
    }
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: Status,
}
 
impl Task {
    // Associated function (no `self`) — like a Java static factory.
    // Call it as Task::new(...).

    fn new (id: u32, title: &str) -> Task {
        Task {
            id,                              // shorthand for `id: id`
            title: String::from(title),
            status: Status::Todo,
        }
    }

    // &mut self: a method that MODIFIES the instance.
    fn mark_done(&mut self) {
        self.status = Status::Done;
    }

    // &self: a method that only READS the instance.
    fn describe(&self) -> String {
        format!("#{} [{}] {}", self.id, self.status.label(), self.title)
    }

    fn mark_in_progress(&mut self) {
        self.status = Status::InProgress    ;
    }
} 

struct TaskList {
    tasks: Vec<Task>,
}


impl TaskList {
    fn new() -> TaskList {
        TaskList { tasks : Vec::new()}
    }


    fn add(&mut self, title: &str) {
        let id = self.tasks.len() as u32 + 1;  // simple incrementing id
        self.tasks.push(Task::new(id, title));
    }

    // Returns Option<&mut Task>: the matching task, or None.
    // iter_mut() + find() = "find the one I can then modify."
    fn find_mut(&mut self, id:u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t|  t.id== id)
    }

    fn complete(&mut self, id:u32) {
        // if let: handle ONLY the Some case, with an else for None.
        if let Some(task) = self.find_mut(id) {
            task.mark_done();
            println!("Completed: {}", task.describe());
        }  else {
            println!("No task with id {}.", id);
        }
    }   

   fn start(&mut self, id: u32) {
        if let Some(task) = self.find_mut(id) {
            task.mark_in_progress();
            println!("Started: {}", task.describe());
        } else {
            println!("No task with id {}.", id);
        }
    } 

    fn remove(&mut self, id: u32) {
    // Step 1: find WHERE the task sits in the vector (its index).
    // .position() returns Option<usize>: Some(index) if found, None if not.
    if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
        // Step 2: remove it at that index. .remove() returns the owned Task.
        let removed = self.tasks.remove(index);
        println!("Removed: {}", removed.describe());
    } else {
        println!("No task with id {}.", id);
    }
}

    fn show_all(&self) {
        println!("\nAll tasks:");
        for task in self.tasks.iter() {
            println!("  {}", task.describe());
        }
    }

    fn show_pending(&self) {
        let pending : Vec<&Task> =  self
            .tasks
            .iter()
            .filter(|t| t.status != Status::Done)
            .collect();
 
        println!("\nPending ({}):", pending.len());
        for task in pending {
            println!("  {}", task.describe());
        }
    }
    
    // Iterator query: count how many are done.
    fn summary(&self) {
        let done = self.tasks.iter().filter(|t| t.status == Status::Done).count();
        println!("\nProgress: {}/{} done.", done, self.tasks.len());
    }
}


fn main() {
    let mut list = TaskList::new();
 
    list.add("Learn ownership");
    list.add("Learn iterators");
    list.add("Build a project");
    list.add("Push to GitHub");
 
    list.show_all();
 
    list.start(1);      
    list.complete(2);
    list.remove(4);
    list.complete(99); // no such task — exercises if let's else branch
 
    list.show_pending();
    list.summary();
}
 