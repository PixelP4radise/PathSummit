# Hill Climbing for the Traveling Salesman Problem (TSP)

This project implements a **Hill Climbing algorithm** to solve the **Traveling Salesman Problem (TSP)** as part of the *Introdução à Inteligência Artificial* coursework.  
The implementation focuses on exploring different neighborhood strategies, handling invalid solutions, and maximizing performance through multi-threading.

---

## 🚀 Features

- **Hill Climbing Search**  
  Iteratively improves the current solution by moving to better neighbors until no improvement is possible.  

- **Neighborhood Generation**  
  - Neighbors are generated based on the **last best solution found**.  
  - Supports multiple neighborhood strategies.  

- **Validity Control**  
  - Algorithm can be run in two modes:  
    - **Allow invalid solutions** (they are explored and penalized).  
    - **Strictly valid mode** (only valid solutions count as iterations).  

- **Concurrency & Performance**  
  - Multi-threaded design using:  
    - **Channels** for communication between threads.  
    - **Mutexes** for shared data synchronization.  
    - **RwLock** for safe concurrent read/write access.  
