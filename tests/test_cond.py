from myvm_lib import entry_point, print_str

@entry_point
def main():
    a = 10
    b = 20
    
    if a < b:
        print_str("a eh menor")
    else:
        print_str("a nao eh menor")
        
    if a == 10:
        print_str("a eh 10")
        
    if a != b:
        print_str("a eh diferente de b")
        
    # Truthiness
    x = 1
    if x:
        print_str("x eh truthy")
        
    y = 0
    if y:
        print_str("y eh truthy (ERRADO)")
    else:
        print_str("y eh falsy")
