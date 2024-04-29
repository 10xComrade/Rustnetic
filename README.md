<p align="center">
  <img src="https://github.com/KohoolateSen/Rustnetic/assets/98024986/35b788a4-48d9-4d8a-bbe3-b01508d39128" width="400em" height="400em"/>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/_Rustnetic-ff5733?logo=github" alt=" Rustnetic">
  <img src="https://img.shields.io/badge/License-GPL V 3.0-ea3c00?logo=gnu" alt="License - GPL V 3.0"/>
  <img src="https://img.shields.io/badge/Language-Rust-e2722e?logo=rust&logoColor=e2722e" alt="Language - Rust">
  <img src="https://img.shields.io/badge/Plotters V 0.3.5-navy?logo=github" alt="Plotters V 0.3.5"/>
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
6. Iterate over steps **2-5** until iteration number reaches **0**.

After processing iterations repeatedly, we notice that the output results become more rational and approach our desired answer more closely.

**an Example of results getting enhanced after each iteration :**

<img align="center" width="70%" height="70%" src="https://github.com/10xComrade/Rustnetic/assets/165196971/1df1788d-a968-41c0-a36f-fbbb2d82b734">

(x = iteration , y = avg f_obj of genes per chromosome)

Note: The average objective function (f_obj) must get closer to zero. Because if the chromosome's genes are going to solve the problem, the f_obj of each chromosome will be zero, thus decreasing the average f_obj of all chromosomes in each iteration.

# Setup

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
- [x] Check fitness values if they're correct ...
- [x] support different variants of equations  
- [x] Show best scores , success rates within each iteration
- [x] Add GUI ( not neccesssary )
