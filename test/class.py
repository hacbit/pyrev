class Test:
    """
    This is a test class
    - __init__: A constructor method
    - change_name: A method to change the name attribute
    - display: A method to display the class attribute and the instance attribute
    """

    Foo = "This is Foo"

    def __init__(self, name: str = "Default"):
        self.name = name
        print("Test class is initialized")

    def change_name(self, new_name: str):
        self.name = new_name
        print(f"Name is changed to \"{self.name}\"")

    def display(self):
        print(self.Foo)
        print("Name is \"%s\"" % self.name)

class NoDoc:
    def __init__(self):
        print("NoDoc class is initialized")
        pass

t = Test()
t.change_name("This is new name")
t.display()