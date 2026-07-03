# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def addTwoNumbers(self, l1, l2) -> ListNode:
        dummy = ListNode()
        new_node = dummy
        carry = 0
        while l1 is not None or l2 is not None or carry == 1:
            sum = carry

            if l1 is not None:
                sum += l1.val
                l1 = l1.next

            if l2 is not None:
                sum += l2.val
                l2 = l2.next

            carry = sum//10
            sum = sum % 10
            new_node.next = ListNode(sum)
            new_node = new_node.next
        return dummy.next


l1 = ListNode(9)
l2 = ListNode(1)
hola = Solution()
lista = (hola.addTwoNumbers(l1, l2))

while True:
    print(lista.val)
    if lista.next is None:
        break
    lista = lista.next
