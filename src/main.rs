fn main() {
    let version = "0.1.0";
    let mut tasks = Vec::new();
    mostrar_bienvenida(version);
    tasks.push(Task {
        id: 1,
        nombre: String::from("Estudiar fisica"),
        completada: false,
    });
    tasks.push(Task {
        id: 2,
        nombre: String::from("Programar rust"),
        completada: true,
    });
    tasks.push(Task {
        id: 3,
        nombre: String::from("Hacer deberes"),
        completada: false,
    });
    agregar_tarea(&mut tasks, String::from("Leer un libro"));

    listar_tareas(&tasks);
    println!("Tareas totales: {}", tasks.len());
}
fn mostrar_bienvenida(version: &str) {
    println!("Bienvenido a Task-CLI!");
    println!("Version: {}", version);
}
fn listar_tareas(tareas: &Vec<Task>) {
    for i in tareas {
        println!(
            "ID: {}, Nombre: {}, Completada: {}",
            i.id,
            i.nombre,
            if i.completada {
                "Completada"
            } else {
                "Pendiente"
            }
        );
    }
}
fn agregar_tarea(tareas: &mut Vec<Task>, nombre: String) {
    let texto = String::from(nombre);
    let id = tareas.len() as u32 + 1;
    tareas.push(Task {
        id,
        nombre: texto,
        completada: false,
    })
}

struct Task {
    id: u32,
    nombre: String,
    completada: bool,
}
