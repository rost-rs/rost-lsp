mod docs;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use utilrs::models::rost::Template;

/// In-memory document storage and LSP service backend.
#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, String>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Performs real-time syntax checking and generates LSP diagnostics.
    async fn validate(&self, uri: &Url, text: &str) {
        let mut diagnostics = Vec::new();

        // 0. High-performance template indexing check
        let template = Template::from_str(text);
        let _parsed_fields = template.read_fields();

        // 1. Balance and syntax validation line-by-line
        let mut brace_depth = 0;
        let mut bracket_depth = 0;

        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();

            // Skip comments
            if trimmed.starts_with("//") {
                continue;
            }

            // Track block braces
            for (char_idx, ch) in line.chars().enumerate() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        if brace_depth == 0 {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position::new(line_idx as u32, char_idx as u32),
                                    end: Position::new(line_idx as u32, (char_idx + 1) as u32),
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String("E001".into())),
                                message: "Unmatched closing brace '}'".into(),
                                source: Some("rost-lsp".into()),
                                ..Default::default()
                            });
                        } else {
                            brace_depth -= 1;
                        }
                    }
                    '[' => bracket_depth += 1,
                    ']' => {
                        if bracket_depth == 0 {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position::new(line_idx as u32, char_idx as u32),
                                    end: Position::new(line_idx as u32, (char_idx + 1) as u32),
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String("E002".into())),
                                message: "Unmatched closing bracket ']'".into(),
                                source: Some("rost-lsp".into()),
                                ..Default::default()
                            });
                        } else {
                            bracket_depth -= 1;
                        }
                    }
                    _ => {}
                }
            }

            // Check unclosed quote on line (if not escaped)
            let quote_count = line.matches('"').count();
            if quote_count % 2 != 0 {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position::new(line_idx as u32, 0),
                        end: Position::new(line_idx as u32, line.len() as u32),
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("E003".into())),
                    message: "Unclosed quotation mark '\"' on line".into(),
                    source: Some("rost-lsp".into()),
                    ..Default::default()
                });
            }

            // Validate property definitions
            if let Some((key, val)) = trimmed.split_once(':') {
                let key = key.trim();
                let val = val.trim().trim_end_matches([',', ';']);

                if !key.is_empty() && !key.starts_with('@') && !key.starts_with('#') {
                    // Check hex colors
                    if let Some(hex) = val.strip_prefix('#') {
                        if hex.len() != 3 && hex.len() != 6 && hex.len() != 8 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position::new(line_idx as u32, 0),
                                    end: Position::new(line_idx as u32, line.len() as u32),
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                code: Some(NumberOrString::String("W001".into())),
                                message: format!("Malformed hex color '#{}': expected 3, 6, or 8 hex digits (e.g., #fff, #1e1e2e, #1e1e2eff)", hex),
                                source: Some("rost-lsp".into()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        // Check unclosed block at end of file
        if brace_depth > 0 {
            let last_line = text.lines().count().saturating_sub(1) as u32;
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(last_line, 0),
                    end: Position::new(last_line, 1),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("E004".into())),
                message: format!("Unclosed block: missing {} closing brace(s) '}}'", brace_depth),
                source: Some("rost-lsp".into()),
                ..Default::default()
            });
        }

        // Publish to editor
        self.client.publish_diagnostics(uri.clone(), diagnostics, None).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "rost-lsp".into(),
                version: Some("0.1.0".into()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["@".into(), "#".into(), ":".into(), "|".into()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ROST Language Server initialized successfully!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents.write().await.insert(uri.clone(), text.clone());
        self.validate(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;
            self.documents.write().await.insert(uri.clone(), text.clone());
            self.validate(&uri, &text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let text = match docs.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let current_line = text.lines().nth(position.line as usize).unwrap_or("");
        let prefix = if (position.character as usize) <= current_line.len() {
            &current_line[..position.character as usize]
        } else {
            current_line
        };

        let mut items = Vec::new();

        // 1. Directives completions
        if prefix.trim_start().starts_with('@') {
            for entry in docs::DIRECTIVES {
                items.push(CompletionItem {
                    label: entry.symbol.into(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some(format!("{} - {}", entry.category, entry.summary)),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: entry.detail_md.into(),
                    })),
                    insert_text: Some(entry.snippet.into()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                });
            }
            return Ok(Some(CompletionResponse::Array(items)));
        }

        // 2. Selectors & Component block snippets
        if prefix.trim_start().starts_with('#') {
            for entry in docs::SELECTORS {
                items.push(CompletionItem {
                    label: entry.symbol.into(),
                    kind: Some(CompletionItemKind::SNIPPET),
                    detail: Some(format!("{} - {}", entry.category, entry.summary)),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: entry.detail_md.into(),
                    })),
                    insert_text: Some(entry.snippet.into()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                });
            }
            return Ok(Some(CompletionResponse::Array(items)));
        }

        // 3. States completion inside pipe delimiters
        if prefix.trim_start().starts_with('|') || prefix.contains('|') {
            for entry in docs::STATES {
                items.push(CompletionItem {
                    label: entry.symbol.into(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(format!("{} - {}", entry.category, entry.summary)),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: entry.detail_md.into(),
                    })),
                    insert_text: Some(entry.snippet.into()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                });
            }
        }

        // 4. Properties and styling attributes
        for entry in docs::PROPERTIES {
            items.push(CompletionItem {
                label: format!("{}:", entry.symbol),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(format!("{} - {}", entry.category, entry.summary)),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: entry.detail_md.into(),
                })),
                insert_text: Some(entry.snippet.into()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        // 5. Units & common values if after colon
        if prefix.ends_with(':') || prefix.ends_with(": ") {
            for entry in docs::UNITS {
                items.push(CompletionItem {
                    label: entry.symbol.into(),
                    kind: Some(CompletionItemKind::UNIT),
                    detail: Some(format!("{} - {}", entry.category, entry.summary)),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: entry.detail_md.into(),
                    })),
                    insert_text: Some(format!("10{},", entry.symbol)),
                    ..Default::default()
                });
            }

            let common_values = [
                ("true", "Boolean true flag"),
                ("false", "Boolean false flag"),
                ("flex", "Flexbox container layout"),
                ("grid", "Grid container layout"),
                ("block", "Standard block flow layout"),
                ("none", "Do not render element"),
                ("center", "Centered layout alignment"),
                ("start", "Align to start boundary"),
                ("end", "Align to end boundary"),
                ("space_between", "Distribute space between items"),
                ("hidden", "Clip overflowing content"),
                ("visible", "Show overflowing content"),
                ("pointer", "Interactive pointing cursor"),
            ];
            for (val, desc) in common_values {
                items.push(CompletionItem {
                    label: val.into(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some(desc.into()),
                    insert_text: Some(format!("{},", val)),
                    ..Default::default()
                });
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let text = match docs.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let current_line = text.lines().nth(position.line as usize).unwrap_or("");
        let col = position.character as usize;

        // Find word under cursor
        let (start, end) = {
            let chars: Vec<char> = current_line.chars().collect();
            if col >= chars.len() {
                return Ok(None);
            }
            let mut s = col;
            while s > 0 && (chars[s - 1].is_alphanumeric() || chars[s - 1] == '_' || chars[s - 1] == '@' || chars[s - 1] == '#') {
                s -= 1;
            }
            let mut e = col;
            while e < chars.len() && (chars[e].is_alphanumeric() || chars[e] == '_' || chars[e] == '.' || chars[e] == '%') {
                e += 1;
            }
            (s, e)
        };

        let word = &current_line[start..end];

        // 1. Dynamic hex color hover with direct RGB info
        if word.starts_with('#') && word.len() > 1 && word[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            let hex_clean = &word[1..];
            let rgb_info = match hex_clean.len() {
                3 => {
                    let r = u8::from_str_radix(&hex_clean[0..1].repeat(2), 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex_clean[1..2].repeat(2), 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex_clean[2..3].repeat(2), 16).unwrap_or(0);
                    format!("**Hex Color Preview:** `{}`\n\n- **RGB:** `rgb({}, {}, {})`\n- **Direct GPU buffer mapping:** zero-cost integer parse", word, r, g, b)
                }
                6 => {
                    let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);
                    format!("**Hex Color Preview:** `{}`\n\n- **RGB:** `rgb({}, {}, {})`\n- **Direct GPU buffer mapping:** zero-cost integer parse", word, r, g, b)
                }
                8 => {
                    let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);
                    let a = u8::from_str_radix(&hex_clean[6..8], 16).unwrap_or(255);
                    let alpha_f = (a as f32) / 255.0;
                    format!("**Hex Color Preview:** `{}`\n\n- **RGBA:** `rgba({}, {}, {}, {:.2})`\n- **Alpha:** `{}%`", word, r, g, b, alpha_f, (alpha_f * 100.0) as u32)
                }
                _ => format!("**Hex Color:** `{}`", word),
            };

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: rgb_info,
                }),
                range: None,
            }));
        }

        // 2. Comprehensive documentation registry lookup
        if let Some(doc_md) = docs::lookup_docs(word) {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc_md.into(),
                }),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let docs = self.documents.read().await;
        let text = match docs.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        // Format document with clean 4-space indentation
        let mut formatted = String::with_capacity(text.len());
        let mut indent_level: usize = 0;

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }

            if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
            }

            for _ in 0..indent_level {
                formatted.push_str("    ");
            }
            formatted.push_str(trimmed);
            formatted.push('\n');

            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }

        let total_lines = text.lines().count() as u32;

        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(total_lines + 1, 0),
            },
            new_text: formatted,
        }]))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
