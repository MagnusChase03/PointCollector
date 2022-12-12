# Rust Reinforcment Learning

## Overview

- Simple Maze Grid
- Checkpoints in the maze towards goal with rewards
- Nerual Network for Agent
- Memory Replay
- Train off of the memory which contains (s, a, s, r)
- Negative points if it runs into a wall, positive points when reaching checkpoint or alot for goal
- Max amount of actions within a trajectory
- Beable to train in batches for faster improvment

## Neural Network

- 12x12 grid, so 144 * 4 = 576 inputs using one hot encoding
- 1 hidden layer with 50 nodes
- 4 outputs representing probability of each action

## File Structure

- Nerual Network Struct
- Map Enum
- Memory Enum