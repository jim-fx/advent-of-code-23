My idea is to map each card do an u8 array:

Eg: AAQJ7 -> 12, 12, 10, 9, 7

A = 12
K = 11
Q = 10
J = 9
T = 8
9 = 7
8 = 6 
7 = 5
6 = 4
5 = 3
4 = 2
3 = 1
2 = 0

No i want to encode the value of the deck in a single number, first we need to check if it is a pattern:

Five of a kind  = 6
Four of a kind  = 5
Full House      = 4
Three of a kind = 3
Two Pair        = 2
One Pair        = 1
High Card       = 0

And then we go through the cards and assign each a single value, so: 

AAQJ7 = ( 2 ) 12 12 10 7;

and then we need to multiply each value by some high number and add them so that the resulting number is something like this:

1.000.000.000.000.000 * KIND +
1.000.000.000.000     * FIRST_CHAR +
1.000.000.000         * SECOND_CHAR +
1.000.000             * THIRD_CHAR +
1.000                 * FORTH_CHAR +
1                     * FIFTH_CHAR

02.012.012.010.009.007;

I would need to use the u64 to store that number
