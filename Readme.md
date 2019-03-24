# Very simple calculator

If you sit in a meeting or need to do some quick calculations but you're not satisfied with using
the built in calculators on most systems (that don't save the inputs so if you make a mistake you 
have to start all over).

Most often the solution is to open a spreadsheet or use a good old fashined calculator, however opening
a spreadsheet can be slow and many times you have 20 of them open alredy, and opening more just 
to do some quick calculations is slow and hard.

You could just create a plain text file and start writing down numbers: 

```
# Number of chairs in rooms
1, 3, 4, 5, 100, 101, 2405, 20, 1

1, 4
10, 0.5

1,5,6,4,3

22, 103, 3

# Employees in departments
66, 40, 48, 10, 2
```

Then you can write `calc --add myfile" and get an output that looks like this:

```
Number of chairs in rooms:
1 + 3 + 4 + 5 + 100 + 101 + 2405 + 20 + 1 = 2640
1 + 4 = 5
10 + 0.5 = 10.5
1 + 5 + 6 + 4 + 3 = 19
22 + 103 + 3 = 128

Employees in departments:
66 + 40 + 48 + 10 + 2 = 166
```