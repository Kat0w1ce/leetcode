#include <iostream>
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* rotateRight(ListNode* head, int k) {
  if (head == nullptr) {
    return nullptr;
  }
  auto p = head;
  int len = 1;
  while (p->next != nullptr) {
    p = p->next;
    len++;
    /* code */
  }
  auto step = k % len - 1;
  p->next = head;
  p = head;
  //   int n = k - ;
  while (step > 0) {
    step--;
    p = p->next;
  }
  auto rst = p->next;
  p->next = nullptr;
  return rst;
}

void dbg_print(ListNode* p) {
  using namespace std;
  while (p != nullptr) {
    /* code */
  }
}