
//  Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    // recursive version
    ListNode *mergeTwoLists(ListNode *l1, ListNode *l2) {
        if (l1 == nullptr)
            return l2;
        if (l2 == nullptr)
            return l1;
        if (l1->val <= l2->val) {
            l1->next = mergeTwoLists(l1->next, l2);
            return l1;
        } else {
            l2->next = mergeTwoLists(l1, l2->next);
            return l2;
        }
    }
    // ListNode *mergeTwoLists(ListNode *l1, ListNode *l2) {
    //     if (l1 == nullptr)
    //         return l2;
    //     if (l2 == nullptr)
    //         return l1;
    //     ListNode *head = nullptr, *another = nullptr;
    //     if (l1->val < l2->val) {
    //         head = l1;
    //         another = l2;
    //     } else {
    //         head = l2;
    //         another = l1;
    //     }
    //     auto p = head;
    //     while (another != nullptr) {
    //         if (p->next == nullptr) {
    //             p->next = another;
    //             break;
    //         }
    //         if (p->next->val > another->val) {
    //             auto tmp = another->next;
    //             another->next = p->next;
    //             p->next = another;
    //             another = tmp;
    //         }
    //         /* code */
    //         p = p->next;
    //     }
    //     return head;
    // }
};