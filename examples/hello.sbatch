#!/bin/bash
#SBATCH --job-name=tntech-hello
#SBATCH --partition=general
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=2
#SBATCH --mem=4G
#SBATCH --time=00:10:00
#SBATCH --output=%x-%j.out

echo "Hello from TN Tech HPC ($SLURM_JOB_ID on $SLURMD_NODENAME)"
srun hostname
