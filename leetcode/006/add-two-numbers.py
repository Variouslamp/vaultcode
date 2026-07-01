# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def recolector(self, nodo):
        lista_comp = []
        if nodo.next:
            lista_comp = self.recolector(nodo.next)
            lista_comp.append(str(nodo.val))
            return lista_comp
        lista_comp.append(str(nodo.val))
        return lista_comp

    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        operaciones = [l1, l2]
        valores = []
        salida = []
        lista_enlazada = []
        for operacion in operaciones:
            lista = self.recolector(operacion)
            numeros = "".join(lista)
            valores.append(int(numeros))
        output = sum(valores)
        for numero in str(output):
            salida.append(int(numero))
        adelante = None
        for i in salida:
            if adelante:
                objeto = ListNode(i, adelante)
            else:
                objeto = ListNode(i)
            adelante = objeto
            lista_enlazada.insert(0, objeto)
        return lista_enlazada[0]


l1 = ListNode(2, ListNode(4, ListNode(3)))
l2 = ListNode(5, ListNode(6, ListNode(4)))
hola = Solution()
lista = hola.addTwoNumbers(l1, l2)
for i in lista:
    print(i.val, i.next)
