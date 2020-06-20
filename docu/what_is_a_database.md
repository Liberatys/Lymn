# What is a database

> "In essence a database is nothing more than a collection of information that exists over a long period of time, often more than many years.
> In common parlance, the term database refers to a collection of data that is managed by a DBMS."
>
> --- Database System The Complete Book


So a database is just a collection of information that is stored
and somehow managed by a system that is tasked with providing an
interface between something or someone with the information stored in it / with it.


What is expected by a DBMS is to allow the user / developer to create a structure in which he wants to store information. What can also be called a schema.
Most often this is expected to be done in a language that is designed to create these schemas.	


The system is also expected to be asked / queried for information and content that is stored within the database.


## Data-Definition Langage (DDL)

A Data-Definition Language is a language that enables the creation, alteration and deletion of schemas inside a DBMS.
Most often these three operations are represented by the following keywords:

* creation : CREATE
* alteration : ALTER
* deletion : DROP

> The keywords stated are referring to SQL (Structured Query Language). It being the most used language in databases I will use it as a reference point to illustrate different
> concepts inside DDL.

In most DBMS today this language is used to create a structure that contains a database which itself contains multiple tables that may or may not have relationships between its data.

	CREATE TABLE [table_name] ([column_name: column_type]...);

	DROP TABLE [table_name];

	ALTER TABLE [table_name] ...(alteration operation {ADD, DROP})
