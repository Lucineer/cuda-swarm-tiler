# cuda-swarm-tiler

GPU-accelerated Monte Carlo yield simulation for MoE (Mixture of Experts) swarm chips.

## What It Does
- Simulates wafer fabrication defects using Poisson distribution
- Grades dies: GOLD (center, 0 defects) / SILVER / BRONZE / SCRAP
- Routes MoE experts to best-available dies
- Estimates unit cost given yield

## CUDA Acceleration
Each Monte Carlo trial runs independently → perfect for GPU parallelism.
Expected speedup: 100-1000x over CPU for 10,000+ trials.

## Integration
See cuda-intelligence/tiler.rs for the CPU reference implementation.