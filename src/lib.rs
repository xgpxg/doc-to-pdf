use anyhow::bail;
use libreoffice_rs::Office;
use libreoffice_rs::urls;
use std::path::Path;
use std::{env, fs};

pub fn set_libreoffice_root_dir(dir: &str) -> anyhow::Result<()> {
    unsafe {
        env::set_var("LIBREOFFICE_ROOT_DIR", dir);
    }
    Ok(())
}

/// 将docx、doc、xlsx、xls、pptx、ppt、csv格式的文件转换为pdf
/// - src: 源文件路径
/// - dest: 目标文件路径
pub fn convert(src: &str, dest: &str) -> anyhow::Result<()> {
    let ext = Path::new(src).extension();
    if ext.is_none() {
        bail!("Invalid filename");
    }
    let ext = ext.unwrap().to_str().unwrap();
    if ext != "docx"
        && ext != "doc"
        && ext != "xlsx"
        && ext != "xls"
        && ext != "pptx"
        && ext != "ppt"
        && ext != "csv"
    {
        bail!("Unsupported file format: {}", ext);
    }

    // 由于libreoffice路径似乎不支持中文，所以先复制一个临时文件
    let temp_dir = env::temp_dir().join("doc-to-pdf");
    fs::create_dir_all(&temp_dir)?;
    let temp = temp_dir.join(&format!("{}.{}", uuid::Uuid::new_v4().to_string(), ext));
    fs::copy(src, &temp)?;

    let libreoffice_root_dir = env::var("LIBREOFFICE_ROOT_DIR")?;
    let program_dir = format!("{}/program", libreoffice_root_dir);

    {
        let mut office = Office::new(&program_dir)?;

        let doc_url = urls::local_into_abs(&temp.display().to_string())?;

        let mut doc = office.document_load(doc_url)?;

        let ok = doc.save_as(dest, "pdf", None);
        if !ok {
            log::error!("save pdf file failed: {}", office.get_error());
            bail!("save pdf file failed: {}", office.get_error());
        }
    }

    fs::remove_file(temp)?;

    Ok(())
}
