# Notes

Variables
duration = 7ms
button_pressed = 2ms

duration_speed = duration-button_pressed
speed = button_pressed;

// Example
race_duration = 7
button_pressed = 2

duration_speed = 7-2;
speed = 2;

result = (7-2) * 2;
result = 5;

wait we dont care about the max, we need to find all the points along that equation that are above a certain value.

For this we need to find the two points along that curve that are equal to that value, eg:


value = (duration-button_pressed) * button_pressed;
value = duration*button_pressed - button_pressed^2;
duration\*button_pressed - button_pressed^2 - value = 0;

code for the quadratic equation:

race_duration = 7;
max_last_win = 9;
button_pressed = x;

value = (race_duration-x) * x;
value = race_duration\*x - x^2; // + x^2 - race_duration\*x;
x^2 - race_duration\*x + value = 0;

// fill in the numbers
x^2 -7x + 9 = 0;

too solve for the two points we use the quadratic equation:

x1 = (7+sqrt(7^2-(4\*9))) / 2
x2 = (7-sqrt(7^2-(4\*9))) / 2
