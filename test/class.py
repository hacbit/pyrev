class Test:
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

t = Test()
t.change_name("This is new name")
t.display()