use super::types::LiveProjectState;
use crate::mcp::common::indexer::IndexState;
use anyhow::Result;

impl LiveProjectState {
    /// Perform initial scan of the project
    pub async fn initial_scan(&self) -> Result<()> {
        println!("🔍 Performing initial project scan...");

        // Marquer l'index comme SCANNING
        {
            let mut index_state = self.index_state.write().await;
            *index_state = IndexState::Scanning;
        }

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = true;
        drop(is_scanning);

        let mut scan_success = true;

        // Scan code
        match crate::mcp::common::indexer::scan_project(&self.project_path).await {
            Ok(index) => {
                let mut code_index = self.code_index.write().await;
                *code_index = index;
                println!("✅ Indexed {} files", code_index.files.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan code: {}", e);
                scan_success = false;
            }
        }

        // Scan routes
        match crate::mcp::common::routes::scan_routes(&self.project_path).await {
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

        // CRITIQUE P0 : Marquer l'index comme READY ou FAILED
        {
            let mut index_state = self.index_state.write().await;
            *index_state = if scan_success {
                IndexState::Ready
            } else {
                IndexState::Failed
            };
            println!("📊 Index state: {}", *index_state);
        }

        Ok(())
    }

    /// Perform initial scan without stdout logs (for stdio mode)
    pub async fn initial_scan_quiet(&self) -> Result<()> {
        eprintln!("🔍 Performing initial project scan...");

        // Marquer l'index comme SCANNING
        {
            let mut index_state = self.index_state.write().await;
            *index_state = IndexState::Scanning;
        }

        let mut is_scanning = self.is_scanning.write().await;
        *is_scanning = true;
        drop(is_scanning);

        let mut scan_success = true;

        // Scan code
        match crate::mcp::common::indexer::scan_project(&self.project_path).await {
            Ok(index) => {
                let mut code_index = self.code_index.write().await;
                *code_index = index;
                eprintln!("✅ Indexed {} files", code_index.files.len());
            }
            Err(e) => {
                eprintln!("⚠️  Failed to scan code: {}", e);
                scan_success = false;
            }
        }

        // Scan routes
        match crate::mcp::common::routes::scan_routes(&self.project_path).await {
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

        // CRITIQUE P0 : Marquer l'index comme READY ou FAILED
        {
            let mut index_state = self.index_state.write().await;
            *index_state = if scan_success {
                IndexState::Ready
            } else {
                IndexState::Failed
            };
            eprintln!("📊 Index state: {}", *index_state);
        }

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

        // Invalider l'index pendant le rescan
        {
            let mut index_state = self.index_state.write().await;
            *index_state = IndexState::Scanning;
        }

        println!("🔄 Rescanning project...");
        self.initial_scan().await
    }
}
