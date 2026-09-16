# Cluster specs (compiled from public pages, 2026-09-16)

## Impulse (2017, Clement Hall 226)

- 42 CPU nodes: 2x Intel Xeon E5-2680v4 (28 cores/node @ 2.4 GHz), 64–896 GB RAM
- 2 GPU nodes: 28 cores + 384 GB RAM + 2x NVIDIA K80 (exposed as 4 devices)
- Interconnect: 56 Gb non-blocking InfiniBand
- Shared 175–300 TB NFS storage with Warp 1

## Warp 1 (2022/2023, NSF 2127188)

- 10 GPU nodes: 2x AMD Epyc 7713 (128 cores/node @ 2.0 GHz), 512 GB RAM
- GPUs: 2x NVIDIA A100 40 GB per node (newer docs; older docs say "4 GPU devices")
- Interconnect: 100 Gb non-blocking InfiniBand

## GENESIS (2025–2026, ASCEND Center — Advanced Scalable Computing, Extreme Networks & Data)

- Tennessee Tech's largest scientific instrument; flagship AI/HPC system
- 27 server nodes, 108 AMD Instinct MI300A APUs, >13 PFLOPS
- Unified-memory architecture for AI/sim workloads
- Workloads: engineering sim, large-scale ML, cybersecurity/AI threat analysis,
  environmental modeling, smart manufacturing, analytics/visualization, materials
- Director: Anthony Skjellum; enables federal grants, doctoral recruiting, lab/industry ties
- More: https://www.tntech.edu/ascend

## Access model

- Consultation required, then ITS/RCD provisions account
- Account request + user docs behind TN Tech login (SSO)
- Admin: Dr. Mike Renfro (ITS/RCD director, ex-NSF XSEDE champion, ACCESS facilitator)
- Facilitators: Sharon Colson, Jim Moroney (ex-ORNL Summit/Frontier), Trevor Clark
- Support: queueing help, containers/Spack/envs, Open OnDemand + Jupyter via OOD
