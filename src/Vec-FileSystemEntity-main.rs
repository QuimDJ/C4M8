#[derive(Debug)]
enum FileSystemEntity {
    File {
        name: String,
    },
    Folder {
        name: String,
        content: Vec<FileSystemEntity>,
    },
}

fn main() {
    // Archivos individuales
    let rust_file = FileSystemEntity::File {
        name: String::from("my_rust_code.rs"),
    };

    let python_file = FileSystemEntity::File {
        name: String::from("my_python_code.py"),
    };

    // Carpeta que contiene archivos
    let code_folder = FileSystemEntity::Folder {
        name: String::from("code_stuff"),
        content: vec![rust_file, python_file],
    };

    // Otro archivo
    let screenplay_file = FileSystemEntity::File {
        name: String::from("screenplay.txt"),
    };

    // Carpeta raíz que contiene archivos y otras carpetas
    let all_documents = FileSystemEntity::Folder {
        name: String::from("documents"),
        content: vec![screenplay_file, code_folder],
    };

    // Mostrar la estructura completa
    println!("{:#?}", all_documents);
}
