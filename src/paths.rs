use std::collections::HashMap;

use crate::client::{Client, PATHS_PREFIX, CREATE_DIRECTORY, CREATE_FILE, DELETE, EXISTS, FREE, GET_STATUS, MOUNT, OPEN_FILE, RENAME, SET_ATTRIBUTE, UNMOUNT, LIST_STATUS};
use crate::option::{self};
use crate::error::Result;
use crate::wire::{FileInfo, FileInfos};

impl Client {
    pub async fn create_directory(&self, path: &str, options: option::CreateDirectory) -> Result<()> {
        let _ = self.post(vec![PATHS_PREFIX, path, CREATE_DIRECTORY].join("/"), HashMap::new(), options).await?;
        Ok(())
    }

    pub async fn create_file(&self, path: &str, options: option::CreateFile) -> Result<i64> {
        let result = self.post(vec![PATHS_PREFIX, path, CREATE_FILE].join("/"), HashMap::new(), options).await?;
        Ok(result)
    }

    pub async fn delete(&self, path: &str, options: option::Delete) -> Result<()> {
        let _ = self.post(vec![PATHS_PREFIX, path, DELETE].join("/"), HashMap::new(), options).await?;
        Ok(())
    }

    pub async fn exists(&self, path: &str,  options: option::Exists) -> Result<bool> {
        let result = self.post(vec![PATHS_PREFIX, path, EXISTS].join("/"), HashMap::new(), options).await?;
        Ok(result)
    }

    pub async fn free(&self, path: &str, options: option::Free) -> Result<()> {
        let _ = self.post(vec![PATHS_PREFIX, path, FREE].join("/"), HashMap::new(), options).await?;
        Ok(())
    }

    pub async fn get_status(&self, path: &str, options: option::GetStatus) -> Result<FileInfo> {
        let result: FileInfo = self.post(vec![PATHS_PREFIX, path, GET_STATUS].join("/"), HashMap::new(), options).await?;
        Ok(result)
    }

    pub async fn list_status(&self, path: &str, options: option::ListStatus) -> Result<FileInfos> {
        let result: FileInfos = self.post(vec![PATHS_PREFIX, path, LIST_STATUS].join("/"), HashMap::new(), options).await?;
        Ok(result)
    }

    pub async fn mount(&self, path: &str, src: &str, options: option::Mount) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("src", src);
        let _ = self.post(vec![PATHS_PREFIX, path, MOUNT].join("/"), params, options).await?;
        Ok(())
    }

    pub async fn open_file(&self, path: &str, options: option::OpenFile) -> Result<i64> {
        let result = self.post(vec![PATHS_PREFIX, path, OPEN_FILE].join("/"), HashMap::new(), options).await?;
        Ok(result)
    }

    pub async fn rename(&self, path: &str, dst: &str, options: option::Rename) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("dst", dst);
        let _ = self.post(vec![PATHS_PREFIX, path, RENAME].join("/"), params, options).await?;
        Ok(())
    }

    pub async fn set_attribute(&self, path: &str, options: option::SetAttribute) -> Result<()> {
        let _ = self.post(vec![PATHS_PREFIX, path, SET_ATTRIBUTE].join("/"), HashMap::new(), options).await?;
        Ok(())
    }

    pub async fn unmount(&self, path: &str, options: option::Unmount) -> Result<()> {
        let _ = self.post(vec![PATHS_PREFIX, path, UNMOUNT].join("/"), HashMap::new(), options).await?;
        Ok(())
    }
}