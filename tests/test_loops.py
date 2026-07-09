from myvm_lib import entry_point, print_str

@entry_point
def main():
    print_str("For Loop:")
    for i in range(3):
        if i == 1:
            continue
        print_str(str(i))
        
    print_str("While Loop:")
    j = 5
    while j > 0:
        if j == 2:
            break
        print_str(str(j))
        j = j - 1
        
    print_str("Fim")
