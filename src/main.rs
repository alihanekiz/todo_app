mod project;
mod todo;

use project::Project;
use project::ProjectOperation;
use todo::Todo;
use todo::TodoOperation;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use serde_json::to_string;
use serde_json::from_str;
use colored::Colorize;


fn main() {
    let mut exit: bool = false;
    let mut projects: Vec<Project> = init_projects();
    if projects.is_empty() {
        projects.push(Project::new("Demo".to_string()));
    }
    loop {
        render_projects(&projects);
        let input = get_user_input().trim().to_string();
        let mut desc = input.to_string();
        desc.remove(0);
        desc = desc.trim().to_string();
        let op = parse_main_menu_input(input);
        match op {
            ProjectOperation::Open | ProjectOperation::Delete => {
                let index: usize = match desc.parse::<usize>() {
                    Ok(val) => {
                        if val >= projects.len() { select_active_project(&projects) }
                        else { val }
                    },
                    Err(_) => select_active_project(&projects)
                };
                if let ProjectOperation::Open = op {
                    while !exit {
                        exit = render_menu(&mut projects[index].todos);
                    }
                } else {
                    // DELETE
                    projects.remove(index);
                }
            },
            ProjectOperation::Create => {
                // create new project
                let p = Project::new(desc);
                projects.push(p);
            },
            ProjectOperation::Invalid => {
                println!("Operation not known.");
                continue;
            },
            ProjectOperation::Exit => {
                let s = to_string(&projects).expect("something went wrong!");
                let _ = write_to_file(s);
                println!("Quitting the app. Have a nice day!");
                return;
            }
        }
    }
}

fn load_file_content() -> io::Result<String> {
    let mut f = File::open("todos.json")?;

    let mut content: String = String::from("");

    let _ = f.read_to_string(&mut content);
    Ok(content)
}

fn parse_main_menu_input(input: String) -> ProjectOperation {
    if input.is_empty() { return ProjectOperation::Invalid; }
    let op: char = input.chars().next().unwrap();
    match op {
        '+' => ProjectOperation::Create,
        '-' => ProjectOperation::Delete,
        '>' => ProjectOperation::Open,
        '!' => ProjectOperation::Exit,
        _ => ProjectOperation::Invalid
    }
}

fn select_active_project(projects: &Vec<Project>) -> usize {
    loop {
        render_projects(projects);
        let uinpt = get_user_input().trim().to_string();

        match uinpt.parse::<usize>() {
            Ok(val) => {
                if val >= projects.len() { continue; }
                else { return val; }
            },
            Err(_) => println!("Input not recognized!"),
        };
    }
}

fn render_projects(projects: &Vec<Project>) {
    for (i, p) in projects.iter().enumerate() {
        println!("[{}] {}", i, p);
    }
}

fn write_to_file(content: String) -> io::Result<()> {
    let mut f = File::create("todos.json")?;

    let _ = f.write_all(content.as_bytes());

    Ok(())
}

fn init_projects() -> Vec<Project> {
    match load_file_content() {
        Ok(val) => from_str(&val).expect("Could not parse json."),
        Err(_) => vec!()
    }
}

fn get_user_input() -> String {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Err reading");
    i.to_string()
}

fn render_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "You have no Todos for today! Have a nice day!".on_green().black());
        return
    }
    for t in todos.iter() {
        println!("{}", t);
    }
}

fn parse_user_input(inpt: &String, todos: &mut Vec<Todo>) -> bool {
    if inpt.is_empty() { return false; }
    let mut desc = inpt.to_string();
    desc.remove(0);
    desc = desc.trim().to_string();
    let operation: char = inpt.chars().next().unwrap();
    match operation {
        '+' => {
            // Add Todo
            todos.push(Todo {
                desc,
                done: false
            });
            false
        },
        '-' => {
            // Remove Todo
            find_todo_and_operate(&desc, todos, TodoOperation::Delete);
            false
        },
        '*' => {
            // Mark Todo as done
            find_todo_and_operate(&desc, todos, TodoOperation::Mark);
            false
        },
        '!' => {
            // Exit program
            true
        },
        _ => {
            // Do nothing!
            println!("Command not recognized!");
            false
        }
    }
}

fn resolve_conflict(found_todos: Vec<&Todo>) -> Todo {
    let res: usize = loop {
        for (i, t) in found_todos.iter().enumerate() {
            println!("[{}] {}", i, t.desc);
        }
        let uinpt = get_user_input().trim().to_string();
        println!("your input: {}", uinpt);
        match uinpt.parse::<usize>() {
            Ok(a) => {
                if a >= found_todos.len() { println!("Invalid selection."); }
                else { break a; }
            },
            Err(_) => {
                println!("Could not parse your input!");
            }
        };
    };
    found_todos[res].clone()
}

fn find_todo_and_operate(inpt: &String, todos: &mut Vec<Todo>, op: TodoOperation) {
    let mut indexes: Vec<usize> = vec!();
    for (i, t) in todos.iter().enumerate() {
        if t.desc.contains(&*inpt) {
            indexes.push(i);
        }
    }
    match indexes.len() {
        2.. => {
            //resolve conflict
            println!("There has been a conflict when searching for a todo.");
            println!("Please resolve it!");
            let mut ts: Vec<&Todo> = vec!();
            for i in indexes.iter() {
                let td: &Todo = &todos[*i];
                ts.push(td);
            }
            let t: Todo = resolve_conflict(ts);
            match op {
                TodoOperation::Mark => {
                    todos.iter_mut().for_each(|a| { if *a == t { a.done = !a.done; } });
                },
                TodoOperation::Delete => {
                    todos.retain(|a| *a != t);
                }
            }
        },
        1 => {
            // remove item
            match op {
                TodoOperation::Delete => {
                    todos.remove(indexes[0]);
                },
                TodoOperation::Mark => {
                    todos[indexes[0]].done = !todos[indexes[0]].done;
                }
            }

        },
        _ => {
            // do nothing
            println!("Could not find todo.");
        }
    }
}

fn render_menu(todos: &mut Vec<Todo>) -> bool {
    println!("----------------------------");
    println!("{}", "Your Todos:".green());
    println!("----------------------------");
    render_todos(todos);
    let inpt = get_user_input();
    parse_user_input(&inpt, todos)
}
