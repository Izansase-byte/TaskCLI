fn main() {
    let version = "0.1.1";
    mostrar_bienvenida(version);
    let mut tasks = Vec::new();

    agregar_tarea(&mut tasks, String::from("Tarea A"));
    agregar_tarea(&mut tasks, String::from("Tarea B"));
    agregar_tarea(&mut tasks, String::from("Tarea C"));

    eliminar_tarea(&mut tasks, 2);

    agregar_tarea(&mut tasks, String::from("Tarea D"));

    completar_tarea(&mut tasks, 4);

    listar_tareas(&tasks);

}
fn eliminar_tarea(tareas: &mut Vec<Task>, id: u32) {
    let  numero_tareas = tareas.iter().position(|tarea| tarea.id == id);
    match numero_tareas {
        Some(posicion) => {   
            tareas.remove(posicion);
            println!("Tarea con ID: {} eliminada!", id);
        }
        None => {
            println!("No existe la tarea con ID: {}", id);
}}}


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
    let id = tareas.iter().map(|tarea| tarea.id).max().unwrap_or(0) + 1;
    tareas.push(Task {
        id,
        nombre: texto,
        completada: false,
    })
}
fn completar_tarea(tareas: &mut Vec<Task>, id: u32) {
    let mut tarea_encontrada = false;
    
    for tarea in tareas.iter_mut() {
        if tarea.id == id {
            tarea.completada = true;
            println!("Tarea: {} completada!", tarea.id);
            tarea_encontrada = true;
            break;
        }}
    if tarea_encontrada ==false || id <= 0 {
    println!("No existe la tarea con ID: {}", id);
    return;
    }
}

struct Task {
    id: u32,
    nombre: String,
    completada: bool,
}
