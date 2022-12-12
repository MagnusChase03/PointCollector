# Rust Reinforcment Learning

## Overview

- Simple Maze Grid
- Move the goal after collection
- Train off of the path after collecting the goal
- Nerual Network for Agent
- Memory Replay
- Train off of the memory which contains (s, a, s, r)
- Negative reward in each state to incentivize faster paths, positive points when reaching goal
- Max amount of actions within a trajectory
- Beable to train in batches for faster improvment

## Neural Network

- x, y cordinate for the goal and player
- 1 hidden layer with 6 nodes
- 4 outputs representing probability of each action
- Train it based on its path and accosiated gains, more likly to do good rewards given a certain state

## File Structure

- Nerual Network Struct
- Map Enum
- Memory Enum