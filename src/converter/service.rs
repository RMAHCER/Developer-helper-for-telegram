// Converter service - упрощённая версия для обработки изображений

use crate::error::{AppError, Result};
use image::ImageFormat;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Сервис конвертации файлов
#[derive(Clone)]
pub struct ConverterService {
    temp_dir: String,
    output_dir: String,
}

impl ConverterService {
    pub fn new(temp_dir: String, output_dir: String) -> Self {
        Self { temp_dir, output_dir }
    }

    /// Инициализация директорий
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.temp_dir).await?;
        fs::create_dir_all(&self.output_dir).await?;
        tracing::info!("Converter directories initialized");
        Ok(())
    }

    /// Конвертировать изображение
    pub async fn convert_image(
        &self,
        input_path: &Path,
        target_format: &str,
    ) -> Result<PathBuf> {
        let format = self.parse_image_format(target_format)?;

        // Читаем изображение
        let img = image::open(input_path).map_err(|e| {
            AppError::FileProcessing(format!("Failed to open image: {}", e))
        })?;

        // Генерируем путь для output файла
        let output_filename = format!(
            "{}.{}",
            uuid::Uuid::new_v4(),
            target_format.to_lowercase()
        );
        let output_path = Path::new(&self.output_dir).join(output_filename);

        // Сохраняем в новом формате
        img.save_with_format(&output_path, format).map_err(|e| {
            AppError::FileProcessing(format!("Failed to convert image: {}", e))
        })?;

        tracing::info!("Converted image to {:?}: {:?}", format, output_path);
        Ok(output_path)
    }

    /// Изменить размер изображения
    pub async fn resize_image(
        &self,
        input_path: &Path,
        width: u32,
        height: u32,
    ) -> Result<PathBuf> {
        let img = image::open(input_path).map_err(|e| {
            AppError::FileProcessing(format!("Failed to open image: {}", e))
        })?;

        let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

        let output_filename = format!("{}_{}x{}.png", uuid::Uuid::new_v4(), width, height);
        let output_path = Path::new(&self.output_dir).join(output_filename);

        resized.save(&output_path).map_err(|e| {
            AppError::FileProcessing(format!("Failed to save resized image: {}", e))
        })?;

        tracing::info!("Resized image to {}x{}: {:?}", width, height, output_path);
        Ok(output_path)
    }

    /// Парсинг формата изображения
    fn parse_image_format(&self, format: &str) -> Result<ImageFormat> {
        match format.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "gif" => Ok(ImageFormat::Gif),
            "webp" => Ok(ImageFormat::WebP),
            "bmp" => Ok(ImageFormat::Bmp),
            _ => Err(AppError::Validation(format!(
                "Unsupported image format: {}",
                format
            ))),
        }
    }

    /// Очистка старых файлов
    pub async fn cleanup_old_files(&self, max_age_hours: u64) -> Result<()> {
        // TODO: Implement cleanup logic
        tracing::debug!("Cleanup old files (max age: {} hours)", max_age_hours);
        Ok(())
    }
}
