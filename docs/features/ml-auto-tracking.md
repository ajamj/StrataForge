---
layout: default
title: ML Auto Tracking
parent: Core Features
nav_order: 1
---

# Machine Learning Auto-Tracking

Seisly uses deep learning models to accelerate the horizon interpretation process.

## How it Works

The auto-tracking engine uses a CNN (U-Net architecture) to predict the Z-offset of reflectors in small 3D patches. Starting from a seed point, the engine expands the interpretation by recursively predicting and snapping to the best reflector match.

## Using the Tool

1. Select the **Auto-Track** tool (🔄).
2. Ensure you have an active horizon selected in the Project Explorer.
3. Click a seed point on a seismic section.
4. The engine will propagate the interpretation across the volume based on the confidence threshold.

---

[Back to Core Features]({% link docs/README.md %})
