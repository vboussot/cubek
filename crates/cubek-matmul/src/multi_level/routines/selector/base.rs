use crate::multi_level::components::tile::TileMatmulKind;

/// Strategy args that carry a [TileMatmul] kind, so convolution / other crates can
/// construct the strategy with the right tile matmul variant without hardcoding the field name.
pub trait TilingArgs {
    fn set_tile_matmul(&mut self, kind: TileMatmulKind);
    /// Ask for `row_count` rows of planes and `partition_k` tiles along k per partition where the
    /// routine's selector takes them; a no-op elsewhere.
    fn set_plane_tiling(&mut self, _row_count: u32, _partition_k: u32) {}
}
