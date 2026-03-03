use netdev;

// Usamos 'pub' para que sea accesible desde fuera
pub fn obtener_info_red() -> Vec<String> {
    let mut resultados = Vec::new();
    
    for interface in netdev::get_interfaces() {
        if !interface.ipv4.is_empty() {
            let info = format!(
                "Interfaz: {} | IP: {:?} | MAC: {:?}",
                interface.name, interface.ipv4[0], interface.mac_addr
            );
            resultados.push(info);
        }
    }
    resultados
}