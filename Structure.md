# Rendering & Implementation Architecture

## Overview

The map is structured in **4 layers** with different rendering and implementation orders. Implementation follows a phased approach per layer: **solid color blocks** → **styling (textures, outlines)** → **FX and shader effects** → **dynamic/animated elements**.

---

## Layer System

| Rendering Order | Layer Name | Implementation Order | Status | Primary Components |
|:---:|:---|:---:|:---|:---|
| 1 | **Paper** | Last | Todo | Background texture (aged parchment) |
| 2 | **Map** | 2nd | Todo | Water, land, lakes, rivers, mountains |
| 3 | **Overlays** | 1st | In Progress | Graticule, scales, notes, titles, diagrams |

---

## Detailed Layer Specifications

### Layer 1: Paper 

**Purpose**: Background foundation for the entire composition.

**Current Phase**: Placeholder only (ignore until Phase 3)

| Phase | Details |
|:---|:---|
| **Solid Color Block** | Uniform background color (cream/aged parchment tone) |
| **Styling** | Texture mapping (paper grain, age marks, stains) |
| **FX & Shaders** | Ink bleed simulation, shadow gradients, vignetting |
| **Dynamic** | Time-based weathering effects (future) |

**Deferred to Phase 3+**

---

### Layer 2: Map 

**Purpose**: Core geographical data for all mapped regions (world map + poles for Kunyu Wanguo Quantu).

**Composition**: Multiple map projections
- Main world map: Kavrayskiy VII projection
- North Pole: Azimuthal projection (perspective sphere)
- South Pole: Azimuthal projection (perspective sphere)

**Rendering Order Within Layer**:
1. Water (background fill)
2. Land rings / landmass polygons
3. Lakes
4. Rivers
5. Mountains / elevation symbols

| Phase | Details | Notes |
|:---|:---|:---|
| **Solid Color Block** | Simple colored geometry for each map section | Water = light blue, Land = tan, verify projection math works |
| **Styling** | Land outlines, coastline refinement, color gradients | Hand-drawn coastline effects, shading |
| **FX & Shaders** | Elevation-based coloring, relief shading, texture overlay | Mountain symbols procedural generation |
| **Dynamic** | N/A for Phase 1 (static maps) | Reserved for climate/weather overlays in Phase 3+ |

**Key Implementation Steps**:
- [ ] Load Natural Earth coastline/land data
- [x] Implement Kavrayskiy VII projection (main map)
- [x] Implement Azimuthal projection (poles - perspective sphere)
- [ ] Render water layer (Bevy ColorMaterial)
- [ ] Render land layer with outlines
- [ ] Add lake/river geometry
- [ ] Integrate elevation data for mountain rendering

---

### Layer 3: Overlays 

**Purpose**: Informational and decorative elements overlaid on maps; includes text, measurements, annotations.

**Static Components** (Phase 1):
- Graticule (latitude/longitude grid) on map regions
- Scale borders with degree markers
- Graticule scale notes (cardinal directions, degree labels)
- Descriptive notes and annotations
- Titles (main title, regional titles)
- Orbit explanation diagrams (static SVG-style)
- Decorative elements (compass rose, celestial diagrams)

**Animated Components** (Phase 2+, deferred):
- Astronomy Sphere rotation
- Celestial mechanics animation
- Day/night cycle visualization

| Phase | Details | Notes |
|:---|:---|:---|
| **Solid Color Block** | Text and line geometry without styling | Basic graticule lines, text outlines, diagram shapes |
| **Styling** | Font rendering, line weights, colors, alignment | Chinese typography (vertical/horizontal), calligraphy style |
| **FX & Shaders** | Text shadow/glow, decorative patterns, ink effects | Aging effects on annotations |
| **Dynamic** | Animated elements (Astronomy Sphere, celestial paths) | Phase 2+ only |

**Key Implementation Steps**:
- [ ] Define graticule system (lat/lon grid per projection)
- [ ] Implement scale borders with tick marks
- [ ] Add text labels (Bevy Text2d)
- [ ] Implement Chinese text rendering (font loading, orientation)
- [ ] Placements rules for notes (land vs. ocean)
- [ ] Create orbit/celestial diagram templates
- [ ] (Phase 2) Implement animated astronomy sphere

---



## Implementation Phases

### Phase 1: Static Foundation (Current)

**Goal**: Establish all three layers with core functionality and basic rendering; verify rendering pipeline and visual hierarchy.

**Order of Implementation**:
1. **Overlays Layer** (Graticule rendering)
   - Kavrayskiy VII graticule for main world map (Meridians + Parallels)
   - Azimuthal polar graticules (North & South poles with perspective sphere)
   - Border frame around canvas
   - ✓ Rendered via custom shader materials (KavrayskiyViiGraticuleMaterial)

2. **Map Layer** (Deferred)
   - Coastline projection (Kavrayskiy VII + Azimuthal) with natural earth data
   - Water/land geometry rendering
   - Color differentiation

3. **Paper Layer** (Placeholder)
   - Parchment background color

**Output**: Interactive Bevy window with graticule overlays for Kavrayskiy VII and polar projections, pan/zoom controls, ready for Phase 2 map geometry.

---

### Phase 2: Styling & Refinement

**Goal**: Apply visual polish to all layers; hand-drawn effects, typography, decorative details.

**Tasks**:
- Hand-drawn coastline simulation (Map)
- Chinese typography and calligraphy styling (Overlays)
- Ornamental borders and embellishment (Outlines)
- Animated overlays framework (deferred details)

---

### Phase 3: Effects & Paper Texture

**Goal**: Add shader effects, atmospheric elements, and paper texture.

**Tasks**:
- Paper texture mapping and aging (Paper)
- Relief shading and elevation effects (Map)
- Ink effects and glow (Overlays)
- Vignetting and post-processing (Paper)

---

### Phase 4: Dynamic & Advanced Features

**Goal**: Implement animated elements and extended features.

**Tasks**:
- Astronomy Sphere animation (Overlays)
- Celestial mechanics visualization
- Interactive pan/zoom
- Multi-style support (future maps beyond Kunyu Wanguo Quantu)

---

## Key Decisions

- **Implementation vs. Rendering Order**: Deliberately different to ensure content visibility and layout stability before visual refinement.
- **Kunyu Wanguo Quantu Focus**: Phase 1 targets this specific map; architecture should support future maps (European, Islamic) but not require generic system overhead now.
- **Static Overlays First**: Animated elements (Astronomy Sphere) deferred to Phase 2 to stabilize rendering foundation.
- **Declarative Outlines**: Layout borders are static and rule-based, not procedurally generated, allowing deterministic layout planning.
- **Iterative Per-Layer**: Each layer completes solid → styling → FX progression before moving to next phase, enabling parallel artist/shader work.

---

## Notes & Future Adjustments

- Adjust rendering/implementation order as progress reveals dependencies.
- Add specific Bevy component/system names during Phase 1 implementation.
- Track shader and texture asset creation in parallel with geometry implementation.
- Document discovered projection math optimizations.

---

## File Structure

```
src/
├── main.rs                              [Entry point]
│   - App initialization
│   - Plugin setup
│   - System scheduling (Startup -> Update)
│
├── cli.rs                               [Existing, unchanged]
│   - CLI argument parsing
│   - Window size computation
│
├── setup.rs                             [Shared setup utilities]
│   - setup_camera_system
│   - Common initialization helpers
│   - Coordinate system setup
│
├── ecs/
│   ├── resources.rs                     [Global resources]
│   │   - MapSettings { window_width, height, cli }
│   │   - ProjectionCache { kavrayskiy_vii, azimuthal }
│   │   - GeospatialData { coastlines, land, rivers, lakes, elevation }
│   │   - LayoutConfig { outline_regions, map_bounds, text_positions }
│   │
│   ├── components.rs                    [Shared component types]
│   │   - LayerType (enum: Paper, Map, Overlays, Outlines)
│   │   - HasSolidColorBlock (marker)
│   │   - HasStyling (marker)
│   │   - HasFXShaders (marker)
│   │   - IsDynamic (marker - Phase 4)
│
├── layers/
│   ├── map/
│   │   ├── components.rs                [Layer 1 components]
│   │   │   - MapRegion { projection, bounds }
│   │   │   - ProjectionData { projection_type, cached_coords }
│   │   │   - WaterGeometry, LandGeometry, etc.
│   │   │
│   │   ├── systems.rs                   [Layer 1 systems]
│   │   │   - setup_map_system (Deferred - Phase 2)
│   │   │   - (Future) update_map_styling_system
│   │   │   - (Future) apply_elevation_shaders_system
│   │   │
│   │   ├── projections.rs               [Projection math - KEEP EXISTING]
   │   │   - kavrayskiy_vii(lon_deg, lat_deg) -> (f32, f32)
   │   │   - perspective_pole(r, lon, lat, lon0, d) -> ProjectionResult
│   │   │   - Projection trait & implementations
│   │   │
│   │   └── geospatial.rs                [Data loading]
│   │       - load_natural_earth_coastlines()
│   │       - load_land_polygons()
│   │       - load_elevation_data()
│   │       - GeospatialLoader utility
│   │
│   ├── overlays/
│   │   ├── components.rs                [Layer 2 components]
│   │   │   - GraticuleGrid { spacing, color, density }
│   │   │   - TextContent { text, language }
│   │   │   - TextStyle { font_size, orientation, color }
│   │   │   - Diagram { diagram_type }
│   │   │   - DecorativeElement { element_type }
│   │   │
│   │   ├── systems.rs                   [Layer 2 systems]
│   │   │   - setup_overlays_system (Startup - 1st implementation)
│   │   │   - (Future) setup_graticule_system
│   │   │   - (Future) apply_typography_styling
│   │   │   - (Future) animate_overlays_system (Phase 4)
│   │   │
│   │   ├── graticule.rs                 [Graticule system]
│   │   │   - generate_graticule_lines(bounds, projection) -> Vec<Line>
│   │   │   - generate_scale_marks(bounds) -> Vec<Tick>
│   │   │
│   │   └── diagrams.rs                  [Diagram templates]
│   │       - OrbitDiagram definition
│   │       - CelestialSphere definition
│   │
│   └── paper/
│       ├── components.rs                [Layer 3 components]
│       │   - PaperBackground { color }
│       │   - PaperTexture (Phase 3+)
│       │
│       └── systems.rs                   [Layer 3 systems]
│           - setup_paper_system (Startup - 2nd implementation)
│           - (Future) apply_paper_texture_system
│           - (Future) apply_aging_effects_system
│
├── render_bevy.rs                       [Render utilities - KEEP EXISTING]
│   - setup_camera() [Can move to setup.rs]
│   - setup_map() [Refactor to use new layer systems]
│   - Mesh2d/ColorMaterial helpers
│
└── overlay.rs                           [Existing, can refactor or archive]
    - (Legacy from CPU-based rendering; migrate functions)
```

### File Organization Summary

#### By Responsibility Type:
- **Components**: One per layer (`outlines/components.rs`, `map/components.rs`, etc.)
- **Systems**: One per layer (`outlines/systems.rs`, `map/systems.rs`, etc.)
- **Domain Logic**: Separated by concern (projections, geospatial, graticule, layout)
- **Shared**: Central `ecs/` folder for resources and cross-cutting components

#### By Implementation Order:
1. `setup.rs` → Camera
2. `overlays/systems.rs` → Graticule rendering
3. `map/systems.rs` → Projected geometry (deferred)
4. `paper/systems.rs` → Background

#### By Phase (Future):
- **Phase 1**: Startup systems only (all files above)
- **Phase 2**: Add styling systems (each layer's `systems.rs` gains `*_styling_system`)
- **Phase 3**: Add shader systems (each layer's `systems.rs` gains `*_shaders_system`)
- **Phase 4**: Add dynamic/animation systems

---

## Component → File Mapping

| Component | File | Layer |
|:---|:---|:---|
| `MapRegion`, `ProjectionData` | `map/components.rs` | Layer 1 |
| `GraticuleGrid`, `TextContent`, `Diagram` | `overlays/components.rs` | Layer 2 |
| `PaperBackground` | `paper/components.rs` | Layer 3 |
| `LayerType`, markers | `ecs/components.rs` | Shared |

## System → File Mapping

| System | File | Order | Phase |
|:---|:---|:---:|:---|
| `setup_camera_system` | `setup.rs` | 1 | 1 |
| `setup_overlays_system` | `overlays/systems.rs` | 2 | 1 |
| `setup_map_system` | `map/systems.rs` | — | 2 (Deferred) |
| `setup_paper_system` | `paper/systems.rs` | 3 | 1 |
| `apply_styling_*` | `layers/*/systems.rs` | — | 2 |
| `apply_shaders_*` | `layers/*/systems.rs` | — | 3 |
| `animate_*` | `layers/*/systems.rs` | — | 4 |