Python Specific notes:
- ArcBall.h's function Matrix3fMulMatrix3f (A, B) uses an unusual 
convention where A' = B * A. The A matrix is changed in place. 
The python function creates a new matrix and operates as A' = A * B.
- ArcBall's math and data structures have largely been implemented
using the Numerical python package. Numerical provides fast, high 
quality, matrix operations and it allows for code that expresses 
the math more succinctly.
- Initialize () doesn't need to generate texture coordinates for 
the sphere quadric because we don't apply an texture maps in this tutorial.
- Python's modulus operator is defined to act differently for negative numbers

