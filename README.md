# Motivation

We all struggle with suitable locations to meet. We've all had to cheekily suggest a place more convenient to us or reluctantly accept a meeting point that we deem much more convenient for the other person. The problem becomes more difficult as the group gets bigger ... how do you decide on a place that takes more or less the same effort for 3 people coming from different starting points to get to? What about 5? What if one uses a bike and the other prefers public transport? What if we want a specific venue? Our choice set for cafes is larger than that for cinemas

# Overview

The literature can be grouped by:

- **Defining a midpoint**: There are two main approaches - max and sum
- **Queries**: What is our problem? Do we want to find an exact location in space? Or do we want to find the most suitable point from a set of locations? The latter is an easier problem
- **Solution space**: Euclidian space vs the road network

# Related Problems

- **Optimal Meeting Point (OMP)**
- **Group Nearest Neighbor (GNN):** Given two sets of points P and Q, a group nearest neighbor (GNN) query retrieves the point(s) of P with the smallest sum of distances to all the points in Q. GNN queries can be applied, for instance, when n users at locations Q = {q1,q2,...,qn} want to choose a restaurant to have dinner together, among a set of restaurants at locations P = {p1,p2,...,pm} in the city.
    - I think this is the one we want.

# What is a midpoint?

1. min-sum: minimise the sum of total distances travelled by all people
2. min-max: minimise the maximum distance travelled by all people
3. weighted distance: these are used when the individuals have different weights (e.g. is a couple weighted as 1 or as 2)

# Solutions

## Solving in Euclidean Space

- “When Euclidean distance is used, the problem is known as the **Weber problem, and the OMP is called the geometric median of the query point set Q”**
- [An Extension of the Generalized Weber Problem](https://onlinelibrary.wiley.com/doi/10.1111/j.1467-9787.1968.tb01323.x)
- "However, it has been shown that no closed form formula exists for the Weber problem and its generalizations, and *these problems are usually solved by gradient descent methods, with initial point chosen as the center of gravity of the query point set Q” - ([Efficient Algorithms for Finding Optimal Meeting Point on Road Networks](https://dl.acm.org/doi/abs/10.14778/3402707.3402734))*

## Solving on the Road Network

[Processing Proximity Relations in Road Networks](https://dl.acm.org/doi/abs/10.1145/1807167.1807196)

[Flexible Aggregate Nearest Neighbor Queries in
Road Networks](https://ieeexplore.ieee.org/abstract/document/8509295) (Group Nearest Neighbour / Aggregate Nearest Neighbour)

## Public transport / multimodal

- I din’t find any papers on meeting points using different modes of transport
- For GNN queries, this could be done if we have travel time matrices by mode

## Efficient algorithms

### **Pruning**

**[Efficient processing of optimal meeting point queries in Euclidean space and road networks](https://link.springer.com/article/10.1007/s10115-013-0686-y)**

- “In the setting of Euclidean space, we propose a general framework for  answering all OMP query variants and also identify the best algorithms  for particular types of OMP queries in the literature. In the setting of road networks, we study how to access only part of the road network and examine part of the candidates. Specifically, we explore two pruning  techniques, namely *Euclidean distance bound* and *threshold algorithm*, which help improve the efficiency of OMP query processing.”

**[On Efficient Aggregate Nearest Neighbor Query Processing in Road Networks](https://link.springer.com/article/10.1007/s11390-015-1560-z)**

### **Approximate and adaptive query processing**

**[Advanced algorithms for optimal meeting points in road networks](https://ietresearch.onlinelibrary.wiley.com/doi/full/10.1049/itr2.12323)**

# Other uses

- ride-hailing: **[Optimal Multi-Meeting-Point Route Search](https://ieeexplore.ieee.org/abstract/document/7300432)**
- DRT: **[Real-world meeting points for shared demand-responsive transportation systems](https://link.springer.com/article/10.1007/s12469-019-00207-y)**
- facility (re)location
    - **[Toward Balancing the Efficiency and Effectiveness in k-Facility Relocation Problem](https://dl.acm.org/doi/abs/10.1145/3587039)**
