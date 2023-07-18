# Distributed Key-Value Store

This repository contains the code for a distributed key-value store inspired by the principles of Redis, with enhanced capabilities for data replication and partition tolerance.

## Features

- **Consistent Hashing**: Ensures a balanced distribution of keys across nodes, while minimizing data relocation during scale up and scale down.
- **Data Replication**: Increases data availability and durability by storing each key-value pair on multiple nodes.
- **Quorum Reads/Writes**: Ensures data consistency across replicas and reduces the chances of stale reads or write conflicts.
- **Partition Tolerance**: Gracefully handles network partitions and ensures the service continues even when some nodes are unreachable.
- **Gossip Protocol**: Allows nodes to communicate state and changes among each other efficiently and with reduced network load.
- **Leader Election**: Handles node failures and ensures a new leader is elected in case of the failure of a current leader node.
- **Hinted Handoff**: A mechanism to handle temporary node failures during write operations, increasing the system's resilience.
- **Tunable Consistency Model**: Allows to choose between eventual and strong consistency models, enabling trade-offs between consistency, performance, and availability.
- **Write-ahead Log**: Provides a mechanism for data persistence and recovery from failures.

## Getting Started

To run this project, clone the repository and follow the instructions in the BUILDING.md file.

## Contributing

Please read CONTRIBUTING.md for details on our code of conduct, and the process for submitting pull requests to us.

## Roadmap

See the open issues for a list of proposed features (and known issues).

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT) - see the LICENSE.md file for details
