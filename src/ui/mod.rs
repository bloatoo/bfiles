pub fn help_message() -> String {
    String::from(
        "
Common

d:
  Delete a file

r:
  Rename a file

c:
  Create a file


Movement

Left arrow key:
  Go back a directory

Right arrow key:
  Go forward a directory

Up arrow key:
  Go up an element

Down arrow key:
  Go down an element

Enter:
  Go forward a directory
        ",
    )
}
