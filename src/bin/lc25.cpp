#include <iostream>
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

void dbg_print(ListNode* p);
ListNode* reverse(ListNode* head, int n) {
  using std::printf;
  auto dummy_head = new ListNode(0, head);
  auto p = head;
  int i = n - 1;
  while (i--) {
    p = p->next;
  }
  auto tail = p;
  auto next = p->next;
  i = n;
  while (dummy_head->next != tail) {
    auto tmp = dummy_head->next;
    dummy_head->next = tmp->next;
    tmp->next = tail->next;
    tail->next = tmp;
    /* code */
  }

  return dummy_head->next;
}

ListNode* reverseKGroup(ListNode* head, int k) {
  if (k == 1) return head;
  auto dummy_head = new ListNode(0, head);
  auto p = dummy_head;
  while (p != nullptr && p->next != nullptr) {
    int n = k;
    auto tail = p;
    auto next = p->next;
    while (n--) {
      if (tail->next == nullptr)
        return dummy_head->next;
      else
        tail = tail->next;
      /* code */
    }
    p->next = reverse(p->next, k);
    p = next;
  }

  return dummy_head->next;
}

int main() {
  auto p1 = new ListNode(5, nullptr);
  auto p2 = new ListNode(4, p1);
  auto p3 = new ListNode(3, p2);
  auto p4 = new ListNode(2, p3);
  auto p5 = new ListNode(1, p4);

  //   dbg_print(p5);
  auto p = reverseKGroup(p5, 3);
  dbg_print(p);
}
void dbg_print(ListNode* p) {
  using std::cout;
  using std::endl;

  while (p != nullptr) {
    cout << p->val << " ";
    p = p->next;
  }
  cout << endl;
}