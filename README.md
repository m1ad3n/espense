### Work in Progress – Expense Tracker

A simple Rust expense-tracking application that will be used for storing and managing expenses through an SQL database. The application provides the following capabilities to a user: editing, deleting, and adding expenses.
Categorization of Expenses: Food, Transportation, Entertainment, etc.
View history of their expenses.

The application will leverage an SQL database (SQLite) for data persistence, meaning all expenses will persist between runs of the application. The command line interface will let users interact with the application by typing commands that will let them manage their expenses.

All you need to do is build and run the project. The application will then ask for commands concerning expenses, and all the data will be persisted in an SQLite database. This project will serve as a good example of how to use Rust with SQL to build something useful around personal finance management.