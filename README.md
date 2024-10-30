# **Flastex** - Flow Fast Execution

**Flastex** is an advanced Business Process Management (BPM) engine built with **Rust**, designed to deliver lightning-fast execution of workflows while maintaining fluidity and precision. Combining the principles of **flow**, **speed**, and **execution**, Flastex empowers businesses to model, orchestrate, and optimize their processes with unmatched efficiency.

Flastex is built with a focus on performance, memory safety, and concurrency, leveraging the power of Rust to manage complex workflows while maintaining speed and reliability. Whether you're automating sophisticated workflows or integrating real-time decision-making, Flastex offers a robust and scalable solution.

Flastex adhere to (OMG specifications)[https://www.omg.org/spec/BPMN/2.0/].

---

## **Why Flastex?**

Flastex stands for **Flow, Fast, Execution**, representing its core strengths:
- **Flow**: Seamlessly manage and orchestrate business processes with dynamic flow modeling.
- **Fast**: Rust ensures lightning-fast performance, ideal for real-time process execution.
- **Execution**: Deliver scalable and reliable process automation with advanced orchestration.

---

## **Why Rust?**
Flastex is built using _Rust_, that offers exceptional strict memory safety. In addition, it is one of the fastest and most memory-efficient modern programming languages. Rust offers performance comparable to C while being significantly faster and more memory-efficient than languages like Java, C#, or Go. This makes Rust the ideal choice for high-throughput applications, where both speed and resource optimization are critical. 

Here are some relevant benchmarks that highlight Rust's performance compared to other languages:

- [Rust vs Java](https://programming-language-benchmarks.vercel.app/rust-vs-java)
- [Rust vs Go](https://programming-language-benchmarks.vercel.app/rust-vs-go)
- [Rust vs C](https://programming-language-benchmarks.vercel.app/rust-vs-c)
- [Rust vs C#](https://programming-language-benchmarks.vercel.app/rust-vs-csharp)
- [Rust vs Swift](https://programming-language-benchmarks.vercel.app/rust-vs-swift)

---

## **Key Features**

- **High Performance**: Flastex leverages Rust's efficient memory management and concurrency model to provide fast and safe execution.
- **Flow Modelling**: Design and automate workflows with built-in support for BPMN standards.
- **Scalable Orchestration**: Scale from simple workflows to complex, enterprise-grade business processes.
- **Real-Time Decision Making**: Support for decision models, integrated into workflows for rapid and dynamic execution.
- **Open-Source Core**: Flastex is built with an open-source philosophy, allowing the community to contribute and extend its functionality.

---

## License

This project is licensed under the [GNU Affero General Public License v3.0](LICENSE.md) (AGPLv3). You are free to use, modify, and distribute the software under the terms of this license.

### Commercial License

If you wish to use this software without the obligations of the AGPLv3 (e.g., for proprietary applications or SaaS without sharing source code), **commercial licenses** are available. For more details or to purchase a commercial license, please contact us at [roberto.trunfio@flastex.tech].

---

## **Roadmap**

### **Milestone 1: Core BPMN Engine**
- Implement BPMN 2.0 XML parsing using `quick-xml`.
- Define data structures for BPMN elements such as tasks, gateways, and events.
- Develop basic sequential task execution.

### **Milestone 2: Database Persistence with Sea-ORM**
- Integrate **Sea-ORM** for persisting process instances, task states, and history.
- Implement database migrations for process data storage.

### **Milestone 3: REST API with Axum**
- Build a REST API using **Axum** to manage processes, including starting, stopping, and querying process states.
- Define API routes for workflow management.

### **Milestone 4: Asynchronous Task Execution**
- Introduce concurrency with asynchronous task execution using Rust’s async/await model.
- Support parallel task execution and asynchronous events.

### **Milestone 5: Advanced BPMN Features**
- Add support for complex BPMN elements like parallel gateways, sub-processes, and event-driven workflows.
- Implement timers, signals, and message events for advanced use cases.

### **Milestone 6: CMMN and DMN Support**
- Extend the core engine to include support for CMMN (Case Management) and DMN (Decision Model Notation).
- Develop integration for dynamic decision-making processes in real-time workflows.

### **Milestone 7: Testing and Optimization**
- Implement comprehensive unit and integration tests for all core components.
- Optimize performance for high-load environments and ensure scalability.

### **Milestone 8: Enterprise Features**
- Develop advanced enterprise features such as multi-tenancy, custom plugins, and advanced reporting.
- Offer enterprise-grade support and services for business-critical deployments.

---

## **Getting Started**

1. Clone the repository:
   ```bash
   git clone git@github.com:Flastex/flastex-bpm.git
   ```
2. Follow the installation instructions for setting up the environment.
3. Start building workflows and contributing to Flastex!

---

## How to Contribute

We appreciate your interest in contributing to Flastex! Please follow the guidelines outlined in our [CONTRIBUTING.md](CONTRIBUTING.md) document for instructions on how to contribute, submit bug reports, or suggest enhancements.