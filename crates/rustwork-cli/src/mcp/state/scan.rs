use super::types::LiveProjectState;
use anyhow::Result;

impl LiveProjectState {
    /// Perform initial scan of the project
    pub async fn initial_scan(&self) -> Result<()> {
        println!("🔍 Performing initial project scan...");

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = true;
        drop(is_scanning);

        // Scan code
        match crate::mcp::indexer::scan_project(&self.project_path).await {
            Ok(index) => {
                let mut code_index = self.code_index.write().await;
                *code_index = index;
                println!("✅ Indexed {} files", code_index.files.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan code: {}", e);
            }
        }

        // Scan routes
        match crate::mcp::routes::scan_routes(&self.project_path).await {
            Ok(registry) => {
                let mut routes = self.routes.write().await;
                *routes = registry;
                println!("✅ Found {} routes", routes.routes.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan routes: {}", e);
            }
        }

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = false;

        Ok(())
    }

    /// Perform initial scan without stdout logs (for stdio mode)
    pub async fn initial_scan_quiet(&self) -> Result<()> {
        eprintln!("🔍 Performing initial project scan...");

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = true;
        drop(is_scanning);

        // Scan code
        match crate::mcp::indexer::scan_project(&self.project_path).await {
            Ok(index) => {
                let mut code_index = self.code_index.write().await;
                *code_index = index;
                eprintln!("✅ Indexed {} files", code_index.files.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan code: {}", e);
            }
        }

        // Scan routes
        match crate::mcp::routes::scan_routes(&self.project_path).await {
            Ok(registry) => {
                let mut routes = self.routes.write().await;
                *routes = registry;
                eprintln!("✅ Found {} routes", routes.routes.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan routes: {}", e);
            }
        }

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = false;

        Ok(())
    }

    /// Rescan the entire project (called on file changes)
    pub async fn rescan(&self) -> Result<()> {
        // Check if already scanning
        {
            let is_scanning = self.is_scanning.read().await;
            if *is_scanning {
                return Ok(());
            }
        }

        println!("🔄 Rescanning project...");
        self.initial_scan().await
    }
}
