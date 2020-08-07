# Lymn

A research project that aims to bring understanding to the internal workings of a RDMS by writing 
a version based on the findings of different literature and proven principles ranging from sqlite to mariadb

Because this is a side project, I'll name all parts of the system in a funny way, because I can.

Parts contained in the system:

- [ ] SQL-Parser {Ocarina}
- [ ] SQL-Sanatizer
- [ ] Buffer-Manager

## Example of usage

	INSERT INTO [table_name] (optional)(column_names) VALUES(value)

	SELECT (columns | *) FROM [table_name] WHERE [col_name] = [col_value]
		
	CREATE TABLE [table_name]([col_name] [col_type],...)


## Display of data

In the current testing loop the return of values in a select query is rendered in a table format

	SELECT * FROM tester

	+-----+--------+
	| col | column |
	+-----+--------+
	| 7   |        |
	+-----+--------+
	| 8   | 9      |
	+-----+--------+

This formating is done with prettytable-rs. -> 
[Prettytable](https://github.com/phsym/prettytable-rs)

## Building

	cargo build

## Testing

Currently only the unit-tests are run on "cargo test".
Integration tests are not executed because the file creation is not set up in a way that would
facilitate the testing.

**If you'd like to test the integration tests you may do so by using the inMemoryTable implementation**

# Disclaimers

Currently only the functionality in the example part is working.

At the moment there is not much validation of input. Therefore a column that doesn't exist will 
panic the system. -> TAKE CARE!!

## Example
CREATE TABLE User(username string, password string);

INSERT INTO User VALUES('Your','Password');

SELECT * FROM User;


## Required
* tesseract
* imagemagick

## Video of PDF storage with OCR
https://youtu.be/OLkc3ieJFYs
