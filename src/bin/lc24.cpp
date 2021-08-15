struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* swapPairs(ListNode* head) {
  auto dummy_head = new ListNode(0, head);
  if (head == nullptr) return nullptr;
  if (head->next == nullptr) return head;
  auto p = dummy_head;
  do {
    /* code */
    auto p1 = p->next;
    auto p2 = p1->next;
    auto tmp1 = p2->next;
    // auto tmp2 = p2;
    p1->next = tmp1;
    p2->next = p1;
    p->next = p2;

    p = p->next->next;
  } while (p->next != nullptr && p->next->next != nullptr);

  return dummy_head->next;
}