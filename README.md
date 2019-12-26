# Nit

Nit is a path tracing renderer built with Rust

<p align="center">
<img width="650" height="650" src="https://i.imgur.com/lXpQpBU.png"/>
</p>

<p align="center">
<img width="650" height="650" src="https://i.imgur.com/SLJWq7z.png"/>
</p>

## Project Roadmap

Tentative plan for the program

### Object Primitives
- [x] Spheres
- [x] Rectangles
- [x] Triangles
- [ ] Generic Polygons

### Materials
- [x] Lambertian
- [x] Metal
- [x] Glass
- [ ] Glossy
- [x] Lights

### Tone Mapping
- [x] Clamp
- [x] Reinhard-Jodie
- [ ] Filmic
- [ ] Camera Response Functions

### Performance
- [x] Parallel Rendering (CPU)
- [ ] Parallel Rendering (GPU)
- [ ] Multi-machine distributed rendering
- [ ] MLT
- [ ] Importance Sampling
- [x] BVH
- [ ] k-d tree

### General Features
- [x] STL File loading
- [ ] Photon mapping
- [ ] Spectral rays
- [ ] Adaptive sampling
- [ ] Russian Roulette loop termination
