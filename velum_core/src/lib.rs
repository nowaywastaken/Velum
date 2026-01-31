pub mod piece_tree;
pub mod line_breaking;
pub mod line_layout;
pub mod ooxml;
pub mod find;

pub use piece_tree::{BufferId, Piece, PieceTree, TextAttributes};
pub use line_breaking::{BreakType, Line, LineBreaker};
pub use line_layout::{DocumentLayout, LineLayout, ParagraphLayout};
pub use ooxml::{parse_ooxml, ParsedDocument, OoxmlError};
pub use find::{SearchOptions, SearchResult, SearchResultSet};

mod bridge_generated;
mod api;
pub use api::*;
