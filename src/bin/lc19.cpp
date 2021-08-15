struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

int main() { return 0; }

// double pointer
ListNode* removeNthFromEnd(ListNode* head, int n) {
  auto dummy_head = ListNode(0, head);
  auto fast = &dummy_head;
  auto slow = &dummy_head;
  bool is_end = false;
  int i = n - 1;
  while (i--) {
    fast = fast->next;
  }
  while (fast->next != nullptr) {
    slow = slow->next;
    fast = fast->next;
    /* code */
  }
  auto tmp = slow->next;
  slow->next = tmp->next;
  tmp->next = nullptr;
  return dummy_head.next;
}

// traverse
// ListNode* removeNthFromEnd(ListNode* head, int n) {
//   int len = 0;
//   ListNode nhead(0, head);
//   auto p = head;
//   while (p) {
//     len++;
//     p = p->next;
//     /* code */
//   }
//   if (len == 1 && len - n == 0) {
//     return nullptr;
//   }
//   int c = 0;
//   p = &nhead;
//   while (c < len - n - 1) {
//     p = p->next;
//     c++;
//   }
//   auto tmp = p->next;
//   p->next = tmp->next;
//   tmp->next = nullptr;
//   return nhead.next;
// }