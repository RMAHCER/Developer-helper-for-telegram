// Image processor - обработка изображений

use crate::error::{AppError, Result};
use image::DynamicImage;
use std::path::Path;

/// Загрузить изображение
pub fn load_image(path: &Path) -> Result<DynamicImage> {
    image::open(path).map_err(|e| AppError::FileProcessing(format!("Failed to load image: {}", e)))
}

/// Применить фильтр grayscale
pub fn grayscale(img: &DynamicImage) -> DynamicImage {
    img.grayscale()
}

/// Применить blur
pub fn blur(img: &DynamicImage, sigma: f32) -> DynamicImage {
    img.blur(sigma)
}

/// Повернуть изображение на 90 градусов
pub fn rotate90(img: &DynamicImage) -> DynamicImage {
    img.rotate90()
}

/// Отразить по горизонтали
pub fn flip_horizontal(img: &DynamicImage) -> DynamicImage {
    img.fliph()
}
