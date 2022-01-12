# PC Test

## The problem:
        There's a need to store data in a three dimensional array, sized roughly 60x60x60. All elements are f32. While populating the array, it is of paramount importance to keep the axes as they are, since otherwise the data would be i) tedious to generate and b) the code would be extremely hard to read. Hence, once populated, we have an array where the order of the axes is x, y, z. Now, the next step in the process requires the data to be represented in a three dimensional array with axes ordered x, z, y and consequently y and z need to be swapped.

1. What data structure(s) would you use?
2. How would you swap the two axes as fast as possible?

In reality the data can naturally be just one continuous chunk of bytes, but the order of the axes needs to be reflected in the physical representation.




## Solution 

- One possible way would be to use an adjacency list i.e Vector of Vectors. By implementing a few methods access and mutation can be made easy and the code will be easy to read and comprehend.
- Also by Maintaining a swap history, the structure can be reverted back to it original form before any swapping.


```
    [
        [], // Axis x
        [], // Axis y
        [], // Axis z
        [], // Axis t
        ...
    ]

```