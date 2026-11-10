// MICRO-OS ROUND ROBIN SCHEDULER

const TASK_COUNT: usize = 3;

// TASK STRUCTURE

struct Task {
    id: usize,
    name: &'static str,
}

// TASK LIST

static TASKS: [Task; TASK_COUNT] = [
    Task {
        id: 1,
        name: "Task 1 - Keyboard",
    },
    Task {
        id: 2,
        name: "Task 2 - Memory",
    },
    Task {
        id: 3,
        name: "Task 3 - System Monitor",
    },
];

// ROUND ROBIN SCHEDULER

pub fn run_demo() {

    crate::kprintln!(" ");
    crate::kprintln!("===== ROUND ROBIN SCHEDULER =====");

    for round in 1..=3 {

        crate::kprintln!("Round {}:", round);

        for task in TASKS.iter() {

            crate::kprintln!(
                "Running Task {} : {}",
                task.id,
                task.name
            );
        }
    }

    crate::kprintln!("All tasks completed!");
    crate::kprintln!("================================");
}
