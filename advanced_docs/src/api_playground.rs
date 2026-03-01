//! API Playground
//! 
//! Provides an interactive API playground for testing and exploring
//! the Vantis Media Player API with live code execution.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// API playground
pub struct APIPlayground {
    /// API playground directory
    playground_dir: PathBuf,
    
    /// API endpoints
    endpoints: HashMap<String, APIEndpoint>,
    
    /// Example requests
    examples: HashMap<String, APIExample>,
}

/// API endpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APIEndpoint {
    /// Endpoint ID
    pub id: String,
    
    /// Endpoint path
    pub path: String,
    
    /// HTTP method
    pub method: HTTPMethod,
    
    /// Description
    pub description: String,
    
    /// Parameters
    pub parameters: Vec<APIParameter>,
    
    /// Request body schema (optional)
    pub request_schema: Option<serde_json::Value>,
    
    /// Response schema
    pub response_schema: serde_json::Value,
    
    /// Example response
    pub example_response: serde_json::Value,
    
    /// Authentication required
    pub auth_required: bool,
}

/// HTTP method
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "uppercase")]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// API parameter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APIParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter location
    pub location: ParameterLocation,
    
    /// Parameter type
    pub param_type: String,
    
    /// Required
    pub required: bool,
    
    /// Description
    pub description: String,
    
    /// Default value (optional)
    pub default_value: Option<serde_json::Value>,
}

/// Parameter location
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Query,
    Path,
    Header,
    Body,
}

/// API example
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APIExample {
    /// Example ID
    pub id: String,
    
    /// Example name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Endpoint ID
    pub endpoint_id: String,
    
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Request body (optional)
    pub request_body: Option<serde_json::Value>,
}

impl APIPlayground {
    /// Create a new API playground
    pub fn new(playground_dir: PathBuf) -> Result<Self> {
        info!("🎮 Initializing API Playground");
        
        // Create playground directory if it doesn't exist
        std::fs::create_dir_all(&playground_dir)?;
        
        // Load API endpoints
        let endpoints = Self::load_endpoints(&playground_dir)?;
        
        // Load examples
        let examples = Self::load_examples(&playground_dir)?;
        
        info!("✅ API playground initialized");
        info!("   - Playground directory: {}", playground_dir.display());
        info!("   - Loaded {} endpoints", endpoints.len());
        info!("   - Loaded {} examples", examples.len());
        
        Ok(Self {
            playground_dir,
            endpoints,
            examples,
        })
    }
    
    /// Load API endpoints from directory
    fn load_endpoints(dir: &Path) -> Result<HashMap<String, APIEndpoint>> {
        let mut endpoints = HashMap::new();
        
        let endpoints_file = dir.join("endpoints.json");
        if endpoints_file.exists() {
            let content = std::fs::read_to_string(&endpoints_file)?;
            let endpoint_list: Vec<APIEndpoint> = serde_json::from_str(&content)?;
            for endpoint in endpoint_list {
                endpoints.insert(endpoint.id.clone(), endpoint);
            }
        }
        
        Ok(endpoints)
    }
    
    /// Load examples from directory
    fn load_examples(dir: &Path) -> Result<HashMap<String, APIExample>> {
        let mut examples = HashMap::new();
        
        let examples_file = dir.join("examples.json");
        if examples_file.exists() {
            let content = std::fs::read_to_string(&examples_file)?;
            let example_list: Vec<APIExample> = serde_json::from_str(&content)?;
            for example in example_list {
                examples.insert(example.id.clone(), example);
            }
        }
        
        Ok(examples)
    }
    
    /// Generate API playground
    pub async fn generate(&self) -> Result<()> {
        info!("🎮 Generating API playground");
        
        // Generate main playground page
        self.generate_playground_page()?;
        
        // Generate endpoint pages
        for (id, endpoint) in &self.endpoints {
            self.generate_endpoint_page(endpoint)?;
            debug!("✅ Generated endpoint page: {}", id);
        }
        
        info!("✅ API playground generated successfully");
        
        Ok(())
    }
    
    /// Generate main playground page
    fn generate_playground_page(&self) -> Result<()> {
        let output_path = self.playground_dir.join("index.html");
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>API Playground - Vantis Media Player</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/api-playground.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;api-playground&quot;>\n");
        html.push_str("    <h1>API Playground</h1>\n");
        html.push_str("    <p>Interactive API testing and exploration</p>\n");
        
        // Endpoint list
        html.push_str("    <div class=&quot;endpoints-list&quot;>\n");
        html.push_str("      <h2>Endpoints</h2>\n");
        
        // Group by method
        let mut by_method: HashMap<HTTPMethod, Vec<&APIEndpoint>> = HashMap::new();
        for endpoint in self.endpoints.values() {
            by_method.entry(endpoint.method.clone()).or_default().push(endpoint);
        }
        
        for (method, endpoints) in by_method {
            html.push_str("      <div class=&quot;method-group&quot;>\n");
            html.push_str("        <h3>");
            html.push_str(&format!("{:?}", method));
            html.push_str("</h3>\n");
            
            for endpoint in endpoints {
                html.push_str("        <div class=&quot;endpoint-item&quot;>\n");
                html.push_str("          <a href=&quot;");
                html.push_str(&endpoint.id);
                html.push_str(".html&quot;>\n");
                html.push_str("            <span class=&quot;path&quot;>");
                html.push_str(&endpoint.path);
                html.push_str("</span>\n");
                html.push_str("            <span class=&quot;description&quot;>");
                html.push_str(&endpoint.description);
                html.push_str("</span>\n");
                html.push_str("          </a>\n");
                html.push_str("        </div>\n");
            }
            
            html.push_str("      </div>\n");
        }
        
        html.push_str("    </div>\n");
        
        // Examples section
        if !self.examples.is_empty() {
            html.push_str("    <div class=&quot;examples-section&quot;>\n");
            html.push_str("      <h2>Examples</h2>\n");
            
            for example in self.examples.values() {
                html.push_str("      <div class=&quot;example-item&quot;>\n");
                html.push_str("        <h3>");
                html.push_str(&example.name);
                html.push_str("</h3>\n");
                html.push_str("        <p>");
                html.push_str(&example.description);
                html.push_str("</p>\n");
                html.push_str("        <button class=&quot;try-example&quot; data-example-id=&quot;");
                html.push_str(&example.id);
                html.push_str("&quot;>Try Example</button>\n");
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/api-playground.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Generate endpoint page
    fn generate_endpoint_page(&self, endpoint: &APIEndpoint) -> Result<()> {
        let output_path = self.playground_dir.join(format!("{}.html", endpoint.id));
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&endpoint.path);
        html.push_str(" - API Playground</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/api-playground.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;endpoint-page&quot;>\n");
        html.push_str("    <a href=&quot;index.html&quot; class=&quot;back-link&quot;>← Back to Playground</a>\n");
        html.push_str("    <h1>");
        html.push_str(&format!("{:?} {}", endpoint.method, endpoint.path));
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&endpoint.description);
        html.push_str("</p>\n");
        
        // Parameters section
        if !endpoint.parameters.is_empty() {
            html.push_str("    <div class=&quot;parameters-section&quot;>\n");
            html.push_str("      <h2>Parameters</h2>\n");
            
            for param in &endpoint.parameters {
                html.push_str("      <div class=&quot;parameter&quot;>\n");
                html.push_str("        <label for=&quot;param-");
                html.push_str(&param.name);
                html.push_str("&quot;>");
                html.push_str(&param.name);
                if param.required {
                    html.push_str(" <span class=&quot;required&quot;>*</span>");
                }
                html.push_str("</label>\n");
                html.push_str("        <input type=&quot;text&quot; id=&quot;param-");
                html.push_str(&param.name);
                html.push_str("&quot; name=&quot;");
                html.push_str(&param.name);
                html.push_str("&quot; placeholder=&quot;");
                html.push_str(&param.param_type);
                html.push_str("&quot;");
                if let Some(default) = &param.default_value {
                    html.push_str(" value=&quot;");
                    html.push_str(&default.to_string());
                    html.push_str("&quot;");
                }
                html.push_str(">\n");
                html.push_str("        <p class=&quot;description&quot;>");
                html.push_str(&param.description);
                html.push_str("</p>\n");
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        // Request body section
        if let Some(schema) = &endpoint.request_schema {
            html.push_str("    <div class=&quot;request-body-section&quot;>\n");
            html.push_str("      <h2>Request Body</h2>\n");
            html.push_str("      <textarea id=&quot;request-body&quot; rows=&quot;10&quot; placeholder=&quot;Enter JSON request body&quot;>\n");
            html.push_str(&serde_json::to_string_pretty(schema)?);
            html.push_str("\n      </textarea>\n");
            html.push_str("    </div>\n");
        }
        
        // Send request button
        html.push_str("    <button id=&quot;send-request&quot; class=&quot;send-button&quot;>Send Request</button>\n");
        
        // Response section
        html.push_str("    <div class=&quot;response-section&quot;>\n");
        html.push_str("      <h2>Response</h2>\n");
        html.push_str("      <div id=&quot;response-status&quot; class=&quot;status&quot;></div>\n");
        html.push_str("      <pre id=&quot;response-body&quot; class=&quot;response-body&quot;></pre>\n");
        html.push_str("    </div>\n");
        
        // Example response
        html.push_str("    <div class=&quot;example-response-section&quot;>\n");
        html.push_str("      <h2>Example Response</h2>\n");
        html.push_str("      <pre class=&quot;example-response&quot;>\n");
        html.push_str(&serde_json::to_string_pretty(&endpoint.example_response)?);
        html.push_str("\n      </pre>\n");
        html.push_str("    </div>\n");
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/api-playground.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Get endpoint by ID
    pub fn get_endpoint(&self, id: &str) -> Option<&APIEndpoint> {
        self.endpoints.get(id)
    }
    
    /// Get all endpoints
    pub fn get_all_endpoints(&self) -> Vec<&APIEndpoint> {
        self.endpoints.values().collect()
    }
    
    /// Get example by ID
    pub fn get_example(&self, id: &str) -> Option<&APIExample> {
        self.examples.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_playground_creation() {
        let dir = PathBuf::from("./api_playground");
        let playground = APIPlayground::new(dir);
        assert!(playground.is_ok());
    }
}