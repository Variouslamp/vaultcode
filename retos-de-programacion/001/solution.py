def main():
    texto = input("Ingrese texto: ")
    texto = texto.strip().replace(" ", "")
    texto_reverso = []
    for i in texto:
        texto_reverso.insert(0, i)
    texto_reverso ="".join(texto_reverso)
    if texto == texto_reverso:
        print("Si, el texto es un palindromo")
    else:
        print("No, el texto no es un palindromo")

main()