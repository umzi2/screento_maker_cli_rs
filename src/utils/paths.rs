use std::fs;
use std::path::{Path, PathBuf};
use mime_guess::MimeGuess;

fn is_image(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(mime) = MimeGuess::from_ext(&extension.to_string_lossy()).first() {
            // Проверяем, является ли MIME тип изображением
            return mime.type_() == mime_guess::mime::IMAGE;
        }
    }
    false
}
pub fn path2vec_img(path: &Path) -> Vec<PathBuf> {
    let mut image_paths = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                // Проверяем, является ли файл изображением
                if is_image(&path) {
                    // Добавляем только имя файла в вектор
                    if let Some(file_name) = path.file_name() {
                        image_paths.push(PathBuf::from(file_name));
                    }
                }
            }
        }
    }
    image_paths
}