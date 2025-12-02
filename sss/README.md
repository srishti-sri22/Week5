## Ideology

- Step 1 : take the secret , the number and the desired k shares from the user using clap via cli parsing. A stuct to store the points - x and y coordinate. main - to input the S, N and K as an CLI input , we'll be using parser

 - Step 2 : lets consider that the secret.
Now let's take the given string and convert it into bytes and then decimal and vice versa to be able to do anything i mean any claculation on it.

 - Step 3 : Now lets focus on making the function to make the polynomial. What what it will have is - a struct Polynomial , to store all the random generated coefficients for the computations.
 Next in the same, make a function to make the put the values of the coefficeints and then calculate the shares by putting the values of x for all the k number of shares to be able to put the values, and return them and print them.


