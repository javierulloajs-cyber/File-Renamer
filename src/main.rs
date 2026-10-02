use core::panic;
use std::{env, fs::self, path::PathBuf};


/*
0 - Ejecutable dirección
1 - Carpeta a modificar
2 - Modo (Agregar o eliminar): A (Agrega), E (Elimina).
3 - Texto final. Sintaxis: "[FILE_NAME] _agregado_ [EXTENSION]" (no importará el orden)
4 - Extensión de archivos a alterar. ej: .exe, .txt, .jar
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parte 1: Obtener los argumentos
    let args = env::args().collect::<Vec<String>>();

    // Parte 1.1: Ayuda en caso de no haber argumentos.s
    // En caso de que no se usen los  argumentos se mostrará ese texto como ayuda
    if args.get(1).is_none() {
        println!("Uso: <ejecutable> <directorio> <modo> <nuevo nombre> <extension>");
        return Ok(());
    }

    // Parte 2: Extraer los argumentos
    // Comprueba que este agarrando un directorio
    let directorio = PathBuf::from(args.get(1).expect("[J]: Directorio no encontrado."));
    if !directorio.is_dir() { panic!("[J]: Lo que se está pasando no es un directorio.") }
    
    let modo = args.get(2).expect("[J]: El segundo argumento (modo) no puede estar vació.");
    // Posteriormente verifico si el texto es un modo valido.

    let renombrado = args.get(3).expect("[J]: El tercer argumento (renombrado) no puede estar vació.");
    // Posteriormente verifico si cumple las condiciones dependiendo del modo.

    let extension = args.get(4).expect("[J]: El  cuarto argumento (extension) no puede estar  vació.");
    // Sirve para filtrar cierto tipo de archivos, no todos.

    // Parte 3: Ejecutar el modo adecuado.
    if modo.eq("-A") {
        renombrar_modo_agregacion(directorio, renombrado, extension)?;
    }
    else if modo.eq("-E") {
        renombrar_modo_eliminacion(directorio, renombrado, extension)?;
    } else {
        panic!("[J]: No es un modo valido.")
    }

    Ok(())
}

fn renombrar_modo_agregacion(directorio: PathBuf, renombrado: &String, extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    for archivo in directorio.read_dir()? {
        // Parte 1: Obtiene el archivo de forma segura (lo omite si no es archivo.)
        let archivo = archivo?.path();
        if !archivo.is_file() { continue; }

        //Parte 1.1: Si no tiene la extensión correcta será ignorado
        if !contiene_extension(&archivo, extension) { continue; }

        // Parte 2: Obtiene el nombre y extensión del archivo por separado.
        let file_name = archivo.file_stem().expect("[J]: No se puede extraer el nombre.").to_string_lossy().to_string();
        let file_extension  = archivo.extension().expect("[J]: No se puede extraer la extensión.").to_string_lossy().to_string();

        // Parte 2.1: Evitar que avance si no segura que tendra el nombre y extensión que venia por default.
        if !renombrado.contains("[FILE_NAME]") && !renombrado.contains("[EXTENSION]") {
            panic!("[J]: Este modo necesita el nombre y extensión original del archivo.");
        }

        // Parte 3: Crear  el nuevo nombre y extensión
        let mut nuevo_nombre = renombrado.to_owned();
        nuevo_nombre = nuevo_nombre.replace("[FILE_NAME]", &file_name);
        nuevo_nombre = nuevo_nombre.replace("[EXTENSION]", &format!(".{}", file_extension));

        // Parte 4: Crea la nueva ruta con el nombre completo hasta el archivo. Toma como base el path del directorio original
        // Y le agrega el nombre que creo el script con anterioridad
        let mut nuevo_nombre_completo = PathBuf::from(&directorio);
        nuevo_nombre_completo.push(&nuevo_nombre);
        
        // Parte 5: Renombrar el archivo final
        fs::rename(&archivo,nuevo_nombre_completo).expect(&format!("[J]: No fue posible renombar el archivo '{:?}'", &archivo));
    }

    println!("[Program]: Agregado completado.");

    Ok(())
}

fn renombrar_modo_eliminacion(directorio: PathBuf, renombrado: &String, extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    for archivo in directorio.read_dir()? {
        // Parte 1: Ignora el elemento si no es un archivo
        let archivo = archivo?.path();
        if !archivo.is_file() { continue; }

        //Parte 1.1: Si no tiene la extensión  correcta será ignorado
        if !contiene_extension(&archivo, extension) { continue; }

        // Parte 2: Obtener y crear el nuevo nombre
        let file_name = archivo.file_name().expect("[J]: No se puede extraer el nombre.").to_string_lossy().to_string();
        let nuevo_nombre = file_name.replace(renombrado, ""); //Indica que ya no habrá nada en su lugar 
        let mut nuevo_nombre_completo = PathBuf::from(&directorio);
        nuevo_nombre_completo.push(nuevo_nombre);


        // Parte 3: Renombrar el archivo
        fs::rename(archivo, nuevo_nombre_completo).expect("[J]: No se pudo eliminar el elemento.");
    }

    println!("[Program]: Eliminacion completada.");

    Ok(())
}

fn contiene_extension(archivo: &PathBuf, extension: &str) -> bool {
    archivo
        .extension()
        .expect("[J]: No se pudo extraer la extensión para la comprobación")
        .to_str()
        .unwrap()
        .eq(extension)
}