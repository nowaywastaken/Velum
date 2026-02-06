pub mod piece_tree;
pub mod line_breaking;
pub mod line_layout;
pub mod ooxml;
pub mod find;
pub mod text_shaping;
pub mod page_layout;
pub mod undo_redo;

pub use piece_tree::{BufferId, Piece, PieceTree, TextAttributes};
pub use line_breaking::{BreakType, Line, LineBreaker};
pub use line_layout::{DocumentLayout, LineLayout, ParagraphLayout};
pub use ooxml::{parse_ooxml, ParsedDocument, OoxmlError};
pub use find::{SearchOptions, SearchResult, SearchResultSet};
pub use page_layout::{PageConfig, PageLayout, RenderedPage, RenderedLine, Rect, PaginationConfig};
pub use undo_redo::{
    Command, CommandError, CommandMetadata, CommandRecord,
    InsertCommand, DeleteCommand,
    UndoRedoManager, CommandExecution, OperationType,
    DEFAULT_MAX_HISTORY_SIZE, DEFAULT_MERGE_WINDOW_MS,
};

mod bridge_generated;
mod api;
pub use api::*;
