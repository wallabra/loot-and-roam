//! Terrain chunked cache.
//!
//! Used to avoid redundant computation of the terrain height function.
//!
//! Also pre-computes other fields, such as the slopes and surface normals,
//! for various purposes, such as objects sliding on terrain, and rendering
//! effects.
