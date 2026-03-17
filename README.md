# Orbis Antiquus (Ancient Maps)

A project to reconstruct historical world maps using modern geospatial data while preserving their original artistic style, cultural context, and visual storytelling.

> Initial focus: **Kunyu Wanguo Quantu (坤輿萬國全圖, 1602)**  
> Future targets include early modern European maps such as 17th-century Amsterdam world maps.

---

## Goals

- Recreate historical maps with:
  - Accurate modern geography
  - Authentic historical aesthetics
  - Period-correct annotations and layout
- Build a reusable pipeline for multiple map styles
- Balance **data accuracy** with **artistic distortion**

---

## 1. Data Sources

We will standardize input data across all map types.

### Core Geography

- **Natural Earth**
  - Coastlines
  - Land polygons
  - Rivers (optional)
  - Lakes (optional)

### Elevation / Terrain

- **SRTM (Shuttle Radar Topography Mission)**
- **GMTED2010** (optional alternative)
- [tangrams/heightmapper](https://github.com/tangrams/heightmapper) (for quick prototyping / visualization)

### Optional Enhancements

- OpenStreetMap (for detailed features if needed)
- Historical datasets (if available, for authenticity)

---

## 2. Workflow / Pipeline

### Overview

```txt
Load Data
↓
Projection
↓
Intentional Distortion
↓
Rendering Engine (Stylization)
↓
Text Reconstruction
↓
Layout System
↓
FX / Post-processing
```

---

### Example: Kunyu Wanguo Quantu (坤輿萬國全圖)

#### 1. Loading Data

- Import GeoJSON:
  - land
  - coastlines
  - optional elevation

---

#### 2. Projection

- Use **Winkel Tripel projection**
- Optionally experiment with oval projections (Eckert IV-like)

---

#### 3. Intentional Distortion

- Adjust geometry to mimic historical worldview:
  - Emphasize Eurasia
  - Compress oceans
  - Slight asymmetry

---

#### 4. Rendering Engine (Stylization)

- Convert geometry into stylized forms:
  - Hand-drawn coastlines
  - Thick borders
  - Simplified shapes

- Mountain system:
  - Extract elevation zones
  - Convert into symbolic mountain chains

- Rivers (optional):
  - Minimal, stylized curves

---

#### 5. Text Reconstruction (Critical)

- Use Classical Chinese annotations:
  - Region names
  - Descriptive paragraphs
- Vertical text layout
- Placement rules:
  - Oceans → large text blocks
  - Land → smaller annotations

---

#### 6. Layout System

Instead of strict panel splitting, define logical regions:

- Map frame (oval boundary)
- Title block
- Notes / annotations
- Special diagrams:
  - 天地圓 (Heaven–Earth diagram)
  - North/South pole representations
- Margins and decorative borders

---

#### 7. FX / Post-processing

- Paper texture (aged parchment)
- Ink irregularities
- Cloud overlays (light, subtle)
- Optional:
  - Day/night tinting
  - Vignette shading
  - Ink bleed simulation

---

## 🏗️ 3. Architecture

### Core Language: Rust 🦀

Used for:

- Projection math
- Geometry processing
- Rendering engine
- SVG/PNG generation

---

### Supporting Tools

#### Python (optional)

Used for:

- Data preprocessing
- GIS operations (GeoPandas, Shapely, PyProj)
- Exporting clean GeoJSON

---

#### CLI / Task Runner

- Linux utilities
- `go-task` (task automation)

---

### Rendering Strategy

- Prefer **CPU-based rendering**
- Avoid heavy GPU usage
- Use GPU only for:
  - Simple texture effects
  - Lightweight visual enhancements

---

## 🎨 4. Design Principles

### Historical Authenticity > Mathematical Perfection

- Accept distortion if it improves authenticity

### Symbolic Representation

- Mountains, rivers, and regions are **visual language**, not exact data

### Layered Composition

- Geometry
- Symbols
- Text
- Decorations

---

## 🔮 Future Ideas / Extensions

- Multiple projection support
- Style presets (Chinese, European, Islamic maps)
- Procedural calligraphy simulation
- Interactive viewer (zoomable SVG)
- Weather effects in poles, deserts, oceans, ...

---

## 🚀 Roadmap

### Phase 1

- Load GeoJSON
- Implement projection
- Basic SVG rendering

### Phase 2

- Stylized coastlines
- Mountain symbol system
- Text placement

### Phase 3

- Layout system
- Decorative elements
- FX pipeline

### Phase 4

- Multi-map support
- Style abstraction

---

## 📌 Notes

This project is not just about maps — it is about reconstructing:

> **how people in the past understood the world**
