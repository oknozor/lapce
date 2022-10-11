use druid::WidgetId;
use lsp_types::CodeLens;

#[derive(Clone)]
pub struct CodeLenseData {
    pub id: WidgetId,
    request_id: usize, 
    pub lenses: Vec<CodeLens>
}

impl CodeLenseData {
    pub fn new() -> Self {
        Self {
            id: WidgetId::next(),
            lenses: vec![],
            request_id: 0,
        }
    }
}

