mod Network;
use Network::scanner;

fn main() {
    println!("=== Monitor de Red Iniciando ===");
    
    // Llamamos a la función dentro de la carpeta externa
    let dispositivos = network::scanner::obtener_info_red();
    
    // Aquí es donde pasamos los datos a la "interfaz"
    mostrar_en_interfaz(dispositivos);
}

fn mostrar_en_interfaz(datos: Vec<String>) {
    println!("\n--- RESULTADOS DE LA INTERFAZ ---");
    if datos.is_empty() {
        println!("No se detectaron interfaces activas.");
    } else {
        for linea in datos {
            println!("[VISTA]: {}", linea);
        }
    }
    println!("---------------------------------\n");
}