# Top 10 Performance Hotspots (ROI List)

Purpose
- Track the highest ROI performance improvements.
- Keep a short, prioritized list for execution.

Status
- Initial hypothesis list based on known hotspots; validate after baseline metrics.

Scoring rubric
- Impact (1-5): user-visible improvement.
- Effort (1-5): change complexity.
- Risk (1-5): regression likelihood.
- ROI = Impact / Effort, adjust down for high risk.

Top 10 list

| Rank | Surface | Hotspot | Impact | Effort | Risk | ROI | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | ARCH | Full canvas redraw on pointer move | 4 | 2 | 2 | 2.0 | Draw on state change; cache static graph layers |
| 2 | LEARN | Demos run offscreen / heavy math on mobile | 4 | 2 | 2 | 2.0 | Gate demo updates; pause when hidden |
| 3 | WELCOME | Per-frame allocations + RNG + color math | 5 | 3 | 2 | 1.7 | Reuse buffers; cache color conversions |
| 4 | BLOG | Runtime markdown parse + large post render | 4 | 3 | 2 | 1.3 | Precompute or cache parsed output |
| 5 | HELIOS | Star list rebuild + overlay text redraw | 4 | 3 | 2 | 1.3 | Cache star list; throttle labels |
| 6 | TOOLS/PLL | Plot redraw per input; CPU-heavy math | 3 | 2 | 2 | 1.5 | Throttle redraw; cache computed series |
| 7 | ATLAS | Path tessellation + full redraw on pan/zoom | 4 | 4 | 3 | 1.0 | Tile caching + LOD |
| 8 | MCAD | Mesh allocations + GPU upload churn | 5 | 4 | 3 | 1.2 | Batch uploads; cache tessellation |
| 9 | SIMULATION/CHLADNI | Shader complexity + particle count | 4 | 4 | 3 | 1.0 | Decouple sim step; reduce particles on mobile |
| 10 | Platform-wide | Large asset payloads (fonts/images/JSON) | 3 | 2 | 1 | 1.5 | Compression + font/image optimization |
