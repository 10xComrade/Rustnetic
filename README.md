<p align="center">
  <img src="https://github.com/KohoolateSen/Rustnetic/assets/98024986/35b788a4-48d9-4d8a-bbe3-b01508d39128" width="400em" height="400em"/>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/_Rustnetic-ff5733?logo=github" alt=" Rustnetic">
  <img src="https://img.shields.io/badge/License-GPL_V_3.0-ea3c00?logo=gnu" alt="License - GPL V 3.0"/>
  <img src="https://img.shields.io/badge/Language-Rust-e2722e?logo=rust&logoColor=e2722e" alt="Language - Rust">
</p>

# Rustnetic

A demonstration of Darwin's Theory of Evolution through a genetic algorithm, illustrating how it enhances the value of genes within chromosomes for subsequent generations, resulting in improved outcomes.

In this demo, we'll see how a genetic algorithm is able to solve an equation.

**Assuming that we have:**
**a + 2b + 3c + 4d = 30**

The purpose is to solve this equation and observe the potential answers we could obtain, thereby ensuring that our algorithm generates increasingly reliable results with each iteration.

**There are multiple crucial steps built in this algorithm:**

1. Population
2. Evaluation
3. Selection
4. Crossover
5. Mutation
6. Iterate over steps **2-6** until iteration reaches **0**

After processing iterations repeatedly, we notice that the output results become more rational and approach our desired answer more closely.

# Installation

1. Make sure you have Rust installed on your PC
2. Clone the project:
```
git clone https://github.com/KohoolateSen/Rustnetic.git
```
3. Change Directory :
```
cd Rustnetic
```
4. Run main.rs using cargo :
```
cargo run
```
# TODOs
- [ ] Code optimizations
- [ٓX] Check fitness values if they're correct ...
- [ ] Make it more flexible that accepts also different variants of equations ( maybe using macros )
- [ ] Show best scores , success rates within each iteration
- [ ] Add GUI ( not neccesssary )
