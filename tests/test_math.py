from myvm_lib import entry_point, print_str

@entry_point
def main():
    a = 10
    b = 3
    
    soma = a + b
    sub = a - b
    mul = a * b
    div = a / b
    mod = a % b
    pow = a ** 2
    xor = a ^ b
    
    print_str("Soma:")
    print_str(str(soma))
    print_str("Sub:")
    print_str(str(sub))
    print_str("Mul:")
    print_str(str(mul))
    print_str("Div:")
    print_str(str(div))
    print_str("Mod:")
    print_str(str(mod))
    print_str("Pow:")
    print_str(str(pow))
    print_str("Xor:")
    print_str(str(xor))
